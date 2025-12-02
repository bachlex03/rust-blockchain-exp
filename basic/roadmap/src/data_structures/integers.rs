fn main() {
    println!("=== Integer Types in Rust ===\n");

    // 1. Integer type basics
    demonstrate_integer_basics();

    // 2. Signed vs unsigned integers
    demonstrate_signed_unsigned();

    // 3. Integer sizes and ranges
    demonstrate_integer_sizes();

    // 4. Integer literals
    demonstrate_integer_literals();

    // 5. Type inference
    demonstrate_type_inference();

    // 6. Integer overflow
    demonstrate_integer_overflow();

    // 7. Architecture-dependent types
    demonstrate_arch_dependent_types();
}

// 1. Integer type basics
fn demonstrate_integer_basics() {
    println!("1. Integer Type Basics");
    println!("   Integers are whole numbers without fractional components\n");

    let x: i32 = 42;
    let y: u32 = 100;

    println!("   Signed integer (i32): {}", x);
    println!("   Unsigned integer (u32): {}", y);

    println!("\n   Default integer type is i32\n");
}

// 2. Signed vs unsigned integers
fn demonstrate_signed_unsigned() {
    println!("2. Signed vs Unsigned Integers");
    println!("   Signed can be negative, unsigned cannot\n");

    // Signed integers (can be negative)
    let signed_positive: i32 = 200;
    let signed_negative: i32 = -200;

    println!("   Signed integers (i32):");
    println!("   Positive: {}", signed_positive);
    println!("   Negative: {}", signed_negative);

    // Unsigned integers (only positive)
    let unsigned: u32 = 300;

    println!("\n   Unsigned integer (u32):");
    println!("   Positive: {}", unsigned);
    // let unsigned_negative: u32 = -100; // Error! Cannot be negative

    println!("\n   Signed: i8, i16, i32, i64, i128");
    println!("   Unsigned: u8, u16, u32, u64, u128\n");
}

// 3. Integer sizes and ranges
fn demonstrate_integer_sizes() {
    println!("3. Integer Sizes and Ranges");
    println!("   Different sizes store different ranges of values\n");

    // 8-bit integers
    let i8_min: i8 = i8::MIN;
    let i8_max: i8 = i8::MAX;
    let u8_max: u8 = u8::MAX;

    println!("   8-bit:");
    println!("   i8 range: {} to {}", i8_min, i8_max);
    println!("   u8 range: 0 to {}", u8_max);

    // 16-bit integers
    let i16_min: i16 = i16::MIN;
    let i16_max: i16 = i16::MAX;
    let u16_max: u16 = u16::MAX;

    println!("\n   16-bit:");
    println!("   i16 range: {} to {}", i16_min, i16_max);
    println!("   u16 range: 0 to {}", u16_max);

    // 32-bit integers
    let i32_min: i32 = i32::MIN;
    let i32_max: i32 = i32::MAX;
    let u32_max: u32 = u32::MAX;

    println!("\n   32-bit:");
    println!("   i32 range: {} to {}", i32_min, i32_max);
    println!("   u32 range: 0 to {}", u32_max);

    // 64-bit integers
    let i64_min: i64 = i64::MIN;
    let i64_max: i64 = i64::MAX;
    let u64_max: u64 = u64::MAX;

    println!("\n   64-bit:");
    println!("   i64 range: {} to {}", i64_min, i64_max);
    println!("   u64 range: 0 to {}", u64_max);

    // 128-bit integers
    println!("\n   128-bit:");
    println!("   i128 range: {} to {}", i128::MIN, i128::MAX);
    println!("   u128 range: 0 to {}", u128::MAX);

    println!();
}

// 4. Integer literals
fn demonstrate_integer_literals() {
    println!("4. Integer Literals");
    println!("   Different ways to write integer values\n");

    // Decimal
    let decimal = 98_222;
    println!("   Decimal: {}", decimal);

    // Hexadecimal
    let hex = 0xff;
    println!("   Hexadecimal (0xff): {}", hex);

    // Octal
    let octal = 0o77;
    println!("   Octal (0o77): {}", octal);

    // Binary
    let binary = 0b1111_0000;
    println!("   Binary (0b1111_0000): {}", binary);

    // Byte (u8 only)
    let byte = b'A';
    println!("   Byte (b'A'): {}", byte);

    // Type suffix
    let typed = 57u8;
    println!("   With type suffix (57u8): {}", typed);

    // Visual separator
    let large_number = 1_000_000;
    println!("   With separator (1_000_000): {}", large_number);

    println!();
}

// 5. Type inference
fn demonstrate_type_inference() {
    println!("5. Type Inference");
    println!("   Rust can infer integer types from context\n");

    // Rust infers i32 by default
    let inferred = 42;
    println!("   let inferred = 42;");
    println!("   Type inferred as i32: {}", inferred);

    // Inference from usage
    let mut number = 10;
    number = number + 5;
    println!("\n   let mut number = 10;");
    println!("   number = number + 5;");
    println!("   Type inferred from operations: {}", number);

    // Explicit type annotation when needed
    let explicit: u64 = 100;
    println!("\n   let explicit: u64 = 100;");
    println!("   Explicit type annotation: {}", explicit);

    // Type inference from function return
    let parsed: i32 = "42".parse().expect("Not a number");
    println!("\n   let parsed: i32 = \"42\".parse().expect(\"Not a number\");");
    println!("   Parsed string to integer: {}", parsed);

    println!();
}

// 6. Integer overflow
fn demonstrate_integer_overflow() {
    println!("6. Integer Overflow");
    println!("   Handling values outside the type's range\n");

    // Debug mode: panics on overflow
    println!("   In debug mode: overflow causes panic");
    println!("   In release mode: two's complement wrapping");

    // Safe overflow handling methods
    let x: u8 = 255;

    // wrapping_* methods
    let wrapped = x.wrapping_add(1);
    println!("\n   255u8.wrapping_add(1) = {}", wrapped);

    // checked_* methods (returns Option)
    let checked = x.checked_add(1);
    println!("   255u8.checked_add(1) = {:?}", checked);

    // overflowing_* methods (returns value and bool)
    let (result, overflowed) = x.overflowing_add(1);
    println!("   255u8.overflowing_add(1) = ({}, {})", result, overflowed);

    // saturating_* methods (clamps at boundaries)
    let saturated = x.saturating_add(1);
    println!("   255u8.saturating_add(1) = {}", saturated);

    println!("\n   Use these methods to explicitly handle overflow\n");
}

// 7. Architecture-dependent types
fn demonstrate_arch_dependent_types() {
    println!("7. Architecture-Dependent Types");
    println!("   isize and usize depend on system architecture\n");

    let size: usize = 100;
    let signed_size: isize = -50;

    println!("   usize: {}", size);
    println!("   isize: {}", signed_size);

    println!("\n   Size in bytes:");
    println!("   usize: {} bytes", std::mem::size_of::<usize>());
    println!("   isize: {} bytes", std::mem::size_of::<isize>());

    println!("\n   Ranges:");
    println!("   usize: 0 to {}", usize::MAX);
    println!("   isize: {} to {}", isize::MIN, isize::MAX);

    println!("\n   Use cases:");
    println!("   - Array/vector indexing");
    println!("   - Collection sizes");
    println!("   - Pointer arithmetic");
    println!("   - Memory addresses");

    // Example: array indexing
    let array = [10, 20, 30, 40, 50];
    let index: usize = 2;
    println!("\n   Array indexing example:");
    println!("   array[{}] = {}", index, array[index]);

    println!();
}