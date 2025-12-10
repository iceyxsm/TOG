# TOG Quick Start Guide

## Prerequisites

Install Rust from [rustup.rs](https://rustup.rs/)

## Building TOG

```bash
# Build the TOG compiler/interpreter
cargo build --release

# The binary will be at: target/release/tog.exe (Windows) or target/release/tog (Unix)
```

## Running Your First Program

1. Create a file `hello.tog`:
```tog
fn main() {
    print("Hello, TOG!")
}
```

2. Run it:
```bash
cargo run -- run hello.tog
# Or if you've installed:
tog run hello.tog
```

## Example Programs

Check out the `examples/` directory for more examples:
- `hello.tog` - Hello World
- `variables.tog` - Variables and types
- `functions.tog` - Functions
- `control_flow.tog` - If/else, loops, pattern matching
- `arrays.tog` - Arrays

## Commands

- `tog run <file>` - Run a TOG program
- `tog check <file>` - Check syntax without running
- `tog build <file>` - Build a TOG program (compiler backend coming soon)
- `tog fmt <file>` - Format a TOG file (formatter coming soon)

## Language Features

TOG combines the best of Python and Rust:

- **Python-like syntax** - Clean, readable code
- **Rust-like safety** - Memory safety without GC
- **Type inference** - Start simple, add types as needed
- **Pattern matching** - Powerful control flow
- **Zero-cost abstractions** - Performance when you need it

## Next Steps

- Read [docs/syntax.md](docs/syntax.md) for full syntax reference
- Check out [examples/](examples/) for more examples
- See [CONTRIBUTING.md](CONTRIBUTING.md) to contribute

