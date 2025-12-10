// Loop analysis for SIMD vectorization
//
// Reasoning: Detecting vectorizable loops is the first step toward SIMD optimization.
// We identify loops that can be safely vectorized:
// 1. Countable loops (known bounds)
// 2. No dependencies between iterations
// 3. Simple operations (add, mul, etc.)
// 4. Contiguous memory access

use crate::compiler::ir::*;
use crate::error::TogError;

#[derive(Debug, Clone)]
#[allow(dead_code)] // Will be used for SIMD vectorization
pub struct LoopInfo {
    pub is_vectorizable: bool,
    pub loop_type: LoopType,
    pub operation_type: OperationType,
    pub estimated_speedup: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Will be used for loop optimization classification
pub enum LoopType {
    SimpleIteration,  // for i in array
    IndexedLoop,     // for i in 0..n
    WhileLoop,       // while condition
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Reduction,       // sum, max, min
    ElementWise,    // a[i] + b[i]
    Map,            // transform each element
    Unknown,
}

pub fn analyze_loops(program: &IrProgram) -> Result<Vec<LoopInfo>, TogError> {
    let mut loops = Vec::new();
    
    for func in &program.functions {
        find_loops_in_block(&func.body, &mut loops)?;
    }
    
    Ok(loops)
}

fn find_loops_in_block(block: &IrBlock, loops: &mut Vec<LoopInfo>) -> Result<(), TogError> {
    match block {
        IrBlock::Block(statements) => {
            for stmt in statements {
                find_loops_in_stmt(stmt, loops)?;
            }
        }
        IrBlock::Expression(_) => {}
    }
    Ok(())
}

fn find_loops_in_stmt(stmt: &IrStatement, loops: &mut Vec<LoopInfo>) -> Result<(), TogError> {
    match stmt {
        IrStatement::While { condition, body } => {
            let info = analyze_while_loop(condition, body)?;
            loops.push(info);
            
            // Recursively find nested loops
            find_loops_in_block(body, loops)?;
        }
        IrStatement::If { then_branch, else_branch, .. } => {
            find_loops_in_block(then_branch, loops)?;
            if let Some(else_b) = else_branch {
                find_loops_in_block(else_b, loops)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn analyze_while_loop(condition: &IrExpression, body: &IrBlock) -> Result<LoopInfo, TogError> {
    // Simple heuristic: check if loop body has simple operations
    let operation_type = detect_operation_type(body);
    let is_vectorizable = is_simple_loop_body(body) && is_countable_loop(condition);
    
    let estimated_speedup = if is_vectorizable {
        match operation_type {
            OperationType::Reduction => 4.0,  // SIMD width
            OperationType::ElementWise => 6.0,
            OperationType::Map => 5.0,
            OperationType::Unknown => 1.0,
        }
    } else {
        1.0
    };
    
    Ok(LoopInfo {
        is_vectorizable,
        loop_type: LoopType::WhileLoop,
        operation_type,
        estimated_speedup,
    })
}

fn is_countable_loop(condition: &IrExpression) -> bool {
    // Check if loop has a clear iteration count
    // For now, we assume while loops with simple conditions might be countable
    // TODO: Implement proper analysis
    matches!(condition, IrExpression::BinaryOp { .. } | IrExpression::Variable(_))
}

fn is_simple_loop_body(block: &IrBlock) -> bool {
    // Check if loop body has simple operations that can be vectorized
    match block {
        IrBlock::Block(statements) => {
            // Simple heuristic: few statements, mostly arithmetic
            statements.len() < 10 && statements.iter().all(|s| is_simple_statement(s))
        }
        IrBlock::Expression(expr) => {
            is_simple_expression(expr)
        }
    }
}

fn is_simple_statement(stmt: &IrStatement) -> bool {
    match stmt {
        IrStatement::Let { value, .. } => {
            is_simple_expression(value)
        }
        IrStatement::Assign { value, .. } => {
            is_simple_expression(value)
        }
        IrStatement::Expression(expr) => {
            is_simple_expression(expr)
        }
        _ => false,
    }
}

fn is_simple_expression(expr: &IrExpression) -> bool {
    match expr {
        IrExpression::BinaryOp { op, .. } => {
            matches!(op, 
                crate::ast::BinaryOp::Add | 
                crate::ast::BinaryOp::Sub | 
                crate::ast::BinaryOp::Mul | 
                crate::ast::BinaryOp::Div
            )
        }
        IrExpression::UnaryOp { .. } => true,
        IrExpression::Literal(_) | IrExpression::Variable(_) => true,
        IrExpression::Index { .. } => true,
        _ => false,
    }
}

fn detect_operation_type(block: &IrBlock) -> OperationType {
    // Detect what kind of operation the loop performs
    // This helps determine the best vectorization strategy
    
    match block {
        IrBlock::Block(statements) => {
            // Look for reduction patterns (sum, max, etc.)
            for stmt in statements {
                if is_reduction_pattern(stmt) {
                    return OperationType::Reduction;
                }
            }
            
            // Look for element-wise operations
            if has_element_wise_operations(statements) {
                return OperationType::ElementWise;
            }
            
            OperationType::Unknown
        }
        IrBlock::Expression(expr) => {
            if is_simple_expression(expr) {
                OperationType::Map
            } else {
                OperationType::Unknown
            }
        }
    }
}

fn is_reduction_pattern(stmt: &IrStatement) -> bool {
    // Detect reduction patterns like: sum += arr[i] or sum = sum + arr[i]
    match stmt {
        IrStatement::Let { value, .. } => {
            matches!(value, IrExpression::BinaryOp { op: crate::ast::BinaryOp::Add, .. })
        }
        IrStatement::Assign { value, .. } => {
            matches!(value, IrExpression::BinaryOp { op: crate::ast::BinaryOp::Add, .. })
        }
        _ => false,
    }
}

fn has_element_wise_operations(statements: &[IrStatement]) -> bool {
    // Check if statements perform element-wise operations
    statements.iter().any(|s| {
        match s {
            IrStatement::Expression(IrExpression::BinaryOp { .. }) => true,
            IrStatement::Assign { value, .. } => {
                matches!(value, IrExpression::BinaryOp { .. })
            }
            IrStatement::Let { value, .. } => {
                matches!(value, IrExpression::BinaryOp { .. } | IrExpression::Index { .. })
            }
            _ => false,
        }
    })
}

// Future: Generate SIMD code for vectorizable loops
#[allow(dead_code)] // Will be used for SIMD code generation
pub fn generate_simd_code(_loop_info: &LoopInfo, _body: &IrBlock) -> Result<IrBlock, TogError> {
    // TODO: Transform loop body to use SIMD instructions
    // This would:
    // 1. Unroll loop by SIMD width
    // 2. Generate SIMD load/store operations
    // 3. Generate SIMD arithmetic operations
    // 4. Handle remainder elements
    Err(TogError::RuntimeError("SIMD code generation not yet implemented".to_string(), None))
}

