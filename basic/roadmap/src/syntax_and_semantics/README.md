## 1. Variables

**mutable & immutable**:

- All variables are immutable by default

**questions**:

- What is the difference between let x = 5; and let mut y = 10;? Why might you get a compiler error if you try to reassign a value to x?

- What is mutable & immutable?

## 2. Constants

- Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

- Rust’s naming convention for constants is to use all uppercase with underscores between words.

- link: `https://doc.rust-lang.org/rust-by-example/custom_types/constants.html`

### Showing

- Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

- The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number

**questions**: What is difference between mut and showing?

- link: `https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing`

## 3. Data types

### 3.1. Scalar types

**Integer Types**

- Integers Overflow
- term: `panicking`

**Floating Point**

**Number**

**Boolean**

**Characters**

### 3.2. Compound types

**Tuples**

- destructuring
- (.) index
- `unit` (tuple without any value)

- **Arrays**

- stack and heap (chapter 4)
- vector (chapter 8)

#### Numeric operation

- add
- subtract
- multiplication
- division
- remainder

ref: `https://doc.rust-lang.org/book/ch03-02-data-types.html#numeric-operations`
ref: `https://doc.rust-lang.org/book/appendix-02-operators.html#appendix-b-operators-and-symbols` (advance)

#### Unrecoverable (section 9)

ref: `https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic`
