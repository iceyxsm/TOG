# TOG Programming Language

**TOG** - The Optimal Language: A modern programming language combining performance, safety, and simplicity.

## Philosophy

TOG aims to provide:
- **High Performance** - JIT + AOT compilation with SIMD/GPU support
- **Memory Safety** - Simpler ownership model, better ergonomics, fast compile times
- **Power & Control** - Memory safety, zero-cost abstractions, modern features
- **Simplicity** - Clean syntax, auto-return, minimal boilerplate

## Key Features

### Performance
- **Multi-tier compilation** - JIT for development, AOT for production
- **SIMD/vectorization** - Automatic CPU vectorization
- **GPU compute support** - CUDA/OpenCL for parallel workloads
- **Profile-guided optimization** - Learn from runtime behavior
- **Zero-cost abstractions** - Compile away high-level features

### Safety & Ergonomics
- **Simpler ownership** - Automatic memory management without GC
- **Gradual typing** - Start dynamic, add types as needed
- **Clear error messages** - Actionable diagnostics
- **Fast compile times** - Incremental compilation
- **Hot reload** - Development with instant feedback

### Power & Control
- **Memory safety** - No segfaults, no use-after-free
- **Modern abstractions** - Without performance cost
- **Inline assembly** - When you need low-level control
- **Custom allocators** - Arena, pool, stack allocators
- **Zero-overhead abstractions** - High-level code, low-level performance

## Quick Start

```tog
// Hello World
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

## Why TOG

1. **No return needed** - Functions auto-return the last expression
2. **print is a function** - Consistent with everything else
3. **Auto type conversion** - Numbers auto-convert to strings when needed
4. **Less boilerplate** - Minimal setup required
5. **Cleaner syntax** - Fewer special cases and exceptions
6. **Intuitive defaults** - Everything just works

## Performance

TOG is designed for high performance:

- **JIT compilation** - Fast development iteration
- **AOT compilation** - Maximum production performance
- **SIMD support** - Automatic CPU vectorization
- **Memory safety** - Without performance penalties

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

