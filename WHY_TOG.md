# Why TOG is Simpler and Better

## Mission: Simpler than Python, Better than Rust

TOG was designed to eliminate the complexity of both Python and Rust while keeping their best features.

## Key Simplifications

### 1. **Auto-Return Functions**
No need for explicit `return` in single-expression functions:

```tog
// TOG - Simple!
fn greet(name) {
    "Hello, " + name
}

// Python - More verbose
def greet(name):
    return f"Hello, {name}!"
```

### 2. **print is a Function**
Consistent with everything else - no special statements:

```tog
// TOG - Everything is a function
print("Hello")
print(greet("World"))

// Python - Special statement
print("Hello")
print(greet("World"))  # Same, but inconsistent
```

### 3. **Auto Type Conversion**
Numbers automatically convert to strings - no manual conversion:

```tog
// TOG - Just works!
print("Count: " + 42)
print("Pi: " + 3.14)

// Python - Need f-strings or str()
print(f"Count: {42}")
print("Count: " + str(42))
```

### 4. **No Boilerplate**
Just write your code - no special guards:

```tog
// TOG - Clean and simple
fn main() {
    print("Hello")
}

// Python - Boilerplate needed
def main():
    print("Hello")

if __name__ == "__main__":
    main()
```

### 5. **Simpler Syntax**
Fewer special cases and exceptions:

```tog
// TOG - Explicit and clear
if x > 5 {
    print("big")
}

// Python - Indentation matters (can be error-prone)
if x > 5:
    print("big")
```

## Better Than Rust

### 1. **Simpler Ownership**
Rust's borrow checker is complex. TOG's ownership is intuitive.

### 2. **No Lifetime Annotations**
No need to understand lifetimes - the compiler handles it.

### 3. **Better Error Messages**
Clear, helpful errors that guide you to the solution.

### 4. **Gradual Typing**
Start without types, add them as needed.

## Comparison Table

| Feature | Python | Rust | TOG |
|---------|--------|------|-----|
| Auto-return | No | No | Yes |
| print as function | No | Yes | Yes |
| Auto type conversion | No | No | Yes |
| No boilerplate | No | Yes | Yes |
| Memory safety | No | Yes | Yes |
| Type inference | Yes | Yes | Yes |
| Gradual typing | No | No | Yes |
| Performance | Slow | Fast | Fast |
| Simplicity | Medium | Complex | **Simple** |

## Learning Curve

- **Python**: Easy to start, but many gotchas
- **Rust**: Steep learning curve, complex concepts
- **TOG**: **Easy to start, stays simple**

## Real-World Example

**Task**: Create a function that greets a user and prints their age.

**Python:**
```python
def greet_user(name, age):
    message = f"Hello, {name}! You are {age} years old."
    print(message)
    return message

if __name__ == "__main__":
    greet_user("Alice", 30)
```

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

**TOG is:**
- Shorter (no boilerplate)
- Simpler (no f-strings, auto-conversion)
- More consistent (everything is a function)
- Safer (memory safety)
- Faster (compiled)

## Conclusion

TOG combines:
- **Python's simplicity** → Made even simpler
- **Rust's safety** → Made more intuitive
- **Rust's performance** → Without the complexity

**TOG: The language that's simpler than Python and better than Rust!**

