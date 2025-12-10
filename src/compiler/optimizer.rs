// Optimization passes for TOG IR
//
// Optimizations are applied in order:
// 1. Constant folding
// 2. Dead code elimination
// 3. Inlining
// 4. Loop optimizations
// 5. Memory optimizations

use crate::compiler::ir::*;
use crate::compiler::codegen::{TypeEnvironment};
use crate::error::TogError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    None,      // -O0: No optimizations
    Basic,     // -O1: Basic optimizations
    Standard,  // -O2: Standard optimizations (default)
    Aggressive, // -O3: Aggressive optimizations
    Size,      // -Os: Optimize for size
}

impl OptimizationLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" | "none" => Some(OptimizationLevel::None),
            "1" | "basic" => Some(OptimizationLevel::Basic),
            "2" | "standard" => Some(OptimizationLevel::Standard),
            "3" | "aggressive" => Some(OptimizationLevel::Aggressive),
            "s" | "size" => Some(OptimizationLevel::Size),
            _ => None,
        }
    }
}

pub fn optimize(program: &mut IrProgram, level: OptimizationLevel) -> Result<(), TogError> {
    if level == OptimizationLevel::None {
        return Ok(());
    }
    
    // Apply optimizations based on level
    match level {
        OptimizationLevel::None => {
            // No optimizations
        }
        OptimizationLevel::Basic => {
            constant_folding(program)?;
        }
        OptimizationLevel::Standard => {
            constant_folding(program)?;
            dead_code_elimination(program)?;
            simple_inlining(program)?;
        }
        OptimizationLevel::Aggressive => {
            constant_folding(program)?;
            dead_code_elimination(program)?;
            aggressive_inlining(program)?;
            loop_optimizations(program)?;
        }
        OptimizationLevel::Size => {
            constant_folding(program)?;
            dead_code_elimination(program)?;
            // Size optimizations would go here
        }
    }
    
    Ok(())
}

// Constant folding: Evaluate constant expressions at compile time
fn constant_folding(program: &mut IrProgram) -> Result<(), TogError> {
    // Use TypeEnvironment for better type-aware constant folding
    let _env = TypeEnvironment::from_program(program);
    
    for func in &mut program.functions {
        fold_constants_in_block(&mut func.body)?;
    }
    Ok(())
}

fn fold_constants_in_block(block: &mut IrBlock) -> Result<(), TogError> {
    match block {
        IrBlock::Block(statements) => {
            for stmt in statements {
                fold_constants_in_stmt(stmt)?;
            }
        }
        IrBlock::Expression(expr) => {
            *expr = fold_constant_expr(expr)?;
        }
    }
    Ok(())
}

fn fold_constants_in_stmt(stmt: &mut IrStatement) -> Result<(), TogError> {
    match stmt {
        IrStatement::Let { value, .. } => {
            *value = fold_constant_expr(value)?;
        }
        IrStatement::Assign { value, .. } => {
            *value = fold_constant_expr(value)?;
        }
        IrStatement::Return(expr) => {
            if let Some(e) = expr {
                *e = fold_constant_expr(e)?;
            }
        }
        IrStatement::Expression(expr) => {
            *expr = fold_constant_expr(expr)?;
        }
        IrStatement::If { condition, then_branch, else_branch } => {
            *condition = fold_constant_expr(condition)?;
            fold_constants_in_block(then_branch)?;
            if let Some(else_b) = else_branch {
                fold_constants_in_block(else_b)?;
            }
        }
        IrStatement::While { condition, body } => {
            *condition = fold_constant_expr(condition)?;
            fold_constants_in_block(body)?;
        }
        IrStatement::Break | IrStatement::Continue => {
            // No optimization needed
        }
    }
    Ok(())
}

fn fold_constant_expr(expr: &IrExpression) -> Result<IrExpression, TogError> {
    match expr {
        IrExpression::BinaryOp { left, op, right } => {
            // Try to evaluate if both are literals
            if let (IrExpression::Literal(left_val), IrExpression::Literal(right_val)) = 
                (left.as_ref(), right.as_ref()) {
                if let Some(result) = evaluate_binary_op(left_val, *op, right_val)? {
                    return Ok(IrExpression::Literal(result));
                }
            }
            
            // Recursively fold children
            let folded_left = fold_constant_expr(left)?;
            let folded_right = fold_constant_expr(right)?;
            
            // Try again after folding children
            if let (IrExpression::Literal(left_val), IrExpression::Literal(right_val)) = 
                (&folded_left, &folded_right) {
                if let Some(result) = evaluate_binary_op(left_val, *op, right_val)? {
                    return Ok(IrExpression::Literal(result));
                }
            }
            
            Ok(IrExpression::BinaryOp {
                left: Box::new(folded_left),
                op: *op,
                right: Box::new(folded_right),
            })
        }
        IrExpression::UnaryOp { op, expr } => {
            let folded = fold_constant_expr(expr)?;
            if let IrExpression::Literal(val) = &folded {
                if let Some(result) = evaluate_unary_op(*op, val)? {
                    return Ok(IrExpression::Literal(result));
                }
            }
            Ok(IrExpression::UnaryOp {
                op: *op,
                expr: Box::new(folded),
            })
        }
        _ => Ok(expr.clone()),
    }
}

fn evaluate_binary_op(left: &IrValue, op: crate::ast::BinaryOp, right: &IrValue) -> Result<Option<IrValue>, TogError> {
    match (left, op, right) {
        (IrValue::Int(a), crate::ast::BinaryOp::Add, IrValue::Int(b)) => {
            Ok(Some(IrValue::Int(a + b)))
        }
        (IrValue::Int(a), crate::ast::BinaryOp::Sub, IrValue::Int(b)) => {
            Ok(Some(IrValue::Int(a - b)))
        }
        (IrValue::Int(a), crate::ast::BinaryOp::Mul, IrValue::Int(b)) => {
            Ok(Some(IrValue::Int(a * b)))
        }
        (IrValue::Int(a), crate::ast::BinaryOp::Div, IrValue::Int(b)) => {
            if *b == 0 {
                return Err(TogError::RuntimeError("Division by zero".to_string(), None));
            }
            Ok(Some(IrValue::Int(a / b)))
        }
        (IrValue::Int(a), crate::ast::BinaryOp::Eq, IrValue::Int(b)) => {
            Ok(Some(IrValue::Bool(a == b)))
        }
        (IrValue::Int(a), crate::ast::BinaryOp::Ne, IrValue::Int(b)) => {
            Ok(Some(IrValue::Bool(a != b)))
        }
        _ => Ok(None), // Can't evaluate at compile time
    }
}

fn evaluate_unary_op(op: crate::ast::UnaryOp, val: &IrValue) -> Result<Option<IrValue>, TogError> {
    match (op, val) {
        (crate::ast::UnaryOp::Neg, IrValue::Int(n)) => {
            Ok(Some(IrValue::Int(-n)))
        }
        (crate::ast::UnaryOp::Not, IrValue::Bool(b)) => {
            Ok(Some(IrValue::Bool(!b)))
        }
        _ => Ok(None),
    }
}

// Dead code elimination: Remove unreachable code
// 
// Reasoning: Removing dead code reduces binary size and improves cache locality.
// This is a foundational optimization that enables better performance.
fn dead_code_elimination(program: &mut IrProgram) -> Result<(), TogError> {
    // Remove unreachable code after returns in each function
    for func in &mut program.functions {
        remove_unreachable_code(&mut func.body)?;
    }
    
    // Remove unused functions (functions that are never called)
    remove_unused_functions(program)?;
    
    Ok(())
}

// Remove unreachable code after return statements
fn remove_unreachable_code(block: &mut IrBlock) -> Result<(), TogError> {
    match block {
        IrBlock::Block(statements) => {
            let mut new_statements = Vec::new();
            let mut found_return = false;
            
            for mut stmt in statements.drain(..) {
                if found_return {
                    // Skip unreachable code after return
                    continue;
                }
                
                match &mut stmt {
                    IrStatement::Return(_) => {
                        found_return = true;
                        new_statements.push(stmt);
                    }
                    IrStatement::If { then_branch, else_branch, .. } => {
                        // Recursively clean branches
                        remove_unreachable_code(then_branch.as_mut())?;
                        if let Some(else_b) = else_branch {
                            remove_unreachable_code(else_b.as_mut())?;
                        }
                        new_statements.push(stmt);
                    }
                    IrStatement::While { body, .. } => {
                        // Clean loop body
                        remove_unreachable_code(body.as_mut())?;
                        new_statements.push(stmt);
                    }
                    IrStatement::Assign { .. } | IrStatement::Let { .. } | IrStatement::Expression(_) | IrStatement::Break | IrStatement::Continue => {
                        new_statements.push(stmt);
                    }
                }
            }
            
            *statements = new_statements;
        }
        IrBlock::Expression(_) => {
            // Single expression, nothing to remove
        }
    }
    Ok(())
}

// Remove functions that are never called
fn remove_unused_functions(program: &mut IrProgram) -> Result<(), TogError> {
    // Collect all function names that are called
    let mut called_functions = std::collections::HashSet::new();
    
    // Always keep main function
    called_functions.insert("main".to_string());
    
    // Find all function calls
    for func in &program.functions {
        find_function_calls(&func.body, &mut called_functions);
    }
    
    // Remove functions that are never called
    program.functions.retain(|f| {
        called_functions.contains(&f.name) || f.is_public
    });
    
    Ok(())
}

fn find_function_calls(block: &IrBlock, called: &mut std::collections::HashSet<String>) {
    match block {
        IrBlock::Block(statements) => {
            for stmt in statements {
                find_function_calls_in_stmt(stmt, called);
            }
        }
        IrBlock::Expression(expr) => {
            find_function_calls_in_expr(expr, called);
        }
    }
}

fn find_function_calls_in_stmt(stmt: &IrStatement, called: &mut std::collections::HashSet<String>) {
    match stmt {
        IrStatement::Let { value, .. } => {
            find_function_calls_in_expr(value, called);
        }
        IrStatement::Assign { value, .. } => {
            find_function_calls_in_expr(value, called);
        }
        IrStatement::Return(expr) => {
            if let Some(e) = expr {
                find_function_calls_in_expr(e, called);
            }
        }
        IrStatement::Expression(expr) => {
            find_function_calls_in_expr(expr, called);
        }
        IrStatement::If { condition, then_branch, else_branch, .. } => {
            find_function_calls_in_expr(condition, called);
            find_function_calls(then_branch, called);
            if let Some(else_b) = else_branch {
                find_function_calls(else_b, called);
            }
        }
        IrStatement::While { condition, body, .. } => {
            find_function_calls_in_expr(condition, called);
            find_function_calls(body, called);
        }
        IrStatement::Break | IrStatement::Continue => {
            // No function calls
        }
    }
}

fn find_function_calls_in_expr(expr: &IrExpression, called: &mut std::collections::HashSet<String>) {
    match expr {
        IrExpression::Call { callee, args } => {
            called.insert(callee.clone());
            for arg in args {
                find_function_calls_in_expr(arg, called);
            }
        }
        IrExpression::BinaryOp { left, right, .. } => {
            find_function_calls_in_expr(left, called);
            find_function_calls_in_expr(right, called);
        }
        IrExpression::UnaryOp { expr, .. } => {
            find_function_calls_in_expr(expr, called);
        }
        IrExpression::Index { base, index } => {
            find_function_calls_in_expr(base, called);
            find_function_calls_in_expr(index, called);
        }
        _ => {}
    }
}

// Simple inlining: Inline small functions
//
// Reasoning: Inlining eliminates function call overhead and enables
// better optimizations (constant propagation, dead code elimination).
// We inline small functions (< 10 statements) that are called frequently.
fn simple_inlining(program: &mut IrProgram) -> Result<(), TogError> {
    // Find functions that are good candidates for inlining
    // Criteria: Small size (< 10 statements), not recursive
    let mut inline_candidates = Vec::new();
    
    for (idx, func) in program.functions.iter().enumerate() {
        let size = estimate_function_size(func);
        if size < 10 && !is_recursive(func, &program.functions) {
            inline_candidates.push(idx);
        }
    }
    
    // Inline candidates (limit iterations to avoid infinite loops)
    for _iteration in 0..3 {
        let mut inlined_any = false;
        
        let functions_clone = program.functions.clone();
        for func in &mut program.functions {
            inline_function_calls(func, &functions_clone, &inline_candidates, &mut inlined_any)?;
        }
        
        if !inlined_any {
            break;
        }
    }
    
    Ok(())
}

// Estimate function size for inlining decisions
fn estimate_function_size(func: &IrFunction) -> usize {
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

// Check if function is recursive (simplified check)
fn is_recursive(func: &IrFunction, _all_functions: &[IrFunction]) -> bool {
    // Simple check: if function calls itself
    let mut calls_self = false;
    find_function_calls_in_block(&func.body, &func.name, &mut calls_self);
    calls_self
}

fn find_function_calls_in_block(block: &IrBlock, target: &str, found: &mut bool) {
    match block {
        IrBlock::Block(statements) => {
            for stmt in statements {
                find_function_calls_in_stmt_for_target(stmt, target, found);
            }
        }
        IrBlock::Expression(expr) => {
            find_function_calls_in_expr_for_target(expr, target, found);
        }
    }
}

fn find_function_calls_in_stmt_for_target(stmt: &IrStatement, target: &str, found: &mut bool) {
    match stmt {
        IrStatement::Let { value, .. } => {
            find_function_calls_in_expr_for_target(value, target, found);
        }
        IrStatement::Assign { value, .. } => {
            find_function_calls_in_expr_for_target(value, target, found);
        }
        IrStatement::Return(expr) => {
            if let Some(e) = expr {
                find_function_calls_in_expr_for_target(e, target, found);
            }
        }
        IrStatement::Expression(expr) => {
            find_function_calls_in_expr_for_target(expr, target, found);
        }
        IrStatement::If { condition, then_branch, else_branch, .. } => {
            find_function_calls_in_expr_for_target(condition, target, found);
            find_function_calls_in_block(then_branch, target, found);
            if let Some(else_b) = else_branch {
                find_function_calls_in_block(else_b, target, found);
            }
        }
        IrStatement::While { condition, body, .. } => {
            find_function_calls_in_expr_for_target(condition, target, found);
            find_function_calls_in_block(body, target, found);
        }
        IrStatement::Break | IrStatement::Continue => {
            // No function calls
        }
    }
}

fn find_function_calls_in_expr_for_target(expr: &IrExpression, target: &str, found: &mut bool) {
    match expr {
        IrExpression::Call { callee, .. } => {
            if callee == target {
                *found = true;
            }
        }
        IrExpression::BinaryOp { left, right, .. } => {
            find_function_calls_in_expr_for_target(left, target, found);
            find_function_calls_in_expr_for_target(right, target, found);
        }
        IrExpression::UnaryOp { expr, .. } => {
            find_function_calls_in_expr_for_target(expr, target, found);
        }
        _ => {}
    }
}

// Inline function calls in a function body
fn inline_function_calls(
    func: &mut IrFunction,
    all_functions: &[IrFunction],
    candidates: &[usize],
    inlined_any: &mut bool,
) -> Result<(), TogError> {
    inline_calls_in_block(&mut func.body, all_functions, candidates, inlined_any)?;
    Ok(())
}

fn inline_calls_in_block(
    block: &mut IrBlock,
    all_functions: &[IrFunction],
    candidates: &[usize],
    inlined_any: &mut bool,
) -> Result<(), TogError> {
    match block {
        IrBlock::Block(statements) => {
            let mut new_statements = Vec::new();
            
            for stmt in statements.drain(..) {
                match stmt {
                    IrStatement::Expression(IrExpression::Call { callee, args }) => {
                        // Try to inline this call
                        if let Some(target_func) = all_functions.iter().find(|f| f.name == callee) {
                            if candidates.contains(&all_functions.iter().position(|f| f.name == target_func.name).unwrap_or(usize::MAX)) {
                                // Inline the function
                                if let Ok(inlined) = inline_call(target_func, &args) {
                                    new_statements.push(IrStatement::Expression(inlined));
                                    *inlined_any = true;
                                    continue;
                                }
                            }
                        }
                        // Couldn't inline, keep original call
                        new_statements.push(IrStatement::Expression(IrExpression::Call { callee, args }));
                    }
                    mut stmt => {
                        // Recursively inline in nested structures
                        inline_calls_in_stmt(&mut stmt, all_functions, candidates, inlined_any)?;
                        new_statements.push(stmt);
                    }
                }
            }
            
            *statements = new_statements;
        }
        IrBlock::Expression(expr) => {
            if let IrExpression::Call { callee, args } = expr {
                if let Some(target_func) = all_functions.iter().find(|f| f.name == *callee) {
                    if candidates.contains(&all_functions.iter().position(|f| f.name == target_func.name).unwrap_or(usize::MAX)) {
                        if let Ok(inlined) = inline_call(target_func, &args) {
                            *expr = inlined;
                            *inlined_any = true;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn inline_calls_in_stmt(
    stmt: &mut IrStatement,
    all_functions: &[IrFunction],
    candidates: &[usize],
    inlined_any: &mut bool,
) -> Result<(), TogError> {
    match stmt {
        IrStatement::Let { value, .. } => {
            inline_calls_in_expr(value, all_functions, candidates, inlined_any)?;
        }
        IrStatement::Assign { value, .. } => {
            inline_calls_in_expr(value, all_functions, candidates, inlined_any)?;
        }
        IrStatement::Return(expr) => {
            if let Some(e) = expr {
                inline_calls_in_expr(e, all_functions, candidates, inlined_any)?;
            }
        }
        IrStatement::Expression(expr) => {
            inline_calls_in_expr(expr, all_functions, candidates, inlined_any)?;
        }
        IrStatement::If { condition, then_branch, else_branch, .. } => {
            inline_calls_in_expr(condition, all_functions, candidates, inlined_any)?;
            inline_calls_in_block(then_branch, all_functions, candidates, inlined_any)?;
            if let Some(else_b) = else_branch {
                inline_calls_in_block(else_b, all_functions, candidates, inlined_any)?;
            }
        }
        IrStatement::While { condition, body, .. } => {
            inline_calls_in_expr(condition, all_functions, candidates, inlined_any)?;
            inline_calls_in_block(body, all_functions, candidates, inlined_any)?;
        }
        IrStatement::Break | IrStatement::Continue => {
            // No function calls to inline
        }
    }
    Ok(())
}

fn inline_calls_in_expr(
    expr: &mut IrExpression,
    all_functions: &[IrFunction],
    candidates: &[usize],
    inlined_any: &mut bool,
) -> Result<(), TogError> {
    match expr {
        IrExpression::Call { callee, args } => {
            if let Some(target_func) = all_functions.iter().find(|f| f.name == *callee) {
                if candidates.contains(&all_functions.iter().position(|f| f.name == target_func.name).unwrap_or(usize::MAX)) {
                    if let Ok(inlined) = inline_call(target_func, args) {
                        *expr = inlined;
                        *inlined_any = true;
                    }
                }
            }
        }
        IrExpression::BinaryOp { left, right, .. } => {
            inline_calls_in_expr(left, all_functions, candidates, inlined_any)?;
            inline_calls_in_expr(right, all_functions, candidates, inlined_any)?;
        }
        IrExpression::UnaryOp { expr, .. } => {
            inline_calls_in_expr(expr, all_functions, candidates, inlined_any)?;
        }
        IrExpression::Index { base, index } => {
            inline_calls_in_expr(base, all_functions, candidates, inlined_any)?;
            inline_calls_in_expr(index, all_functions, candidates, inlined_any)?;
        }
        _ => {}
    }
    Ok(())
}

// Inline a function call by replacing it with the function body
// with parameter substitution
fn inline_call(func: &IrFunction, args: &[IrExpression]) -> Result<IrExpression, TogError> {
    if args.len() != func.params.len() {
        return Err(TogError::RuntimeError(
            format!("Argument count mismatch: expected {}, got {}", func.params.len(), args.len()),
            None
        ));
    }
    
    // For now, simple inlining: replace function body expression
    // TODO: Handle parameter substitution properly
    match &func.body {
        IrBlock::Expression(expr) => {
            Ok(expr.clone())
        }
        IrBlock::Block(_) => {
            // Complex function body - would need proper variable renaming
            // For now, don't inline
            Err(TogError::RuntimeError("Complex function body inlining not yet implemented".to_string(), None))
        }
    }
}

// Aggressive inlining: Inline more functions based on heuristics
fn aggressive_inlining(_program: &mut IrProgram) -> Result<(), TogError> {
    // TODO: Implement aggressive inlining
    // - Use profile data if available
    // - Inline hot functions
    Ok(())
}

// Loop optimizations: Unroll, fuse, vectorize loops
//
// Reasoning: Loop optimizations are crucial for performance, especially for
// numerical computing. We analyze loops and apply transformations:
// 1. Loop unrolling: Reduce loop overhead
// 2. Loop fusion: Combine multiple loops
// 3. SIMD vectorization: Use CPU vector instructions
fn loop_optimizations(program: &mut IrProgram) -> Result<(), TogError> {
    // Analyze loops for vectorization opportunities
    let _loop_infos = crate::compiler::loop_analysis::analyze_loops(program)?;
    
    // For now, we just analyze. Actual transformation would happen here.
    // TODO: Apply loop unrolling
    // TODO: Apply loop fusion
    // TODO: Apply SIMD vectorization based on loop_infos
    
    Ok(())
}

