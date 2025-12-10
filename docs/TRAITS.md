# Traits in TOG

Traits in TOG provide a way to define shared behavior across different types, similar to interfaces in other languages. They enable polymorphism, code reuse, and clean abstractions.

## Table of Contents
- [What are Traits?](#what-are-traits)
- [Defining Traits](#defining-traits)
- [Implementing Traits](#implementing-traits)
- [Inherent Implementations](#inherent-implementations)
- [Method Dispatch](#method-dispatch)
- [Examples](#examples)
- [Best Practices](#best-practices)

---

## What are Traits?

Traits define a set of methods that types can implement. They specify **what** a type can do, without specifying **how** it does it.

**Key Benefits:**
- **Polymorphism** - Different types can implement the same trait
- **Code Reuse** - Share behavior across types
- **Abstraction** - Define interfaces without implementation details
- **Type Safety** - Compiler ensures trait methods are implemented

---

## Defining Traits

Use the `trait` keyword to define a trait with method signatures:

```tog
trait Drawable {
    fn draw(self)
    fn area(self) -> float
}
```

**Syntax:**
- `trait TraitName { ... }` - Define a trait
- Method signatures only (no bodies)
- `self` parameter for instance methods
- Optional return types

**Example:**
```tog
trait Printable {
    fn print(self)
    fn to_string(self) -> string
}

trait Comparable {
    fn compare(self, other) -> int
    fn equals(self, other) -> bool
}
```

---

## Implementing Traits

Use `impl TraitName for TypeName` to implement a trait for a specific type:

```tog
struct Circle {
    radius: float
}

impl Drawable for Circle {
    fn draw(self) {
        print("Drawing circle with radius: " + self.radius)
    }
    
    fn area(self) -> float {
        3.14159 * self.radius * self.radius
    }
}
```

**Syntax:**
- `impl TraitName for TypeName { ... }` - Implement trait for type
- Must implement **all** trait methods
- Method bodies are required
- `self` refers to the instance

---

## Inherent Implementations

Inherent implementations add methods directly to a type without a trait:

```tog
impl Circle {
    fn new(radius: float) -> Circle {
        Circle { radius: radius }
    }
    
    fn diameter(self) -> float {
        self.radius * 2.0
    }
}
```

**Syntax:**
- `impl TypeName { ... }` - Add methods to a type
- No trait name
- Can include constructors and type-specific methods

**Use Cases:**
- Constructors (`new`, `from`, etc.)
- Type-specific utility methods
- Methods that don't fit into a trait

---

## Method Dispatch

TOG uses the following method resolution order:

1. **Inherent methods** - Check type's own methods first
2. **Trait methods** - Check implemented traits
3. **Struct methods** - Check struct definition methods (legacy)

**Example:**
```tog
struct Point {
    x: float,
    y: float
}

impl Point {
    fn new(x: float, y: float) -> Point {
        Point { x: x, y: y }
    }
}

impl Drawable for Point {
    fn draw(self) {
        print("Point at (" + self.x + ", " + self.y + ")")
    }
}

fn main() {
    let p = Point::new(10.0, 20.0)  // Inherent method
    p.draw()                         // Trait method
}
```

---

## Examples

### Example 1: Shape Hierarchy

```tog
trait Shape {
    fn area(self) -> float
    fn perimeter(self) -> float
}

struct Rectangle {
    width: float,
    height: float
}

impl Shape for Rectangle {
    fn area(self) -> float {
        self.width * self.height
    }
    
    fn perimeter(self) -> float {
        2.0 * (self.width + self.height)
    }
}

struct Circle {
    radius: float
}

impl Shape for Circle {
    fn area(self) -> float {
        3.14159 * self.radius * self.radius
    }
    
    fn perimeter(self) -> float {
        2.0 * 3.14159 * self.radius
    }
}
```

### Example 2: Serialization

```tog
trait Serializable {
    fn to_json(self) -> string
    fn from_json(json: string) -> self
}

struct User {
    name: string,
    age: int
}

impl Serializable for User {
    fn to_json(self) -> string {
        "{\"name\":\"" + self.name + "\",\"age\":" + self.age + "}"
    }
    
    fn from_json(json: string) -> User {
        // Parse JSON and create User
        // (simplified for example)
        User { name: "parsed", age: 0 }
    }
}
```

### Example 3: Iterator Pattern

```tog
trait Iterator {
    fn next(self) -> int
    fn has_next(self) -> bool
}

struct Range {
    current: int,
    end: int
}

impl Iterator for Range {
    fn next(self) -> int {
        let value = self.current
        self.current = self.current + 1
        value
    }
    
    fn has_next(self) -> bool {
        self.current < self.end
    }
}
```

---

## Best Practices

### 1. Keep Traits Focused
```tog
// Good: Focused trait
trait Drawable {
    fn draw(self)
}

// Bad: Too many responsibilities
trait Everything {
    fn draw(self)
    fn save(self)
    fn load(self)
    fn validate(self)
}
```

### 2. Use Descriptive Names
```tog
// Good
trait Comparable
trait Serializable
trait Drawable

// Bad
trait Thing
trait Stuff
trait DoIt
```

### 3. Prefer Composition Over Inheritance
```tog
// Good: Multiple small traits
trait Drawable { fn draw(self) }
trait Movable { fn move(self, x: float, y: float) }
trait Scalable { fn scale(self, factor: float) }

// Implement multiple traits
impl Drawable for Sprite { ... }
impl Movable for Sprite { ... }
impl Scalable for Sprite { ... }
```

### 4. Use Inherent Impls for Constructors
```tog
impl Point {
    fn new(x: float, y: float) -> Point {
        Point { x: x, y: y }
    }
    
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}
```

### 5. Document Trait Requirements
```tog
// Trait for types that can be compared
// Implementers must provide a total ordering
trait Ord {
    fn compare(self, other) -> int  // Returns -1, 0, or 1
}
```

---

## Future Features (Planned)

- **Trait Bounds** - Generic functions with trait constraints
- **Default Methods** - Provide default implementations in traits
- **Associated Types** - Types associated with traits
- **Trait Objects** - Dynamic dispatch with trait types
- **Operator Overloading** - Implement operators via traits

---

## See Also
- [Structs and Methods](../README.md#structs)
- [Enums](../examples/enums.tog)
- [Type System](../README.md#type-system)
- [Examples](../examples/traits.tog)

