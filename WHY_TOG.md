# Why TOG

## Mission: Combining Simplicity, Safety, and Performance

TOG was designed to combine the best features of modern programming languages while maintaining simplicity and performance.

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

## Key Advantages

### 1. **Simpler Ownership**
TOG's ownership model is intuitive and easy to understand.

### 2. **No Lifetime Annotations**
The compiler handles memory management automatically.

### 3. **Clear Error Messages**
Helpful errors that guide you to the solution.

### 4. **Gradual Typing**
Start without types, add them as needed for optimization.

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

TOG is designed to be easy to learn and use, with a gentle learning curve that stays consistent.

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

**TOG features:**
- Minimal boilerplate
- Simple syntax with auto-conversion
- Consistent design (everything is a function)
- Memory safety
- High performance through compilation

## Conclusion

TOG combines:
- **Simplicity** - Clean, intuitive syntax
- **Safety** - Memory safety without complexity
- **Performance** - Compiled for speed

**TOG: A modern programming language that balances simplicity, safety, and performance.**

