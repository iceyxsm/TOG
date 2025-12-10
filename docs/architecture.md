# TOG Compiler Architecture

## Overview

TOG uses a multi-stage compilation pipeline designed for maximum performance and flexibility.

## Compilation Pipeline

```
Source Code (.tog)
    ↓
[Lexer] → Tokens
    ↓
[Parser] → AST (Abstract Syntax Tree)
    ↓
[Type Checker] → Typed AST (with type information)
    ↓
[IR Generator] → IR (Intermediate Representation)
    ↓
[Optimizer] → Optimized IR
    ↓
[Backend] → Native Code / Bytecode
```

## Design Decisions

### 1. Intermediate Representation (IR)

**Why IR?**
- Backend-agnostic: Same IR can target LLVM, Cranelift, JIT, or GPU
- Optimization-friendly: Easier to optimize than AST
- Type information: Preserves type information for better optimizations

**IR Structure**:
- Functions with typed parameters
- Statements and expressions
- Type annotations preserved
- Simplified control flow

### 2. Multi-Backend System

**Why multiple backends?**
- **LLVM**: Maximum optimization for production
- **Cranelift**: Fast compilation for development
- **JIT**: Runtime compilation for interactive use
- **GPU**: Parallel compute workloads

**Backend Trait**:
```rust
trait Backend {
    fn generate_code(&self, ir: &IrProgram) -> Result<Vec<u8>>;
    fn supports_optimization(&self) -> bool;
}
```

### 3. Optimization Levels

**Levels**:
- `-O0`: No optimization (fastest compile)
- `-O1`: Basic (constant folding)
- `-O2`: Standard (default, includes inlining)
- `-O3`: Aggressive (all optimizations)
- `-Os`: Size optimization

**Optimization Passes**:
1. Constant folding
2. Dead code elimination
3. Function inlining
4. Loop optimizations
5. Memory optimizations

### 4. Type System for Optimization

**Gradual Typing**:
- Start without types (dynamic)
- Add types for optimization
- Type inference helps optimizer

**Type Information in IR**:
- Preserves type annotations
- Infers types when missing
- Enables better optimizations

## Performance Considerations

### Compile-Time Performance

1. **Incremental Compilation**: Only recompile changed modules
2. **Fast Backend Option**: Use Cranelift for development
3. **Lazy Optimization**: Skip optimizations at -O0

### Runtime Performance

1. **Zero-Cost Abstractions**: High-level code → efficient low-level code
2. **SIMD**: Automatic vectorization (planned)
3. **Profile-Guided**: Use runtime data for optimization (planned)

## Future Enhancements

### Phase 1: Foundation (Current)
- [x] IR generation
- [x] Basic optimizations
- [x] Backend architecture

### Phase 2: Backend Implementation
- [ ] LLVM integration
- [ ] Cranelift integration
- [ ] JIT compiler

### Phase 3: Advanced Optimizations
- [ ] SIMD/vectorization
- [ ] Profile-guided optimization
- [ ] Link-time optimization

### Phase 4: GPU Support
- [ ] CUDA backend
- [ ] OpenCL backend
- [ ] GPU kernel generation

## Reasoning

### Why not compile directly to LLVM IR?

- **Flexibility**: Can switch backends without changing frontend
- **Optimization**: Can optimize before backend-specific codegen
- **Testing**: Easier to test optimizations on IR

### Why multiple optimization levels?

- **Development**: Fast iteration with -O0
- **Production**: Maximum performance with -O3
- **Embedded**: Size optimization with -Os

### Why preserve type information?

- **Optimization**: Type-aware optimizations are better
- **Error Detection**: Catch type errors early
- **Code Generation**: Better codegen with type info

## Conclusion

This architecture provides:
- **Flexibility**: Multiple backends for different use cases
- **Performance**: Optimizations at multiple levels
- **Extensibility**: Easy to add new backends/optimizations
- **Simplicity**: Clean separation of concerns

