# TOG Programming Language

**TOG** - The Optimal Language: Faster than Numba, Better than Rust, More Powerful than C.

## Philosophy

TOG aims to be:
- **Faster than Numba** - JIT + AOT compilation with SIMD/GPU support
- **Better than Rust** - Simpler ownership, better ergonomics, faster compile times
- **More powerful than C** - Memory safety, zero-cost abstractions, modern features
- **Simpler than Python** - Clean syntax, auto-return, no boilerplate

## Key Features

### Performance (Faster than Numba)
- **Multi-tier compilation** - JIT for development, AOT for production
- **SIMD/vectorization** - Automatic CPU vectorization
- **GPU compute support** - CUDA/OpenCL for parallel workloads
- **Profile-guided optimization** - Learn from runtime behavior
- **Zero-cost abstractions** - Compile away high-level features

### Safety & Ergonomics (Better than Rust)
- **Simpler ownership** - Automatic memory management without GC
- **Gradual typing** - Start dynamic, add types as needed
- **Better error messages** - Clear, actionable diagnostics
- **Faster compile times** - Incremental compilation
- **Hot reload** - Development with instant feedback

### Power (More powerful than C)
- **Memory safety** - No segfaults, no use-after-free
- **Modern abstractions** - Without performance cost
- **Inline assembly** - When you need C-level control
- **Custom allocators** - Arena, pool, stack allocators
- **Zero-overhead abstractions** - High-level code, C-level performance

## Quick Start

```tog
// Hello World - Simpler than Python!
fn main() {
    print("Hello, TOG!")
}

// Variables - type inference by default
let name = "TOG"
let age = 2025
let pi = 3.14159

// Functions - no return needed for single expressions!
fn greet(name) {
    "Hello, " + name
}

fn add(a, b) {
    a + b
}

// print is a function (not a statement)
print(greet("World"))
print("Sum: " + add(10, 20))  // Auto-converts numbers to strings!

// Pattern matching
match value {
    1 => print("One"),
    2 => print("Two"),
    _ => print("Other")
}
```

## Why TOG is Simpler than Python

1. **No return needed** - Functions auto-return the last expression
2. **print is a function** - Consistent with everything else
3. **Auto type conversion** - Numbers auto-convert to strings when needed
4. **Less boilerplate** - No `if __name__ == "__main__"` needed
5. **Cleaner syntax** - Fewer special cases and exceptions
6. **Better defaults** - Everything just works intuitively

## Performance Targets

TOG is designed to outperform:

- **Numba**: Better JIT compilation with automatic SIMD
- **Rust**: Faster compile times, simpler ownership
- **C**: Memory safety with equal or better performance

See [docs/performance.md](docs/performance.md) for detailed architecture.

## Installation

```bash
# Build from source
cargo build --release

# Or use the installer
./install.sh
```

## Documentation

See [docs/](docs/) for full documentation.

## Examples

See [examples/](examples/) for example programs.

## Contributing

Contributions welcome! See CONTRIBUTING.md for guidelines.

## License

MIT License

