# Chapter 3: Variables Mutability

refs: [text](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

## Mutable & Immutable Variables

Rust’s philosophy revolves around safety without sacrificing performance. The mutability rules are one of the cornerstones of that safety.

**WHAT ?**:

By default, all variable bindings in Rust are immutable.

**WHY ?**:

Reason,Explanation

1. Prevents bugs,"Accidental mutation is one of the most common sources of bugs in C, C++, JavaScript, Python, etc."
2. Enables fearless concurrency,Immutable data can be safely shared between threads without locks. This is why Arc<T> works so well.
3. Better optimizations,"The compiler can make stronger assumptions (e.g., constant propagation, dead store elimination) when data never changes."
4. Data-race freedom,Rust’s borrow checker can guarantee at compile time that mutable data is never accessed concurrently unsafely.
5. Clear intent,"When you see let mut, you immediately know “this thing is intended to change”."

**WHEN ?**:
**HOW ?**:

```rust
let mut counter = 0;
counter += 1;           // OK
counter = counter * 2;  // OK
```

```rust
struct Point { x: i32, y: i32 }

let mut p = Point { x: 0, y: 0 };
p.x = 10;   // OK because p is mut
```

```rust
let p = Point { x: 0, y: 0 };
p.x = 10;   // ERROR: cannot mutate immutable variable
```

```rust
// Interior mutability (when you need mutation inside immutable context)
// Sometimes you want mutable data behind an immutable reference (e.g., in a struct field that is shared).
Rustuse std::cell::RefCell;

struct HasMutableState {
history: RefCell<Vec<String>>,
}

let obj = HasMutableState {
history: RefCell::new(vec![])
};

obj.history.borrow_mut().push("login".to_string());
// Works even if `obj` itself is immutable!
```

## Constants

**WHAT ?**:

Compile-time values that never change. Must have an explicit type and cannot be
declared `mut`. Can be defined in any scope, including the crate root.

**WHY ?**:

Provide stable, nameable values for configuration and invariants, enable
optimizations, and communicate intent that a value is fixed.

**WHEN ?**:

Use for domain invariants, limits, bit masks, time units, and sizes known at
compile time. Prefer `const` over `let` when the value should not change and is
independent of runtime state.

**HOW ?**:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

```rust
const MAX_POINTS: u32 = 100_000;
```

## Shadowing

**WHAT ?**:

Creating a new variable with the same name in the same scope, which replaces
the previous binding. The new variable can have a different type or value.

**WHY ?**:

Allows reusing a name for successive transformations, avoids inventing new
names, and enforces immutability at each step. Makes intent explicit: you must
use `let` to change the value, preventing accidental reassignment.

**WHEN ?**:

Use when performing a sequence of transformations on data where each step
produces a new value. Prefer over `mut` when the intermediate values should
remain immutable. Combine with type changes (e.g., string → length) to encode
progression clearly.

**HOW ?**:

```rust
let x = 5;
let x = x + 1;
```

```rust
let spaces = "   ";
let spaces = spaces.len();

// if we try to use mut for this, as shown here, we’ll get a compile-time error
let mut spaces = "   ";
spaces = spaces.len(); // error
```

# Chapter 3.1: Data types

refs: [text](https://doc.rust-lang.org/book/ch03-02-data-types.html)

## Scalar Types

**WHAT ?**:

Single-value types: integers (`i*`, `u*`), floating points (`f32`, `f64`),
`bool`, and `char` (Unicode scalar value, 4 bytes).

**WHY ?**:

Represent numeric quantities, flags, and single characters with precise sizes
and operations. Rust’s static typing enables safety and performance.

**WHEN ?**:

Defaults: `i32` for integers, `f64` for floats. Use unsigned types (`u*`) for
non-negative counts. Use `bool` for conditions, `char` for single Unicode
characters. Add type annotations when inference is ambiguous.

**HOW ?**:

```rust
let x = 2.0;        // f64 by default
let y: f32 = 3.0;   // explicit 32-bit float

let sum = 5 + 10;
let difference = 95.5 - 4.3;
let product = 4 * 30;
let quotient = 56.7 / 32.2;
let truncated = -5 / 3; // -1 (integer division)
let remainder = 43 % 5; // 3

let t = true;
let f: bool = false;

let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat = '😻';
```

## Compound Types

**WHAT ?**:

Tuples and arrays. Tuples group values of possibly different types with fixed
length. Arrays store a fixed-length sequence of one type.

**WHY ?**:

Group related values compactly, return multiple values, and use stack-allocated fixed-size collections. Prefer `Vec<T>` when the length can change.

**WHEN ?**:

Use tuples for heterogeneous grouping and function returns. Use arrays for
compile-time-known sizes and indexing. Switch to vectors for dynamic sizes.

**HOW ?**:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;            // destructuring
let a0 = tup.0;                 // indexing
let a1 = tup.1;
let a2 = tup.2;
```

```rust
let a = [1, 2, 3, 4, 5];
let months = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];
let first = months[0];
let second = months[1];

let a: [i32; 5] = [1, 2, 3, 4, 5];

let a = [3; 5]; // [3, 3, 3, 3, 3]
```

## QnA

### Q: What is concurrency in Rust?

A: Running multiple tasks that make progress independently within a program.
In Rust, concurrency uses threads, message passing (channels), shared-state with
sync primitives (`Arc`, `Mutex`), and async tasks (`async`/`await`). The
ownership and type system, plus `Send`/`Sync`, help prevent data races at
compile time.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }));
}

for h in handles { h.join().unwrap(); }
```

### Q: What if we assign a new type to a mutable variable?

```rust
let mut spaces = "   ";
spaces = spaces.len(); // error
```

A:

### Q: What is Rust's type system?

A: Rust's type system is **statically typed language**, meaning types are checked at compile time. It ensures memory safety and prevents runtime errors related to type mismatches.

- which means that it must know the types of all variables at compile time

### Q: Rust is an expression-based language?

A: Yes, Rust is an expression-based language. This means that every expression in Rust has a value, and you can use expressions in places where you would use a value, such as assignments, function arguments, and return values.

### Q: What is integer overflow?

A: Integer overflow occurs when a mathematical operation on integers results in a value that is outside the valid range of that integer type. In Rust, integer overflow is **undefined behavior**, meaning the compiler is free to do whatever it wants, including wrapping the value around.

For example, consider the following code:

```rust
let mut x = u8::MAX;
x += 1;
```

In this case, `x` is a `u8` type, which has a range of 0 to 255. When we add 1 to `x`, it overflows and wraps around to 0. This is undefined behavior, and the compiler is free to do whatever it wants.

To avoid integer overflow, you can use the `wrapping_*` methods provided by the `std::num::Wrapping` type. These methods perform the operation and wrap the result around, rather than causing undefined behavior.

For example:

```rust
let mut x = Wrapping(u8::MAX);
x += Wrapping(1);
assert_eq!(x.0, 0);
```

### Q: What --release flag?

A: Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. Instead, it relies on the programmer to ensure that integer operations do not result in overflow. If overflow occurs, the behavior is undefined, and the program may crash or produce unexpected results.

- Example: In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have

### Q: How to handle integer overflow?

A: To handle integer overflow, you can use the `wrapping_*` methods provided by the `std::num::Wrapping` type. These methods perform the operation and wrap the result around, rather than causing undefined behavior.

- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
- Return the None value if there is overflow with the `checked_*` methods.
- Return the value and a Boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods.

### Q: How many bytes for Boolean type?

A: 1 byte

### Q: How many bytes for Char type?

A: 4 bytes

### Q: When to use Array or Vector?

A: Use arrays when the length is known at compile time and won't change. Use vectors when the length can change or when you need to store a dynamic number of elements.

- Arrays are more useful when you know the number of elements will not need to change
