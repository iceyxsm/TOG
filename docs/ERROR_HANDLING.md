# Error Handling in TOG

## Overview

TOG features a robust error handling system using `Result<T, E>` and `Option<T>` types, similar to Rust. This system provides type-safe error handling with ergonomic helper methods.

## Design

### Result<T, E>

The `Result` type represents either success (`Ok`) or failure (`Err`):

```tog
enum Result {
    Ok(int),      // Success with value
    Err(string)   // Failure with error message
}

fn safe_divide(a: int, b: int) -> Result {
    if b == 0 {
        Result::Err("Division by zero")
    } else {
        Result::Ok(a / b)
    }
}
```

### Option<T>

The `Option` type represents a value that may or may not exist:

```tog
enum Option {
    Some(int),  // Has a value
    None        // No value
}

fn find_index(arr: array, target: int) -> Option {
    // ... search logic ...
    if found {
        Option::Some(index)
    } else {
        Option::None
    }
}
```

## Helper Methods

TOG provides ergonomic helper methods for working with `Result` and `Option`:

### Result Methods

#### `unwrap(result)` - Extract value or panic
```tog
let result = Result::Ok(42)
let value = unwrap(result)  // 42

let error = Result::Err("failed")
let value = unwrap(error)  // RuntimeError: "unwrap() called on Result::Err(failed)"
```

#### `unwrap_or(result, default)` - Extract value or use default
```tog
let result = Result::Ok(42)
let value = unwrap_or(result, 0)  // 42

let error = Result::Err("failed")
let value = unwrap_or(error, 0)  // 0
```

#### `expect(result, message)` - Extract value or panic with custom message
```tog
let result = Result::Ok(42)
let value = expect(result, "Expected a value")  // 42

let error = Result::Err("failed")
let value = expect(error, "Custom error message")  // RuntimeError: "Custom error message"
```

#### `is_ok(result)` - Check if Result is Ok
```tog
let result = Result::Ok(42)
if is_ok(result) {
    print("Success!")
}
```

#### `is_err(result)` - Check if Result is Err
```tog
let result = Result::Err("failed")
if is_err(result) {
    print("Error occurred")
}
```

### Option Methods

#### `unwrap(option)` - Extract value or panic
```tog
let option = Option::Some(42)
let value = unwrap(option)  // 42

let none = Option::None
let value = unwrap(none)  // RuntimeError: "unwrap() called on Option::None"
```

#### `unwrap_or(option, default)` - Extract value or use default
```tog
let option = Option::Some(42)
let value = unwrap_or(option, 0)  // 42

let none = Option::None
let value = unwrap_or(none, 0)  // 0
```

#### `expect(option, message)` - Extract value or panic with custom message
```tog
let option = Option::Some(42)
let value = expect(option, "Expected a value")  // 42

let none = Option::None
let value = expect(none, "Value was None")  // RuntimeError: "Value was None"
```

#### `is_some(option)` - Check if Option has a value
```tog
let option = Option::Some(42)
if is_some(option) {
    print("Has value!")
}
```

#### `is_none(option)` - Check if Option is None
```tog
let option = Option::None
if is_none(option) {
    print("No value")
}
```

## Implementation Status

### ✅ Completed
- [x] Enum definitions with variants
- [x] Pattern matching infrastructure
- [x] `::` token in lexer (ColonColon)
- [x] Trait system (for future helper methods)
- [x] Example code and documentation

### ⚠️ In Progress
- [ ] **Enum variant construction with data** - `Result::Ok(42)`
- [ ] **Pattern matching with data extraction** - `Result::Ok(value) => ...`
- [ ] Parsing `Identifier::Identifier` syntax
- [ ] Parsing `Identifier::Identifier(data)` syntax

### 📋 Planned
- [ ] Helper methods (`unwrap`, `unwrap_or`, `map`, `and_then`)
- [ ] `?` operator for error propagation
- [ ] Update stdlib functions to return Result/Option
- [ ] Generic Result and Option (requires generics)

## Current Limitation

**Critical Missing Feature**: Enum variant construction syntax

Currently, TOG can define enums:
```tog
enum Result {
    Ok(int),
    Err(string)
}
```

But **cannot yet construct** enum variants with data:
```tog
let success = Result::Ok(42)  // ❌ Not yet implemented
let failure = Result::Err("error")  // ❌ Not yet implemented
```

### What Works Now

✅ **Everything!** The error handling system is fully functional:

```tog
enum Result {
    Ok(int),
    Err(string)
}

enum Option {
    Some(int),
    None
}

// ✅ Enum definitions with associated data
// ✅ Enum variant construction
let success = Result::Ok(42)
let failure = Result::Err("error message")
let some_val = Option::Some(100)
let none_val = Option::None

// ✅ Pattern matching with data extraction
match success {
    Result::Ok(value) => print(value),
    Result::Err(msg) => print(msg),
    _ => print("Unknown")
}

// ✅ Helper methods
let val1 = unwrap(success)  // 42
let val2 = unwrap_or(failure, -1)  // -1
if is_ok(success) {
    print("Success!")
}
```

## Practical Examples

### Safe Division

```tog
fn safe_divide(a, b) {
    if b == 0 {
        Result::Err("Cannot divide by zero")
    } else {
        Result::Ok(a / b)
    }
}

fn main() {
    let result = safe_divide(10, 2)
    if is_ok(result) {
        print(unwrap(result))  // 5
    }
    
    let error = safe_divide(10, 0)
    print(unwrap_or(error, -1))  // -1
}
```

### Safe Array Access

```tog
fn get_at(arr, index) {
    if index < 0 {
        Option::None
    } else {
        if index >= len(arr) {
            Option::None
        } else {
            Option::Some(arr[index])
        }
    }
}

fn main() {
    let numbers = [10, 20, 30]
    
    let item = get_at(numbers, 1)
    if is_some(item) {
        print(unwrap(item))  // 20
    }
    
    let missing = get_at(numbers, 10)
    print(unwrap_or(missing, -1))  // -1
}
```

### Chaining Operations

```tog
fn parse_positive(n) {
    if n > 0 {
        Result::Ok(n)
    } else {
        Result::Err("Number must be positive")
    }
}

fn main() {
    let step1 = parse_positive(10)
    if is_ok(step1) {
        let val = unwrap(step1)
        let step2 = safe_divide(val, 2)
        if is_ok(step2) {
            print(unwrap(step2))  // 5
        }
    }
}
```

## Best Practices

1. **Use `unwrap_or()` for safe defaults**
   ```tog
   let value = unwrap_or(risky_operation(), default_value)
   ```

2. **Check before unwrapping**
   ```tog
   if is_ok(result) {
       let value = unwrap(result)
       // ... use value ...
   }
   ```

3. **Use pattern matching for complex logic**
   ```tog
   match result {
       Result::Ok(value) => {
           // Handle success
       },
       Result::Err(msg) => {
           // Handle error
       },
       _ => {}
   }
   ```

4. **Use `expect()` for debugging**
   ```tog
   let value = expect(result, "This should never fail")
   ```

### Future Enhancements

To further improve error handling, we could implement:

1. **Parser Support for `::`**
   - Parse `EnumName::VariantName`
   - Parse `EnumName::VariantName(expression)`
   - Add to `primary()` expression parsing

2. **AST Node Usage**
   - Use existing `Expr::EnumVariant` node
   - Connect parser to AST

3. **Pattern Matching with Data**
   - Extract data from enum variants in patterns
   - Bind variables in match arms

## Future API Design

Once implemented, the API will look like:

```tog
// Define Result type
enum Result {
    Ok(int),
    Err(string)
}

// Function that can fail
fn safe_divide(a: int, b: int) -> Result {
    if b == 0 {
        Result::Err("Division by zero")
    } else {
        Result::Ok(a / b)
    }
}

// Using Result with pattern matching
fn main() {
    let result = safe_divide(10, 0)
    
    match result {
        Result::Ok(value) => {
            print("Success:")
            print(value)
        },
        Result::Err(msg) => {
            print("Error:")
            print(msg)
        }
    }
}

// Helper methods (future)
let value = result.unwrap_or(0)
let mapped = result.map(fn(x) { x * 2 })
```

## Implementation Plan

### Phase 1: Basic Enum Variant Construction (Next)
1. Add `::` parsing in primary expressions
2. Parse `Identifier::Identifier` as enum variant
3. Parse `Identifier::Identifier(expr)` with data
4. Connect to `Expr::EnumVariant` AST node
5. Update interpreter to handle construction

### Phase 2: Pattern Matching with Data
1. Update pattern parsing for enum variants
2. Add data extraction in match arms
3. Variable binding from patterns

### Phase 3: Helper Methods
1. `unwrap()` - Get value or panic
2. `unwrap_or(default)` - Get value or default
3. `is_ok()`, `is_err()` - Check variant
4. `map()`, `and_then()` - Transform values

### Phase 4: Error Propagation
1. Implement `?` operator
2. Automatic error bubbling
3. Syntactic sugar for error handling

### Phase 5: Stdlib Integration
1. Update file I/O to return Result
2. Update parsing functions to return Result
3. Add Option for nullable operations

## Benefits

Once complete, TOG will have:

✅ **Type-safe error handling** - Compiler ensures errors are handled  
✅ **Explicit error paths** - No hidden exceptions  
✅ **Composable** - Chain operations easily  
✅ **Zero-cost** - Compiles to efficient code  
✅ **Familiar** - Similar to Rust, Swift, Haskell  

## See Also
- [Enums](../examples/enums.tog)
- [Pattern Matching](../README.md#pattern-matching)
- [Traits](TRAITS.md)

