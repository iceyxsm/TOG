// Type checker for TOG
//
// Performs type checking and inference to enable better optimizations.
// Uses gradual typing - types are optional but help with optimization.

use crate::ast::*;
use crate::error::TogError;
use std::collections::HashMap;

pub struct TypeChecker {
    environment: HashMap<String, Type>,
    struct_defs: HashMap<String, (Vec<(String, Option<Type>)>, Vec<MethodDecl>)>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
            struct_defs: HashMap::new(),
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> Result<(), TogError> {
        for stmt in &program.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }
    
    fn check_statement(&mut self, stmt: &Stmt) -> Result<(), TogError> {
        match stmt {
            Stmt::Let { name, type_annotation, value } => {
                let value_type = self.infer_expression_type(value)?;
                
                if let Some(annotated_type) = type_annotation {
                    // Check type compatibility
                    if !types_compatible(&value_type, annotated_type) {
                        return Err(TogError::TypeError(
                            format!("Type mismatch: expected {:?}, got {:?}", annotated_type, value_type),
                            None
                        ));
                    }
                    self.environment.insert(name.clone(), annotated_type.clone());
                } else {
                    // Type inference
                    self.environment.insert(name.clone(), value_type);
                }
            }
            Stmt::Assign { name, value } => {
                // Check if variable exists
                if !self.environment.contains_key(name) {
                    return Err(TogError::TypeError(
                        format!("Cannot assign to undefined variable: {}", name),
                        None
                    ));
                }
                let value_type = self.infer_expression_type(value)?;
                let var_type = self.environment.get(name).unwrap();
                
                // Check type compatibility
                if !types_compatible(&value_type, var_type) {
                    return Err(TogError::TypeError(
                        format!("Type mismatch in assignment: variable '{}' has type {:?}, but assigned value has type {:?}", name, var_type, value_type),
                        None
                    ));
                }
            }
            Stmt::AssignField { object, field, value } => {
                // Check object type and field existence
                let obj_type = self.infer_expression_type(object)?;
                if let Type::Struct(struct_name) = obj_type {
                    if let Some((fields, _)) = self.struct_defs.get(&struct_name) {
                        if let Some((_, field_type_opt)) = fields.iter().find(|(fname, _)| fname == field) {
                            let value_type = self.infer_expression_type(value)?;
                            if let Some(field_type) = field_type_opt {
                                if !types_compatible(&value_type, field_type) {
                                    return Err(TogError::TypeError(
                                        format!("Type mismatch in field assignment: field '{}' has type {:?}, but assigned value has type {:?}", field, field_type, value_type),
                                        None
                                    ));
                                }
                            }
                        } else {
                            return Err(TogError::TypeError(
                                format!("Struct '{}' has no field '{}'", struct_name, field),
                                None
                            ));
                        }
                    }
                } else {
                    return Err(TogError::TypeError(
                        format!("Cannot assign field to non-struct type {:?}", obj_type),
                        None
                    ));
                }
            }
            Stmt::StructDef { name, fields, methods } => {
                self.struct_defs.insert(name.clone(), (fields.clone(), methods.clone()));
            }
            Stmt::EnumDef { .. } => {
                // Enum definitions - no type checking needed here
            }
            Stmt::TraitDef { .. } => {
                // Trait definitions - no type checking needed here
            }
            Stmt::ImplBlock { .. } => {
                // Impl blocks - type checking will be added later
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.infer_expression_type(expr)?;
                }
            }
            Stmt::Break | Stmt::Continue => {
                // No type checking needed for break/continue
            }
            Stmt::Expr(expr) => {
                self.infer_expression_type(expr)?;
            }
        }
        Ok(())
    }
    
    fn infer_expression_type(&self, expr: &Expr) -> Result<Type, TogError> {
        match expr {
            Expr::Literal(lit) => {
                Ok(match lit {
                    Literal::Int(_) => Type::Int,
                    Literal::Float(_) => Type::Float,
                    Literal::String(_) => Type::String,
                    Literal::Bool(_) => Type::Bool,
                    Literal::Array(elems) => {
                        if elems.is_empty() {
                            Type::Array(Box::new(Type::Infer))
                        } else {
                            let first_type = self.infer_expression_type(&elems[0])?;
                            Type::Array(Box::new(first_type))
                        }
                    }
                    Literal::None => Type::None,
                })
            }
            Expr::StructLiteral { name, .. } => {
                Ok(Type::Struct(name.clone()))
            }
            Expr::EnumVariant { enum_name, .. } => {
                Ok(Type::Enum(enum_name.clone()))
            }
            Expr::Variable(name) => {
                self.environment.get(name)
                    .cloned()
                    .ok_or_else(|| TogError::TypeError(
                        format!("Undefined variable: {}", name),
                        None
                    ))
            }
            Expr::BinaryOp { left, op, right } => {
                let left_type = self.infer_expression_type(left)?;
                let right_type = self.infer_expression_type(right)?;
                
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        // Arithmetic operations
                        let left_clone = left_type.clone();
                        let right_clone = right_type.clone();
                        match (left_type, right_type) {
                            (Type::Int, Type::Int) => Ok(Type::Int),
                            (Type::Float, _) | (_, Type::Float) => Ok(Type::Float),
                            (Type::String, _) | (_, Type::String) => Ok(Type::String),
                            _ => Err(TogError::TypeError(
                                format!("Invalid operation: {:?} {:?} {:?}", left_clone, op, right_clone),
                                None
                            )),
                        }
                    }
                    BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | 
                    BinaryOp::Gt | BinaryOp::Ge => {
                        Ok(Type::Bool)
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type == Type::Bool && right_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(TogError::TypeError("Logical operations require bool operands".to_string(), None))
                        }
                    }
                    BinaryOp::Mod => {
                        if left_type == Type::Int && right_type == Type::Int {
                            Ok(Type::Int)
                        } else {
                            Err(TogError::TypeError("Modulo requires int operands".to_string(), None))
                        }
                    }
                }
            }
            Expr::UnaryOp { op, expr } => {
                let expr_type = self.infer_expression_type(expr)?;
                match op {
                    UnaryOp::Not => {
                        if expr_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(TogError::TypeError("Not operator requires bool operand".to_string(), None))
                        }
                    }
                    UnaryOp::Neg => {
                        match expr_type {
                            Type::Int | Type::Float => Ok(expr_type),
                            _ => Err(TogError::TypeError("Negation requires numeric operand".to_string(), None)),
                        }
                    }
                }
            }
            Expr::Call { callee, args } => {
                // For builtin functions
                if let Expr::Variable(name) = callee.as_ref() {
                    match name.as_str() {
                        "print" => {
                            // print returns None
                            Ok(Type::None)
                        }
                        "len" => {
                            if args.len() == 1 {
                                Ok(Type::Int)
                            } else {
                                Err(TogError::TypeError("len() expects 1 argument".to_string(), None))
                            }
                        }
                        _ => {
                            // TODO: Look up function definition
                            Ok(Type::Infer)
                        }
                    }
                } else {
                    Ok(Type::Infer)
                }
            }
            Expr::Block(statements) => {
                let mut last_type = Type::None;
                for stmt in statements {
                    match stmt {
                        Stmt::Return(expr) => {
                            if let Some(expr) = expr {
                                last_type = self.infer_expression_type(expr)?;
                            }
                        }
                        Stmt::Expr(expr) => {
                            last_type = self.infer_expression_type(expr)?;
                        }
                        _ => {}
                    }
                }
                Ok(last_type)
            }
            Expr::If { then_branch, else_branch, .. } => {
                let then_type = self.infer_expression_type(then_branch)?;
                if let Some(else_expr) = else_branch {
                    let else_type = self.infer_expression_type(else_expr)?;
                    // Both branches should have compatible types
                    if types_compatible(&then_type, &else_type) {
                        Ok(then_type)
                    } else {
                        Ok(Type::Infer) // Incompatible types, infer
                    }
                } else {
                    Ok(Type::None)
                }
            }
            Expr::While { .. } => {
                Ok(Type::None)
            }
            Expr::For { .. } => {
                Ok(Type::None)
            }
            Expr::Match { .. } => {
                // TODO: Infer from match arms
                Ok(Type::Infer)
            }
            Expr::Function { return_type, .. } => {
                Ok(return_type.clone().unwrap_or(Type::Infer))
            }
            Expr::Index { array, index } => {
                let array_type = self.infer_expression_type(array)?;
                let index_type = self.infer_expression_type(index)?;
                
                // Index must be Int
                if index_type != Type::Int {
                    return Err(TogError::TypeError(
                        format!("Array index must be Int, got {:?}", index_type),
                        None
                    ));
                }
                
                // Get element type from array
                match array_type {
                    Type::Array(elem_type) => Ok(*elem_type),
                    Type::String => Ok(Type::String), // String indexing returns String (char)
                    _ => Err(TogError::TypeError(
                        format!("Cannot index type {:?}", array_type),
                        None
                    ))
                }
            }
            Expr::FieldAccess { object, field } => {
                let obj_type = self.infer_expression_type(object)?;
                match obj_type {
                    Type::Struct(name) => {
                        if let Some((fields, _)) = self.struct_defs.get(&name) {
                            if let Some((_, ty)) = fields.iter().find(|(fname, _)| fname == field) {
                                if let Some(t) = ty {
                                    return Ok(t.clone());
                                }
                            }
                        }
                        Ok(Type::Infer)
                    }
                    _ => Ok(Type::Infer),
                }
            }
        }
    }
}

fn types_compatible(t1: &Type, t2: &Type) -> bool {
    match (t1, t2) {
        (Type::Infer, _) | (_, Type::Infer) => true, // Infer is compatible with anything
        (a, b) => a == b,
    }
}

