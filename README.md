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

## Why TOG

TOG was designed to combine the best features of modern programming languages while maintaining simplicity and performance.

### Key Simplifications

#### 1. Auto-Return Functions
No need for explicit `return` in single-expression functions:

```tog
// TOG - Simple!
fn greet(name) {
    "Hello, " + name
}
```

#### 2. print is a Function
Consistent with everything else - no special statements:

```tog
// TOG - Everything is a function
print("Hello")
print(greet("World"))
```

#### 3. Auto Type Conversion
Numbers automatically convert to strings - no manual conversion:

```tog
// TOG - Just works!
print("Count: " + 42)
print("Pi: " + 3.14)
```

#### 4. No Boilerplate
Just write your code - no special guards:

```tog
// TOG - Clean and simple
fn main() {
    print("Hello")
}
```

#### 5. Simpler Ownership
TOG's ownership model is intuitive and easy to understand. The compiler handles memory management automatically - no lifetime annotations needed.

#### 6. Gradual Typing
Start without types, add them as needed for optimization. Type inference works everywhere.

### Real-World Example

**Task**: Create a function that greets a user and prints their age.

**TOG:**
```tog
fn greet_user(name, age) {
    let message = "Hello, " + name + "! You are " + age + " years old."
    print(message)
    message
}

fn main() {
    greet_user("Alice", 30)
}
```

**TOG features:**
- Minimal boilerplate
- Simple syntax with auto-conversion
- Consistent design (everything is a function)
- Memory safety
- High performance through compilation

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

## Installation

```bash
# Build from source
cargo build --release

# The binary will be at: target/release/tog.exe (Windows) or target/release/tog (Unix)
```

## Performance

TOG is designed for high performance:

- **JIT compilation** - Fast development iteration
- **AOT compilation** - Maximum production performance
- **SIMD support** - Automatic CPU vectorization
- **Memory safety** - Without performance penalties

See [docs/performance.md](docs/performance.md) for detailed architecture.

## Documentation

- [Quick Start Guide](QUICKSTART.md)
- [Full Documentation](docs/)
- [Implementation Status](IMPLEMENTATION_STATUS.md)

## Examples

See [examples/](examples/) for example programs.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License
