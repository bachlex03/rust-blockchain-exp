# Summary

ref: [text](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

## mutable and immutable

ref: [text](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability)

**? WHAT**:

- mutable: is a variable that can be changed after it's set.
- immutable: is a variable that cannot be changed after it's set.

note: Rust enforces immutability by default: after binding a value to a name, you cannot reassign it unless you explicitly mark the binding as mutable with keyword: `mut`.

**? WHY**:

- Immutability is a `core concept` in Rust that helps prevent bugs and make code easier to reason about.
- This helps catch errors early in the development process and makes it easier to reason about the behavior of your code.

**? WHEN**:

- You should use mutable variables when you need to change the value of a variable multiple times.
- You should use immutable variables when you don't need to change the value of a variable after it's set.

**? HOW**:

- To create a mutable variable, you use the `mut` keyword before the variable name.
- To create an immutable variable, you don't use the `mut` keyword.

```rust
/// Create a immutable variable `guess` of type `u32`
let guess: u32 = 66;

/// Create an immutable variable `secret_number` of type `u32`
let mut secret_number: u32 = 37;

secret_number = 66;

print!("The secret number is: {}", secret_number);
```

## Shadowing

ref: [text](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

**? WHAT**:

- Shadowing is creating a new variable with the same name as a previous variable, effectively “overwriting” the old binding.
- The new variable can have a different type or value, and it hides the previous one within its scope.

**? WHY**:

- It allows reusing a variable name when transforming a value through several steps without needing unique names for each step.
- You can change the type of a value while still keeping the same name, which isn’t possible with `mut`.

**? WHEN**:

- Use shadowing when you want to perform a sequence of transformations on a value and don’t need the intermediate values afterward.
- Prefer it over `mut` when the transformation logically produces a new value rather than mutating the existing one.

**? HOW**:

- Declare a new variable with `let` using the same name as an existing variable; the new binding shadows the old one.
- The shadowing is scoped to the block in which the new `let` occurs.

```rust
let variable_name = 10;

// shadowing with different type but with the same name
let variable_name = "hello";

print!("The value of variable_name is: {}", variable_name);
```

## Associated Function

ref: [text](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)

**? WHAT**:

- An associated function is a function defined inside an `impl` block that does **not** take `self` as its first parameter.
- It is called using the `::` syntax on the type itself (e.g., `Type::function_name()`), **similar to static methods in other languages**.

**? WHY**:

- They allow namespace organization by grouping utility or constructor-like logic with the type without requiring an instance.
- This keeps the global namespace clean and signals that the operation is conceptually tied to the type.

**? WHEN**:

- Use an associated function when the operation logically belongs to the type but does not need or cannot use an existing instance.
- Common cases include constructors (e.g., `String::new()`), factory helpers, or type-level constants.

**? HOW**:

- Define the function inside an `impl Type` block without `self`, `&self`, or `&mut self` as the first parameter.
- Invoke it with `TypeName::function_name(args…)`.

```rust
impl TypeName {
    fn associated_function_name(args: Type) -> ReturnType {
        // function body
    }
}

let variable_Name = TypeName::associated_function_name(args);

// example
let mut guess = String::new();
```

## Handling Potential Failure with **Result**

**? WHAT**:

- `Result<T, E>` is an enum (enumeration) that represents either success (`Ok(T)`) or failure (`Err(E)`).
- It forces the caller to explicitly handle both the happy path and the error path.
- An **instance** of Result has an `expect` method that you can call. If this instance of Result is an Err value, `expect` **will cause the program to crash and display the message that you passed as an argument to `expect`** If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, `expect` will **take the return value that `Ok` is holding and return just that value to you so you can use it**. In this case, that **value is the number of bytes in the user’s input.**
- If you don’t call expect, the program will compile, but you’ll get a warning
- Rust warns that you haven’t used the Result value returned from read_line, indicating that the program hasn’t handled a possible error.

**? WHY**:

- Rust lacks exceptions; `Result` is the idiomatic way to propagate and handle recoverable errors.
- By making failure part of the type system, the compiler prevents you from accidentally ignoring errors.

**? WHEN**:

- Use `Result` for any operation that can reasonably fail at runtime (file I/O, parsing, network calls, etc.).
- Prefer `Result` over `panic!` when the caller might be able to recover or retry.

**? HOW**:

- Return `Result<T, E>` from functions that can fail.
- Use pattern matching (`match`, `if let`, `let else`) or the `?` operator to extract the success value or propagate the error.
- Provide meaningful error types (often custom enums or existing std/io/error types).

## Enum (Enumeration)

ref: [text](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

**? WHAT**:

- An enum defines a type with a fixed set of variants; each variant can optionally carry data.
- Common enums in this project: `Result<T, E>` for success/failure and `Ordering` for comparison results.
- Enums are typically consumed with `match` for exhaustive, type-safe branching.

**? WHY**:

- Precisely models finite states and prevents invalid combinations at compile time.
- Enables clear, explicit handling of all cases, improving reliability and readability.
- Integrates with pattern matching for concise and maintainable control flow.

**? WHEN**:

- Representing discrete outcomes or states (e.g., success vs. error, less/greater/equal).
- Designing APIs that must communicate structured results without exceptions.
- In this game: parsing user input (`Result`) and comparing guesses (`Ordering`).

**? HOW**:

```rust
// Define a custom enum with data payloads
enum GameState {
    Playing,
    Won(u32),
}

// Use standard enums from the standard library
use std::cmp::Ordering;

let guess: u32 = match guess.trim().parse::<u32>() {
    Ok(n) => n,
    Err(_) => continue,
};

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => { println!("You win!"); }
}
```

References:

- Parse result handling: `src/main.rs:22–25` (`Result`)
- Comparison branching: `src/main.rs:27–34` (`Ordering`)

## Pattern Matching

ref: [text](https://doc.rust-lang.org/book/ch06-02-match.html)

**? WHAT**:

- Pattern matching lets you destructure and branch on data shapes using `match`, covering all possible cases exhaustively.
- Works with enums (`Result`, `Option`, `Ordering`), literals, ranges, and nested structures.
- In this project, `match` handles parse outcomes and number comparison.

**? WHY**:

- Enforces exhaustive handling, preventing unhandled cases at compile time.
- Improves clarity over chained `if/else` for multi-variant logic.
- Encourages safe control flow by making error and success paths explicit.

**? WHEN**:

- When working with enums like `Result`, `Option`, `Ordering`.
- When branching on structured data, ranges, or specific literal values.
- In this game: after parsing input and when comparing guesses.

**? HOW**:

```rust
// Handle parsing: continue loop on invalid input
let guess: u32 = match guess.trim().parse() {
    Ok(n) => n,
    Err(_) => continue,
};

// Compare using Ordering variants
use std::cmp::Ordering;
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => { println!("You win!"); break; }
}
```

## Crate

**? WHAT**:

- Remember that a crate is a collection of Rust source code files. The project we’ve been building is a **binary crate**, which is an **executable**. The rand crate is a **library crate**, which contains code that is intended to be used in other programs and can’t be executed on its own.
- Cargo considers these versions to have public APIs compatible with version 0.8.5, and this specification ensures you’ll get the latest patch release that will still compile with the code in this chapter. Any version 0.9.0 or greater is not guaranteed to have the same API as what the following examples use.

**? HOW**:

```toml
[dependencies]
rand = "0.8.5"
```

## Traits

refs: [text](https://doc.rust-lang.org/book/ch10-02-traits.html)

**? WHAT**:

- Traits define shared behavior that types can implement; they are Rust’s mechanism for interfaces.
- Common trait usage in this project comes from the `rand` crate, where RNG functions are exposed via traits internally; the code uses the convenience function `rand::random_range` that leverages trait-provided RNG.
- Standard library traits frequently encountered in CLI apps include `Display`/`Debug` for printing and `FromStr` for parsing; in this project, `parse()` uses the `FromStr` trait under the hood to convert `String` to `u32`.

**? WHY**:

- Traits enable polymorphism and code reuse without inheritance, supporting generic programming and clean abstractions.
- Decouple APIs from concrete types, making them easier to test, mock, and extend.

**? WHEN**:

- When multiple types should share behavior (e.g., formatting, parsing, comparison).
- When designing libraries with generic constraints (`T: Display`, `T: FromStr`, etc.).
- In this game: numeric parsing via `FromStr`, and ordering via `Ord`/`PartialOrd` for `cmp`.

**? HOW**:

```rust
use std::str::FromStr;

// FromStr allows parsing from &str to a type
let n = u32::from_str("42").unwrap();

// Display lets you format values with {}
use std::fmt::Display;
fn print_value<T: Display>(x: T) {
    println!("{}", x);
}

// Ord/PartialOrd enable comparisons used by cmp
use std::cmp::Ordering;
fn compare<T: Ord>(a: T, b: T) -> Ordering { a.cmp(&b) }
```

References:

- Parsing via `FromStr` through `str::parse`: `src/main.rs:22–25`
- Ordering via `Ord`/`cmp`: `src/main.rs:27–34`

## Keywords to remember

- `mut`
- `let`
- `match`
- methods
- associated functions
- external crates
- "In Rust, variables are immutable by default"
- `&` indicates that this argument is a reference which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times and one of Rust’s major advantages is how safe and easy it is to use references. For now, all you need to know is that, like variables, references are immutable by default
- The `parse` method returns a Result type

## QnA

### Q: what is the difference between read_line(&guess) and read_line(&mut guess)

A: The difference between `read_line(&guess)` and `read_line(&mut guess)` is that the first one takes a reference to a `String` as an argument, while the second one takes a mutable reference to a `String` as an argument.
ref: [Chapter 4 will explain references more thoroughly.](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

- The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that, like variables, **references are immutable by default**. Hence, you need to write &mut guess rather than &guess to make it mutable

- A **reference type** is a data type that stores a memory address (a reference) pointing to the actual data, rather than holding the data itself, allowing multiple variables to point to the same object in memory. When you assign one reference type variable to another, you copy the address, not the data, so both variables then share and can modify the same underlying object, unlike **value types** which copy the data.

### Q: What is the difference between a method and an associated function?

A: A method is a function defined inside an `impl` block that takes `self` as its first parameter. It is called using the dot syntax on an instance of the type (e.g., `instance.method_name(args…)`). An associated function is a function defined inside an `impl` block that does not take `self` as its first parameter. It is called using the `::` syntax on the type itself (e.g., `TypeName::function_name(args…)`).
