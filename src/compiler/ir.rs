// Intermediate Representation (IR) for TOG
// 
// IR is backend-agnostic and allows for optimizations before code generation.
// It's simpler than LLVM IR but more structured than AST.

use crate::ast::*;
use crate::error::TogError;

#[derive(Debug, Clone)]
pub struct IrProgram {
    pub functions: Vec<IrFunction>,
    pub globals: Vec<IrGlobal>,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<IrParam>,
    pub return_type: Option<Type>,
    pub body: IrBlock,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct IrParam {
    pub name: String,
    pub param_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct IrGlobal {
    pub name: String,
    pub value_type: Type,
    pub initializer: IrValue,
}

#[derive(Debug, Clone)]
pub enum IrBlock {
    Block(Vec<IrStatement>),
    Expression(IrExpression),
}

#[derive(Debug, Clone)]
pub enum IrStatement {
    Let {
        name: String,
        value: IrExpression,
    },
    Assign {
        name: String,
        value: IrExpression,
    },
    Return(Option<IrExpression>),
    Break,
    Continue,
    Expression(IrExpression),
    If {
        condition: IrExpression,
        then_branch: Box<IrBlock>,
        else_branch: Option<Box<IrBlock>>,
    },
    While {
        condition: IrExpression,
        body: Box<IrBlock>,
    },
}

#[derive(Debug, Clone)]
pub enum IrExpression {
    Literal(IrValue),
    Variable(String),
    BinaryOp {
        left: Box<IrExpression>,
        op: BinaryOp,
        right: Box<IrExpression>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<IrExpression>,
    },
    Call {
        callee: String,
        args: Vec<IrExpression>,
    },
    Index {
        base: Box<IrExpression>,
        index: Box<IrExpression>,
    },
}

#[derive(Debug, Clone)]
pub enum IrValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    #[allow(dead_code)] // Will be used for array literal optimization
    Array(Vec<IrExpression>),
    None,
}

pub fn ast_to_ir(program: Program) -> Result<IrProgram, TogError> {
    let mut functions = Vec::new();
    let mut globals = Vec::new();
    
    for stmt in program.statements {
        match stmt {
            Stmt::Expr(Expr::Function { name, params, return_type, body }) => {
                let ir_params: Vec<IrParam> = params.iter().map(|p| IrParam {
                    name: p.name.clone(),
                    param_type: p.type_annotation.clone(),
                }).collect();
                
                let ir_body = expr_to_ir_block(&body)?;
                
                functions.push(IrFunction {
                    name,
                    params: ir_params,
                    return_type: return_type.clone(),
                    body: ir_body,
                    is_public: true, // TODO: Determine from AST
                });
            }
            Stmt::Let { name, type_annotation, value } => {
                // Global variable
                let value_type = type_annotation.unwrap_or(Type::Infer);
                let ir_value = expr_to_ir_value(&value)?;
                
                globals.push(IrGlobal {
                    name,
                    value_type,
                    initializer: ir_value,
                });
            }
            _ => {
                // Other statements in global scope
                // TODO: Handle these
            }
        }
    }
    
    Ok(IrProgram { functions, globals })
}

fn expr_to_ir_block(expr: &Expr) -> Result<IrBlock, TogError> {
    match expr {
        Expr::Block(statements) => {
            let mut ir_stmts = Vec::new();
            for stmt in statements {
                ir_stmts.push(stmt_to_ir(stmt)?);
            }
            Ok(IrBlock::Block(ir_stmts))
        }
        _ => {
            let ir_expr = expr_to_ir_expr(expr)?;
            Ok(IrBlock::Expression(ir_expr))
        }
    }
}

fn stmt_to_ir(stmt: &Stmt) -> Result<IrStatement, TogError> {
    match stmt {
        Stmt::Let { name, value, .. } => {
            Ok(IrStatement::Let {
                name: name.clone(),
                value: expr_to_ir_expr(value)?,
            })
        }
        Stmt::Assign { name, value } => {
            Ok(IrStatement::Assign {
                name: name.clone(),
                value: expr_to_ir_expr(value)?,
            })
        }
        Stmt::AssignField { object: _, field: _, value: _ } => {
            // Field assignment not yet supported in IR
            Err(TogError::RuntimeError("Field assignment not yet supported in IR".to_string(), None))
        }
        Stmt::Return(expr) => {
            let ir_expr = expr.as_ref().map(expr_to_ir_expr).transpose()?;
            Ok(IrStatement::Return(ir_expr))
        }
        Stmt::Break => {
            Ok(IrStatement::Break)
        }
        Stmt::Continue => {
            Ok(IrStatement::Continue)
        }
        Stmt::StructDef { .. } => {
            Err(TogError::RuntimeError(
                "Struct definitions not yet supported in IR conversion".to_string(),
                None
            ))
        }
        Stmt::EnumDef { .. } => {
            Err(TogError::RuntimeError(
                "Enum definitions not yet supported in IR conversion".to_string(),
                None
            ))
        }
        Stmt::TraitDef { .. } => {
            Err(TogError::RuntimeError(
                "Trait definitions not yet supported in IR conversion".to_string(),
                None
            ))
        }
        Stmt::ImplBlock { .. } => {
            Err(TogError::RuntimeError(
                "Impl blocks not yet supported in IR conversion".to_string(),
                None
            ))
        }
        Stmt::Expr(expr) => {
            match expr {
                Expr::If { condition, then_branch, else_branch } => {
                    let else_ir = if let Some(else_expr) = else_branch {
                        Some(Box::new(expr_to_ir_block(else_expr)?))
                    } else {
                        None
                    };
                    Ok(IrStatement::If {
                        condition: expr_to_ir_expr(condition)?,
                        then_branch: Box::new(expr_to_ir_block(then_branch)?),
                        else_branch: else_ir,
                    })
                }
                Expr::While { condition, body } => {
                    Ok(IrStatement::While {
                        condition: expr_to_ir_expr(condition)?,
                        body: Box::new(expr_to_ir_block(body)?),
                    })
                }
                _ => {
                    Ok(IrStatement::Expression(expr_to_ir_expr(expr)?))
                }
            }
        }
    }
}

fn expr_to_ir_expr(expr: &Expr) -> Result<IrExpression, TogError> {
    match expr {
        Expr::Literal(lit) => {
            Ok(IrExpression::Literal(literal_to_ir_value(lit)?))
        }
        Expr::Variable(name) => {
            Ok(IrExpression::Variable(name.clone()))
        }
        Expr::BinaryOp { left, op, right } => {
            Ok(IrExpression::BinaryOp {
                left: Box::new(expr_to_ir_expr(left)?),
                op: *op,
                right: Box::new(expr_to_ir_expr(right)?),
            })
        }
        Expr::UnaryOp { op, expr } => {
            Ok(IrExpression::UnaryOp {
                op: *op,
                expr: Box::new(expr_to_ir_expr(expr)?),
            })
        }
        Expr::Call { callee, args } => {
            let callee_name = match callee.as_ref() {
                Expr::Variable(name) => name.clone(),
                _ => return Err(TogError::RuntimeError("Only variable calls supported in IR".to_string(), None)),
            };
            
            let ir_args: Result<Vec<IrExpression>, TogError> = 
                args.iter().map(expr_to_ir_expr).collect();
            
            Ok(IrExpression::Call {
                callee: callee_name,
                args: ir_args?,
            })
        }
        Expr::Index { array, index } => {
            Ok(IrExpression::Index {
                base: Box::new(expr_to_ir_expr(array)?),
                index: Box::new(expr_to_ir_expr(index)?),
            })
        }
        Expr::StructLiteral { .. } => {
            Err(TogError::RuntimeError(
                "Struct literals not yet supported in IR codegen".to_string(),
                None
            ))
        }
        Expr::FieldAccess { .. } => {
            Err(TogError::RuntimeError(
                "Field access not yet supported in IR codegen".to_string(),
                None
            ))
        }
        Expr::For { variable: _variable, iterable: _iterable, body: _body } => {
            // For loops in IR - convert to while loop for now
            // TODO: Implement proper for loop in IR
            Err(TogError::RuntimeError(
                "For loops not yet implemented in IR conversion".to_string(),
                None
            ))
        }
        Expr::EnumVariant { .. } => {
            Err(TogError::RuntimeError(
                "Enum variants not yet supported in IR codegen".to_string(),
                None
            ))
        }
        _ => {
            Err(TogError::RuntimeError("Unsupported expression in IR conversion".to_string(), None))
        }
    }
}

fn expr_to_ir_value(expr: &Expr) -> Result<IrValue, TogError> {
    match expr {
        Expr::Literal(lit) => literal_to_ir_value(lit),
        _ => Err(TogError::RuntimeError("Expected literal value".to_string(), None)),
    }
}

fn literal_to_ir_value(lit: &Literal) -> Result<IrValue, TogError> {
    match lit {
        Literal::Int(n) => Ok(IrValue::Int(*n)),
        Literal::Float(n) => Ok(IrValue::Float(*n)),
        Literal::String(s) => Ok(IrValue::String(s.clone())),
        Literal::Bool(b) => Ok(IrValue::Bool(*b)),
        Literal::Array(_elems) => {
            // For now, we'll represent arrays as a list of expressions
            // In a real implementation, we'd need proper array handling
            Err(TogError::RuntimeError("Array literals in IR not yet implemented".to_string(), None))
        }
        Literal::None => Ok(IrValue::None),
    }
}

