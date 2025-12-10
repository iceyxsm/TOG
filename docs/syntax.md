# TOG Language Syntax

## Overview

TOG combines the readability of Python with the safety and performance of Rust.

## Comments

```tog
// Single-line comments start with //
```

## Variables

```tog
// Type inference
let name = "TOG"
let age = 25

// Explicit types
let count: int = 42
let price: float = 99.99
let message: string = "Hello"
```

## Functions

TOG functions are simpler than Python - no return needed for single expressions!

```tog
// Simple function - auto-returns last expression
fn greet(name) {
    "Hello, " + name
}

// With explicit return (optional)
fn greet_explicit(name) {
    return "Hello, " + name
}

// Function with types (optional)
fn add(a: int, b: int) -> int {
    a + b  // No return needed!
}

// Function without parameters
fn main() {
    print("Hello, World!")
    print(greet("TOG"))  // Calls the function
}
```

**Key Simplifications:**
- No `return` keyword needed for single expressions
- Types are optional (gradual typing)
- `print` is a function, not a special statement

## Control Flow

### If-Else

```tog
if condition {
    // code
} else {
    // code
}
```

### While Loops

```tog
while condition {
    // code
}
```

### Pattern Matching

```tog
match value {
    1 => print("One"),
    2 => print("Two"),
    _ => print("Other")
}
```

## Operators

### Arithmetic
- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `%` Modulo

### Comparison
- `==` Equal
- `!=` Not equal
- `<` Less than
- `<=` Less than or equal
- `>` Greater than
- `>=` Greater than or equal

### Logical
- `&&` And
- `||` Or
- `!` Not

## Types

- `int` - 64-bit integers
- `float` - 64-bit floating point
- `string` - UTF-8 strings
- `bool` - Boolean (true/false)
- `array[T]` - Arrays of type T
- `none` - Null/none value

## Arrays

```tog
let numbers = [1, 2, 3, 4, 5]
let names = ["Alice", "Bob"]
```

## Built-in Functions

- `print(value)` - Print a value to stdout

