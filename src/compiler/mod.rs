// TOG Compiler - Multi-backend compilation system
// 
// Architecture:
// 1. AST → IR (Intermediate Representation)
// 2. IR → Optimized IR (optimization passes)
// 3. IR → Backend-specific code (LLVM, Cranelift, JIT, etc.)

pub mod backend;
pub mod ir;
pub mod optimizer;
pub mod codegen;
pub mod native_gen;
pub mod loop_analysis;

use crate::ast::Program;
use crate::error::TogError;
use backend::{Backend, BackendType};
use optimizer::OptimizationLevel;

pub struct Compiler {
    backend: Box<dyn Backend>,
    opt_level: OptimizationLevel,
}

impl Compiler {
    pub fn new(backend_type: BackendType, opt_level: OptimizationLevel) -> Result<Self, TogError> {
        let backend = backend::create_backend(backend_type, opt_level)?;
        Ok(Self {
            backend,
            opt_level,
        })
    }
    
    pub fn compile(&mut self, program: Program) -> Result<Vec<u8>, TogError> {
        // Step 1: Convert AST to IR
        let mut ir = ir::ast_to_ir(program)?;
        
        // Step 2: Optimize IR
        optimizer::optimize(&mut ir, self.opt_level)?;
        
        // Step 3: Generate code using backend
        self.backend.generate_code(&ir)
    }
    
    pub fn compile_to_file(&mut self, program: Program, output_path: &std::path::Path) -> Result<(), TogError> {
        let code = self.compile(program)?;
        std::fs::write(output_path, code)
            .map_err(|e| TogError::IoError(format!("Failed to write output: {}", e)))?;
        Ok(())
    }
}

