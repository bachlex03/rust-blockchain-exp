# Functions

refs: [text](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## Statement vs Expression

**WHAT ?**:

Statements perform actions and don’t produce a value; expressions evaluate to a
value. Rust is expression-oriented: blocks and control flow (`if`, `match`)
yield values.

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value.

**WHY ?**:

Enables composable code where computations return values directly from blocks
and functions, reduces mutable state, and makes intent clear. A trailing
semicolon turns an expression into a statement that yields `()`.

**WHEN ?**:

Use expressions to compute values for assignments, arguments, and returns. Use
statements for `let` bindings, item declarations, and loops. Omit the semicolon
to use the last expression’s value; add it to discard the value.

**HOW ?**:

```rust
let y = 6;        // statement (binding) using an expression on the right
let z = 5 + 6;    // expression assigned to a binding
```

```rust
let y = { let x = 3; x + 1 };    // block is an expression, value is 4
```

```rust
fn five() -> i32 { 5 }
fn plus_one(x: i32) -> i32 { x + 1 }
```

```rust
let a = { 3 + 4 };         // 7
let b: () = { 3 + 4; };    // unit value because of semicolon
```

```rust
let n = -3;
let abs = if n >= 0 { n } else { -n };    // if is an expression
```

```rust
let kind = match abs {
    0 => "zero",
    1..=9 => "small",
    _ => "large",
};
```

## QnA

### Q: Rust is an expression-based language?

A: Yes, Rust is an expression-based language. This means that every expression in Rust has a value, and you can use expressions in places where you would use a value, such as assignments, function arguments, and return values.

### Q: Calling a function is an expression?

A: Yes, calling a function is an expression in Rust. This means that the function call evaluates to a value, which can be assigned to a variable or used in other expressions.

### Q: Calling a macro is an expression?

A: Yes, calling a macro is an expression in Rust. This means that the macro invocation evaluates to a value, which can be assigned to a variable or used in other expressions.

### Q: A new scope block created with curly brackets is an expression?

A: Yes, a new scope block created with curly brackets is an expression in Rust. This means that the block evaluates to a value, which is the value of the last expression in the block.

- Example:

```rust
let x = {
    let y = 3;
    y + 1
};
```

### Note: In function expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
