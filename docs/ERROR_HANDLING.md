# Error Handling in TOG (In Progress)

## Overview

TOG is implementing a robust error handling system using `Result<T, E>` and `Option<T>` types, similar to Rust. This document describes the design and current implementation status.

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

Simple enum variants without data:
```tog
enum Status {
    Success,
    Failure,
    Pending
}

// This works:
let status = Status::Success  // ✅ Simple variant (no parser support yet)
```

### What's Needed

To complete error handling, we need to implement:

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

