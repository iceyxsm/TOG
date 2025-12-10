// Simple native code generator
//
// This generates C-like code as an intermediate step before full LLVM integration.
// Reasoning: 
// 1. Useful for testing optimizations
// 2. Can be compiled with GCC/Clang for immediate native code
// 3. Easier to debug than LLVM IR
// 4. Stepping stone to full LLVM backend

use crate::compiler::ir::*;
use crate::error::TogError;

pub struct NativeCodeGenerator {
    output: String,
    indent_level: usize,
}

impl NativeCodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }
    
    pub fn generate_c_code(program: &IrProgram) -> Result<String, TogError> {
        let mut gen = Self::new();
        
        gen.output.push_str("#include <stdio.h>\n");
        gen.output.push_str("#include <stdint.h>\n");
        gen.output.push_str("#include <stdbool.h>\n");
        gen.output.push_str("#include <string.h>\n\n");
        
        // Generate globals
        for global in &program.globals {
            gen.generate_global(global)?;
        }
        
        // Generate functions
        for func in &program.functions {
            gen.generate_function(func)?;
        }
        
        Ok(gen.output)
    }
    
    fn generate_global(&mut self, global: &IrGlobal) -> Result<(), TogError> {
        let c_type = type_to_c_type(&global.value_type);
        self.output.push_str(&format!("{} {} = ", c_type, global.name));
        self.generate_value(&global.initializer)?;
        self.output.push_str(";\n");
        Ok(())
    }
    
    fn generate_function(&mut self, func: &IrFunction) -> Result<(), TogError> {
        let return_type = func.return_type.as_ref()
            .map(type_to_c_type)
            .unwrap_or_else(|| "void".to_string());
        
        // Function signature
        self.output.push_str(&format!("{} {}(", return_type, func.name));
        
        // Parameters
        let params: Vec<String> = func.params.iter().map(|p| {
            let param_type = p.param_type.as_ref()
                .map(type_to_c_type)
                .unwrap_or_else(|| "int64_t".to_string());
            format!("{} {}", param_type, p.name)
        }).collect();
        
        self.output.push_str(&params.join(", "));
        self.output.push_str(") {\n");
        
        self.indent_level += 1;
        self.generate_block(&func.body)?;
        self.indent_level -= 1;
        
        self.output.push_str("}\n\n");
        Ok(())
    }
    
    fn generate_block(&mut self, block: &IrBlock) -> Result<(), TogError> {
        match block {
            IrBlock::Block(statements) => {
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
            }
            IrBlock::Expression(expr) => {
                self.indent();
                self.generate_expression(expr)?;
                self.output.push_str(";\n");
            }
        }
        Ok(())
    }
    
    fn generate_statement(&mut self, stmt: &IrStatement) -> Result<(), TogError> {
        self.indent();
        
        match stmt {
            IrStatement::Let { name, value } => {
                // Infer type from value (simplified)
                self.output.push_str("int64_t "); // TODO: Proper type inference
                self.output.push_str(name);
                self.output.push_str(" = ");
                self.generate_expression(value)?;
                self.output.push_str(";\n");
            }
            IrStatement::Assign { name, value } => {
                self.indent();
                self.output.push_str(name);
                self.output.push_str(" = ");
                self.generate_expression(value)?;
                self.output.push_str(";\n");
            }
            IrStatement::Return(expr) => {
                self.output.push_str("return");
                if let Some(e) = expr {
                    self.output.push_str(" ");
                    self.generate_expression(e)?;
                }
                self.output.push_str(";\n");
            }
            IrStatement::Break => {
                self.indent();
                self.output.push_str("break;\n");
            }
            IrStatement::Continue => {
                self.indent();
                self.output.push_str("continue;\n");
            }
            IrStatement::Expression(expr) => {
                self.generate_expression(expr)?;
                self.output.push_str(";\n");
            }
            IrStatement::If { condition, then_branch, else_branch } => {
                self.output.push_str("if (");
                self.generate_expression(condition)?;
                self.output.push_str(") {\n");
                
                self.indent_level += 1;
                self.generate_block(then_branch)?;
                self.indent_level -= 1;
                
                if let Some(else_b) = else_branch {
                    self.indent();
                    self.output.push_str("} else {\n");
                    self.indent_level += 1;
                    self.generate_block(else_b)?;
                    self.indent_level -= 1;
                }
                
                self.indent();
                self.output.push_str("}\n");
            }
            IrStatement::While { condition, body } => {
                self.output.push_str("while (");
                self.generate_expression(condition)?;
                self.output.push_str(") {\n");
                
                self.indent_level += 1;
                self.generate_block(body)?;
                self.indent_level -= 1;
                
                self.indent();
                self.output.push_str("}\n");
            }
        }
        
        Ok(())
    }
    
    fn generate_expression(&mut self, expr: &IrExpression) -> Result<(), TogError> {
        match expr {
            IrExpression::Literal(val) => {
                self.generate_value(val)?;
            }
            IrExpression::Variable(name) => {
                self.output.push_str(name);
            }
            IrExpression::BinaryOp { left, op, right } => {
                self.output.push_str("(");
                self.generate_expression(left)?;
                self.output.push_str(" ");
                self.output.push_str(binary_op_to_c(op));
                self.output.push_str(" ");
                self.generate_expression(right)?;
                self.output.push_str(")");
            }
            IrExpression::UnaryOp { op, expr } => {
                self.output.push_str(unary_op_to_c(op));
                self.output.push_str("(");
                self.generate_expression(expr)?;
                self.output.push_str(")");
            }
            IrExpression::Call { callee, args } => {
                self.output.push_str(callee);
                self.output.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_expression(arg)?;
                }
                self.output.push_str(")");
            }
            IrExpression::Index { base, index } => {
                self.generate_expression(base)?;
                self.output.push_str("[");
                self.generate_expression(index)?;
                self.output.push_str("]");
            }
        }
        Ok(())
    }
    
    fn generate_value(&mut self, val: &IrValue) -> Result<(), TogError> {
        match val {
            IrValue::Int(n) => {
                self.output.push_str(&n.to_string());
            }
            IrValue::Float(n) => {
                self.output.push_str(&n.to_string());
            }
            IrValue::String(s) => {
                self.output.push_str("\"");
                self.output.push_str(&escape_string(s));
                self.output.push_str("\"");
            }
            IrValue::Bool(b) => {
                self.output.push_str(if *b { "true" } else { "false" });
            }
            IrValue::None => {
                self.output.push_str("NULL");
            }
            IrValue::Array(_) => {
                return Err(TogError::RuntimeError("Array literals in C codegen not yet implemented".to_string(), None));
            }
        }
        Ok(())
    }
    
    fn indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("    ");
        }
    }
}

fn type_to_c_type(ty: &crate::ast::Type) -> String {
    match ty {
        crate::ast::Type::Int => "int64_t".to_string(),
        crate::ast::Type::Float => "double".to_string(),
        crate::ast::Type::String => "char*".to_string(),
        crate::ast::Type::Bool => "bool".to_string(),
        crate::ast::Type::None => "void".to_string(),
        crate::ast::Type::Array(_) => "int64_t*".to_string(), // Simplified
        crate::ast::Type::Function { .. } => "void*".to_string(), // Function pointer
        crate::ast::Type::Infer => "int64_t".to_string(), // Default
        crate::ast::Type::Struct(_) => "void*".to_string(), // Placeholder for structs
        crate::ast::Type::Enum(_) => "int64_t".to_string(), // Enums as integers
    }
}

fn binary_op_to_c(op: &crate::ast::BinaryOp) -> &str {
    match op {
        crate::ast::BinaryOp::Add => "+",
        crate::ast::BinaryOp::Sub => "-",
        crate::ast::BinaryOp::Mul => "*",
        crate::ast::BinaryOp::Div => "/",
        crate::ast::BinaryOp::Mod => "%",
        crate::ast::BinaryOp::Eq => "==",
        crate::ast::BinaryOp::Ne => "!=",
        crate::ast::BinaryOp::Lt => "<",
        crate::ast::BinaryOp::Le => "<=",
        crate::ast::BinaryOp::Gt => ">",
        crate::ast::BinaryOp::Ge => ">=",
        crate::ast::BinaryOp::And => "&&",
        crate::ast::BinaryOp::Or => "||",
    }
}

fn unary_op_to_c(op: &crate::ast::UnaryOp) -> &str {
    match op {
        crate::ast::UnaryOp::Not => "!",
        crate::ast::UnaryOp::Neg => "-",
    }
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\t", "\\t")
}

