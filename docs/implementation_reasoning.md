# Implementation Reasoning and Design Decisions

This document explains the reasoning behind key implementation decisions in TOG.

## 1. Intermediate Representation (IR)

**Decision**: Create a backend-agnostic IR instead of compiling directly to LLVM IR.

**Reasoning**:
- **Flexibility**: Can switch backends (LLVM, Cranelift, JIT, GPU) without changing frontend
- **Optimization**: Easier to optimize IR than AST or backend-specific IR
- **Testing**: Can test optimizations independently of backend
- **Incremental**: Can add backends gradually without rewriting everything

**Trade-offs**:
- Extra conversion step (AST → IR → Backend IR)
- But: Enables better architecture and future flexibility

## 2. Multi-Backend System

**Decision**: Support multiple compilation backends with a trait-based architecture.

**Reasoning**:
- **LLVM**: Maximum optimization for production (beats C)
- **Cranelift**: Fast compilation for development (beats Rust compile times)
- **JIT**: Runtime compilation for interactive use (beats Numba iteration speed)
- **Native C**: Simple codegen for testing and as stepping stone

**Implementation**:
```rust
trait Backend {
    fn generate_code(&self, ir: &IrProgram) -> Result<Vec<u8>>;
}
```

**Benefits**:
- Choose backend based on use case
- Easy to add new backends
- Can fallback if one backend fails

## 3. Optimization Pipeline

**Decision**: Implement optimizations in stages with different levels.

**Reasoning**:
- **-O0**: Fast compilation for development
- **-O1**: Basic optimizations (constant folding)
- **-O2**: Standard optimizations (default, includes inlining)
- **-O3**: Aggressive optimizations (all passes)
- **-Os**: Size optimization for embedded

**Optimization Order**:
1. Constant folding (always safe, enables other optimizations)
2. Dead code elimination (removes unreachable code)
3. Function inlining (enables more optimizations)
4. Loop optimizations (SIMD, unrolling, fusion)

**Why this order?**
- Constant folding creates more dead code
- Dead code elimination cleans up after inlining
- Inlining enables more constant propagation
- Loop optimizations benefit from previous optimizations

## 4. Dead Code Elimination

**Decision**: Remove unreachable code and unused functions.

**Reasoning**:
- **Binary size**: Smaller binaries = better cache usage
- **Compilation**: Less code to optimize = faster compilation
- **Performance**: Removes unnecessary code from hot paths

**Implementation Strategy**:
- Remove code after return statements
- Remove unused functions (not called anywhere)
- Keep public functions (might be called externally)

**Trade-offs**:
- Need to be careful with side effects
- But: Safe for pure functions and most code

## 5. Function Inlining

**Decision**: Inline small functions (< 10 statements) automatically.

**Reasoning**:
- **Performance**: Eliminates function call overhead
- **Optimization**: Enables better constant propagation
- **SIMD**: Inlined code is easier to vectorize

**Heuristics**:
- Size limit: < 10 statements
- Not recursive: Avoid infinite inlining
- Iterative: Inline in multiple passes

**Why iterative?**
- Inlining creates more opportunities for inlining
- But limit to 3 iterations to avoid code bloat

## 6. Native C Code Generator

**Decision**: Create a C code generator before full LLVM integration.

**Reasoning**:
- **Testing**: Can test optimizations immediately
- **Stepping stone**: Easier than LLVM, teaches codegen
- **Useful**: Generated C can be compiled with GCC/Clang
- **Debugging**: C code is easier to read than LLVM IR

**Limitations**:
- Not as optimized as LLVM
- But: Good enough for testing and development

**Future**: Will be replaced by LLVM backend, but useful now.

## 7. Loop Analysis for SIMD

**Decision**: Analyze loops to detect vectorization opportunities.

**Reasoning**:
- **Performance**: SIMD can give 4-8x speedup
- **Automatic**: No manual annotations needed (unlike Rust/C)
- **Foundation**: First step toward actual SIMD code generation

**Detection Criteria**:
- Countable loops (known bounds)
- Simple operations (add, mul, etc.)
- No dependencies between iterations
- Contiguous memory access

**Future Steps**:
1. ✅ Loop detection (done)
2. ⏳ SIMD code generation (planned)
3. ⏳ Alignment handling (planned)
4. ⏳ Remainder loop generation (planned)

## 8. Type Propagation

**Decision**: Improve type inference in IR for better optimizations.

**Reasoning**:
- **Optimization**: Type-aware optimizations are better
- **SIMD**: Need to know if operations are numeric
- **Codegen**: Can use specialized instructions

**Implementation**:
- Propagate types through expressions
- Handle type promotion (int + float = float)
- Infer types from context

**Benefits**:
- Better constant folding (know types)
- Better SIMD detection (know if numeric)
- Better code generation (use specialized ops)

## 9. Constant Folding

**Decision**: Evaluate constant expressions at compile time.

**Reasoning**:
- **Performance**: No runtime cost for constant expressions
- **Size**: Smaller code (no need to store constants)
- **Enables**: Creates more dead code elimination opportunities

**Examples**:
- `2 + 3` → `5`
- `true && false` → `false`
- `10 * 0` → `0`

**Safety**:
- Only fold if both operands are constants
- Check for division by zero at compile time
- Preserve side effects (don't fold function calls)

## 10. Architecture Decisions Summary

| Decision | Reasoning | Trade-off |
|----------|-----------|-----------|
| IR-based | Flexibility, optimization | Extra conversion step |
| Multi-backend | Choose best tool for job | More code to maintain |
| Optimization levels | Development vs production | More complexity |
| Dead code elimination | Smaller binaries | Need careful analysis |
| Function inlining | Performance, optimization | Code bloat risk |
| Native C generator | Testing, stepping stone | Less optimal than LLVM |
| Loop analysis | SIMD foundation | Analysis overhead |
| Type propagation | Better optimizations | More complex inference |

## Conclusion

All decisions prioritize:
1. **Performance**: Faster than Numba, better than Rust, equal to C
2. **Simplicity**: Easier than Rust, simpler than Python
3. **Flexibility**: Multiple backends, gradual implementation
4. **Extensibility**: Easy to add new optimizations/backends

The architecture is designed to evolve: start simple, add complexity only when needed.

