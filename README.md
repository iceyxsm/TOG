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
- **GPU compute support** - Automatic GPU acceleration for numeric operations
- **Parallel processing** - Multi-threaded execution with batch processing
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

// GPU and Parallel Processing - Automatic acceleration!
fn process_large_dataset() {
    let data = range(1, 1000000)
    
    // Automatically uses GPU if available
    let sum = gpu_sum(data)
    let mean = gpu_mean(data)
    
    // Parallel processing across CPU cores
    let parallel_result = parallel_sum(data)
    
    print("Sum: " + sum)
    print("Mean: " + mean)
}

// print is a function (not a statement)
print(greet("World"))
print("Sum: " + add(10, 20))  // Auto-converts numbers to strings!

// Error handling with Result/Option
enum Result {
    Ok(int),
    Err(string)
}

fn safe_divide(a, b) {
    if b == 0 {
        Result::Err("Division by zero")
    } else {
        Result::Ok(a / b)
    }
}

let result = safe_divide(10, 2)
let value = unwrap_or(result, 0)  // 5

// Pattern matching with data extraction
match result {
    Result::Ok(value) => print(value),
    Result::Err(msg) => print(msg),
    _ => print("Unknown")
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
- **GPU acceleration** - Automatic offloading of numeric operations
- **Parallel processing** - Multi-threaded execution with batch processing
- **Memory safety** - Without performance penalties

### GPU and Parallel Processing

TOG automatically accelerates numeric operations using GPU and parallel processing:

```tog
// Automatic GPU acceleration for large arrays
let data = range(1, 1000000)
let sum = gpu_sum(data)        // 10-100x faster on GPU
let mean = gpu_mean(data)      // Automatic GPU dispatch
let product = gpu_product(data) // Parallel reduction

// Multi-threaded parallel processing
let result = parallel_sum(data) // Uses all CPU cores

// Batch processing for cache optimization
let batch = batch_size()        // Get optimal batch size
```

**Performance Benefits:**
- **GPU operations**: 10-100x speedup for large numeric arrays
- **Parallel processing**: 2-8x speedup on multi-core CPUs
- **Batch processing**: Improved cache locality and memory bandwidth
- **Automatic dispatch**: No manual optimization needed

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
