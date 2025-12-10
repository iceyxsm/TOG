# Pattern Matching in TOG

## Overview

TOG features a powerful pattern matching system inspired by Rust and functional languages, enabling safe and expressive handling of enums and data extraction.

## Features

### 1. Enum Definitions

Define custom types with multiple variants:

```tog
enum Status {
    Success,
    Failure,
    Pending
}

enum Result {
    Ok(int),
    Err(string)
}
```

### 2. Enum Construction

Create enum values with or without associated data:

```tog
let status = Status::Success
let result_ok = Result::Ok(42)
let result_err = Result::Err("Something went wrong")
```

### 3. Match Expressions

Pattern match on enum variants:

```tog
match status {
    Status::Success => print("Success!"),
    Status::Failure => print("Failure!"),
    Status::Pending => print("Pending..."),
    _ => print("Unknown")
}
```

### 4. Data Extraction

Extract and bind data from enum variants:

```tog
match result_ok {
    Result::Ok(value) => {
        print("Got value:")
        print(value)  // Prints: 42
    },
    Result::Err(msg) => {
        print("Got error:")
        print(msg)
    },
    _ => print("Unknown")
}
```

### 5. Wildcard Patterns

Catch-all pattern for exhaustive matching:

```tog
match x {
    _ => print("Matches anything")
}
```

## Pattern Types

### Literal Patterns
```tog
match x {
    42 => print("The answer"),
    0 => print("Zero"),
    _ => print("Something else")
}
```

### Variable Patterns
```tog
match x {
    value => print(value)  // Binds x to 'value'
}
```

### Wildcard Pattern
```tog
match x {
    _ => print("Ignored")  // Doesn't bind a variable
}
```

### Enum Variant Patterns
```tog
match result {
    Result::Ok(value) => print(value),      // With data extraction
    Result::Err(msg) => print(msg),         // With data extraction
    _ => print("Fallback")
}
```

## Implementation Details

### Parser

The parser distinguishes between:
- **Struct literals**: `Point { x: 1, y: 2 }`
- **Match expressions**: `match x { ... }`

This is achieved through sophisticated lookahead:
- Checks for `Identifier {`
- Looks ahead to see if followed by `Identifier :`
- Only parses as struct literal if the pattern matches

### Interpreter

Pattern matching is evaluated as follows:
1. Evaluate the match expression value
2. For each match arm:
   - Try to match the pattern against the value
   - If match succeeds, bind any extracted variables
   - Create a new environment with bindings
   - Evaluate the arm body in that environment
3. Return the result of the first matching arm

### Variable Binding

When a pattern like `Result::Ok(value)` matches:
1. The interpreter extracts the associated data from the enum
2. Creates a binding: `value => <extracted_data>`
3. Adds this binding to a temporary environment
4. Evaluates the match arm body with access to `value`

## Examples

### Basic Error Handling

```tog
enum Result {
    Ok(int),
    Err(string)
}

fn divide(a: int, b: int) -> Result {
    if b == 0 {
        return Result::Err("Division by zero")
    }
    return Result::Ok(a / b)
}

fn main() {
    let result = divide(10, 2)
    match result {
        Result::Ok(value) => print(value),
        Result::Err(msg) => print(msg),
        _ => print("Unknown")
    }
}
```

### Option Type

```tog
enum Option {
    Some(int),
    None
}

fn find_value(arr: [int], target: int) -> Option {
    // ... search logic ...
    return Option::Some(index)
}

fn main() {
    let result = find_value([1, 2, 3], 2)
    match result {
        Option::Some(index) => print(index),
        Option::None => print("Not found"),
        _ => print("Unknown")
    }
}
```

### State Machine

```tog
enum State {
    Idle,
    Running,
    Paused,
    Stopped
}

fn handle_state(state: State) {
    match state {
        State::Idle => print("Ready to start"),
        State::Running => print("Currently running"),
        State::Paused => print("Paused"),
        State::Stopped => print("Stopped"),
        _ => print("Unknown state")
    }
}
```

## Future Enhancements

- **Nested patterns**: `Result::Ok(Some(value))`
- **Guard clauses**: `Result::Ok(x) if x > 0 => ...`
- **Multiple patterns per arm**: `Status::Success | Status::Pending => ...`
- **Tuple patterns**: `(x, y) => ...`
- **Array patterns**: `[first, rest...] => ...`
- **Struct patterns**: `Point { x, y } => ...`

## Testing

Run the comprehensive pattern matching test:

```bash
cargo run --release -- run examples/pattern_matching_complete.tog
```

This demonstrates:
- ✅ Enum definitions
- ✅ Enum variant construction (with and without data)
- ✅ Pattern matching on enum variants
- ✅ Data extraction from enum variants
- ✅ Wildcard patterns
- ✅ Match expressions

## Performance

Pattern matching in TOG is:
- **Zero-cost**: Compiles to efficient machine code
- **Type-safe**: Checked at compile time (when type checker is enabled)
- **Exhaustive**: Ensures all cases are handled (future enhancement)

## See Also

- [Error Handling](ERROR_HANDLING.md) - Using Result and Option
- [Traits](TRAITS.md) - Implementing behavior for enums
- [Type System](../README.md#type-system) - Gradual typing with enums

