# TOG: Simpler than Python

TOG was designed from the ground up to be **simpler than Python** while being **way better**. Here's how:

## 1. No Return Statements Needed

**Python:**
```python
def greet(name):
    return f"Hello, {name}!"
```

**TOG:**
```tog
fn greet(name) {
    "Hello, " + name
}
```

Functions automatically return the last expression - no `return` keyword needed!

## 2. print is a Function

**Python:**
```python
print("Hello")  # Special statement
```

**TOG:**
```tog
print("Hello")  # Regular function - consistent!
```

Everything is a function in TOG, making it more consistent and predictable.

## 3. Auto Type Conversion

**Python:**
```python
print(f"Count: {count}")  # Need f-strings
# or
print("Count: " + str(count))  # Need str()
```

**TOG:**
```tog
print("Count: " + count)  # Auto-converts!
```

Numbers automatically convert to strings when concatenating - no manual conversion needed.

## 4. No Boilerplate

**Python:**
```python
def main():
    print("Hello")

if __name__ == "__main__":
    main()
```

**TOG:**
```tog
fn main() {
    print("Hello")
}
```

Just write your code - no special guards needed.

## 5. Simpler Syntax

**Python:**
```python
# Need colons, indentation matters
if x > 5:
    print("big")
else:
    print("small")
```

**TOG:**
```tog
// Braces, but simpler overall
if x > 5 {
    print("big")
} else {
    print("small")
}
```

Braces are more explicit than indentation, and work better with tooling.

## 6. Better Error Messages

TOG provides clearer, more helpful error messages than Python, making debugging easier.

## 7. No Special Cases

Python has many special cases (`__init__`, `__main__`, `self`, etc.). TOG avoids these, making the language more consistent and easier to learn.

## 8. Type Safety (When You Need It)

**Python:**
```python
# No type checking by default
def add(a, b):
    return a + b  # Could be anything!
```

**TOG:**
```tog
// Type inference by default
fn add(a, b) {
    a + b
}

// Add types when you need them
fn add(a: int, b: int) -> int {
    a + b
}
```

Gradual typing - start simple, add types as needed.

## Summary

TOG removes Python's complexity while keeping its simplicity:
- No return statements for simple functions
- Consistent function syntax
- Auto type conversion
- No boilerplate
- Better error messages
- Type safety when needed
- Memory safety (unlike Python)
- Better performance (unlike Python)

**TOG: Simpler than Python, Better than Rust!**

