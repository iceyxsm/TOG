// Code generation utilities
//
// This module provides utilities for generating code from IR.
// Each backend will implement its own code generation, but
// shared utilities go here.

use crate::compiler::ir::*;

// Helper functions for code generation that can be shared across backends

pub fn get_function_by_name<'a>(program: &'a IrProgram, name: &str) -> Option<&'a IrFunction> {
    program.functions.iter().find(|f| f.name == name)
}

pub fn is_builtin_function(name: &str) -> bool {
    matches!(name, "print" | "len" | "to_string")
}

pub fn estimate_function_size(func: &IrFunction) -> usize {
    // Estimate function size for inlining decisions
    // Simple heuristic: count statements
    count_statements_in_block(&func.body)
}

fn count_statements_in_block(block: &IrBlock) -> usize {
    match block {
        IrBlock::Block(statements) => {
            statements.len()
        }
        IrBlock::Expression(_) => 1,
    }
}

// Type information helpers with improved propagation
//
// Reasoning: Better type information enables:
// 1. Better optimizations (type-specific optimizations)
// 2. SIMD detection (need to know if operations are numeric)
// 3. Better code generation (can use specialized instructions)

pub struct TypeEnvironment {
    variables: std::collections::HashMap<String, crate::ast::Type>,
    functions: std::collections::HashMap<String, crate::ast::Type>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
            functions: std::collections::HashMap::new(),
        }
    }
    
    pub fn from_program(program: &IrProgram) -> Self {
        let mut env = Self::new();
        
        // Add global variables
        for global in &program.globals {
            env.variables.insert(global.name.clone(), global.value_type.clone());
        }
        
        // Add function return types
        for func in &program.functions {
            if let Some(return_type) = &func.return_type {
                env.functions.insert(func.name.clone(), return_type.clone());
            }
        }
        
        env
    }
    
    pub fn get_variable_type(&self, name: &str) -> Option<&crate::ast::Type> {
        self.variables.get(name)
    }
    
    pub fn get_function_type(&self, name: &str) -> Option<&crate::ast::Type> {
        self.functions.get(name)
    }
}

pub fn infer_expression_type(expr: &IrExpression, program: &IrProgram) -> Option<crate::ast::Type> {
    let env = TypeEnvironment::from_program(program);
    infer_expression_type_with_env(expr, program, &env)
}

pub fn infer_expression_type_with_env(
    expr: &IrExpression, 
    program: &IrProgram,
    env: &TypeEnvironment,
) -> Option<crate::ast::Type> {
    match expr {
        IrExpression::Literal(val) => {
            Some(match val {
                IrValue::Int(_) => crate::ast::Type::Int,
                IrValue::Float(_) => crate::ast::Type::Float,
                IrValue::String(_) => crate::ast::Type::String,
                IrValue::Bool(_) => crate::ast::Type::Bool,
                IrValue::None => crate::ast::Type::None,
                IrValue::Array(_) => crate::ast::Type::Array(Box::new(crate::ast::Type::Infer)),
            })
        }
        IrExpression::Variable(name) => {
            // Look up in environment
            env.get_variable_type(name).cloned()
        }
        IrExpression::BinaryOp { left, op, right } => {
            // Type inference for binary operations with improved propagation
            let left_type = infer_expression_type_with_env(left, program, env)?;
            let right_type = infer_expression_type_with_env(right, program, env)?;
            
            match op {
                crate::ast::BinaryOp::Add | crate::ast::BinaryOp::Sub | 
                crate::ast::BinaryOp::Mul | crate::ast::BinaryOp::Div => {
                    // Arithmetic operations - improved type promotion
                    match (left_type, right_type) {
                        (crate::ast::Type::Int, crate::ast::Type::Int) => Some(crate::ast::Type::Int),
                        (crate::ast::Type::Float, _) | (_, crate::ast::Type::Float) => Some(crate::ast::Type::Float),
                        (crate::ast::Type::String, _) | (_, crate::ast::Type::String) => Some(crate::ast::Type::String),
                        (crate::ast::Type::Infer, t) | (t, crate::ast::Type::Infer) => Some(t), // Propagate known type
                        _ => None,
                    }
                }
                crate::ast::BinaryOp::Eq | crate::ast::BinaryOp::Ne |
                crate::ast::BinaryOp::Lt | crate::ast::BinaryOp::Le |
                crate::ast::BinaryOp::Gt | crate::ast::BinaryOp::Ge => {
                    Some(crate::ast::Type::Bool)
                }
                crate::ast::BinaryOp::And | crate::ast::BinaryOp::Or => {
                    Some(crate::ast::Type::Bool)
                }
                crate::ast::BinaryOp::Mod => {
                    // Modulo requires integers
                    if left_type == crate::ast::Type::Int && right_type == crate::ast::Type::Int {
                        Some(crate::ast::Type::Int)
                    } else {
                        None
                    }
                }
            }
        }
        IrExpression::UnaryOp { op, expr } => {
            let expr_type = infer_expression_type_with_env(expr, program, env)?;
            match op {
                crate::ast::UnaryOp::Not => {
                    if expr_type == crate::ast::Type::Bool {
                        Some(crate::ast::Type::Bool)
                    } else {
                        None
                    }
                }
                crate::ast::UnaryOp::Neg => {
                    match expr_type {
                        crate::ast::Type::Int | crate::ast::Type::Float => Some(expr_type),
                        _ => None,
                    }
                }
            }
        }
        IrExpression::Call { callee, .. } => {
            env.get_function_type(callee).cloned()
                .or_else(|| {
                    get_function_by_name(program, callee)
                        .and_then(|f| f.return_type.clone())
                })
        }
        IrExpression::Index { base, .. } => {
            // Array indexing: if base is array[T], return T
            let base_type = infer_expression_type_with_env(base, program, env)?;
            match base_type {
                crate::ast::Type::Array(element_type) => Some(*element_type),
                _ => None,
            }
        }
    }
}

// Check if type is numeric (useful for SIMD detection)
pub fn is_numeric_type(ty: &crate::ast::Type) -> bool {
    matches!(ty, crate::ast::Type::Int | crate::ast::Type::Float)
}

// Check if expression is numeric (for vectorization)
pub fn is_numeric_expression(expr: &IrExpression, program: &IrProgram) -> bool {
    infer_expression_type(expr, program)
        .map(|ty| is_numeric_type(&ty))
        .unwrap_or(false)
}

