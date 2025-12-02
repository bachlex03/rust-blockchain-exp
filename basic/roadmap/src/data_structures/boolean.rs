fn main() {
    println!("=== Boolean Type in Rust ===\n");

    // 1. Boolean basics
    demonstrate_bool_basics();

    // 2. Boolean operations
    demonstrate_bool_operations();

    // 3. Logical operators
    demonstrate_logical_operators();

    // 4. Comparison operators
    demonstrate_comparison_operators();

    // 5. Boolean in control flow
    demonstrate_control_flow();

    // 6. Boolean casting
    demonstrate_bool_casting();

    // 7. Short-circuit evaluation
    demonstrate_short_circuit();

    // 8. Practical examples
    demonstrate_practical_examples();
}

// 1. Boolean basics
fn demonstrate_bool_basics() {
    println!("1. Boolean Basics");
    println!("   bool has only two values: true and false\n");

    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("   t = {}", t);
    println!("   f = {}", f);

    println!("\n   Size of bool: {} byte", std::mem::size_of::<bool>());
    println!("   Booleans are one byte in size\n");
}

// 2. Boolean operations
fn demonstrate_bool_operations() {
    println!("2. Boolean Operations");
    println!("   Bitwise and logical operations on booleans\n");

    let a = true;
    let b = false;

    // Bitwise AND (&)
    println!("   Bitwise AND (&):");
    println!("   true & true = {}", true & true);
    println!("   true & false = {}", true & false);
    println!("   false & false = {}", false & false);

    // Bitwise OR (|)
    println!("\n   Bitwise OR (|):");
    println!("   true | true = {}", true | true);
    println!("   true | false = {}", true | false);
    println!("   false | false = {}", false | false);

    // Bitwise XOR (^)
    println!("\n   Bitwise XOR (^):");
    println!("   true ^ true = {}", true ^ true);
    println!("   true ^ false = {}", true ^ false);
    println!("   false ^ false = {}", false ^ false);

    // NOT (!)
    println!("\n   NOT (!):");
    println!("   !true = {}", !a);
    println!("   !false = {}", !b);

    println!();
}

// 3. Logical operators
fn demonstrate_logical_operators() {
    println!("3. Logical Operators");
    println!("   Short-circuiting logical operators\n");

    let _a = true;
    let _b = false;

    // Logical AND (&&)
    println!("   Logical AND (&&):");
    println!("   true && true = {}", true && true);
    println!("   true && false = {}", true && false);
    println!("   false && true = {}", false && true);
    println!("   false && false = {}", false && false);

    // Logical OR (||)
    println!("\n   Logical OR (||):");
    println!("   true || true = {}", true || true);
    println!("   true || false = {}", true || false);
    println!("   false || true = {}", false || true);
    println!("   false || false = {}", false || false);

    // Difference between & and &&, | and ||
    println!("\n   Difference:");
    println!("   & and | evaluate both operands");
    println!("   && and || short-circuit (stop early if result is known)");

    println!();
}

// 4. Comparison operators
fn demonstrate_comparison_operators() {
    println!("4. Comparison Operators");
    println!("   Comparisons return boolean values\n");

    let x = 5;
    let y = 10;

    println!("   x = {}, y = {}", x, y);
    println!("   x == y: {}", x == y);
    println!("   x != y: {}", x != y);
    println!("   x < y: {}", x < y);
    println!("   x > y: {}", x > y);
    println!("   x <= y: {}", x <= y);
    println!("   x >= y: {}", x >= y);

    // Boolean comparisons
    println!("\n   Boolean comparisons:");
    println!("   true == true: {}", true == true);
    println!("   true == false: {}", true == false);
    println!("   true != false: {}", true != false);

    println!();
}

// 5. Boolean in control flow
fn demonstrate_control_flow() {
    println!("5. Boolean in Control Flow");
    println!("   if expressions require boolean conditions\n");

    let condition = true;

    // if expression
    if condition {
        println!("   condition is true");
    } else {
        println!("   condition is false");
    }

    // if-else chain
    let number = 6;
    if number % 4 == 0 {
        println!("   {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("   {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("   {} is divisible by 2", number);
    } else {
        println!("   {} is not divisible by 4, 3, or 2", number);
    }

    // if as an expression
    let result = if condition { "yes" } else { "no" };
    println!("   Result: {}", result);

    // while loop with boolean
    let mut count = 0;
    while count < 3 {
        count += 1;
    }
    println!("   Counted to {}", count);

    println!();
}

// 6. Boolean casting
fn demonstrate_bool_casting() {
    println!("6. Boolean Casting");
    println!("   Converting between bool and integers\n");

    // bool to integer
    let t = true;
    let f = false;

    println!("   true as i32 = {}", t as i32);
    println!("   false as i32 = {}", f as i32);
    println!("   true as u8 = {}", t as u8);
    println!("   false as u8 = {}", f as u8);

    // Note: Cannot directly cast integer to bool
    // Must use comparison
    let num = 5;
    let is_nonzero = num != 0;
    println!("\n   {} != 0 = {}", num, is_nonzero);

    let zero = 0;
    let is_zero = zero == 0;
    println!("   {} == 0 = {}", zero, is_zero);

    println!();
}

// 7. Short-circuit evaluation
fn demonstrate_short_circuit() {
    println!("7. Short-Circuit Evaluation");
    println!("   && and || stop evaluating when result is known\n");

    // && short-circuits
    println!("   Logical AND (&&) short-circuits:");
    let result = false && {
        println!("   This won't print!");
        true
    };
    println!("   Result: {}", result);

    let result2 = true && {
        println!("   This will print!");
        true
    };
    println!("   Result: {}", result2);

    // || short-circuits
    println!("\n   Logical OR (||) short-circuits:");
    let result3 = true || {
        println!("   This won't print!");
        false
    };
    println!("   Result: {}", result3);

    let result4 = false || {
        println!("   This will print!");
        true
    };
    println!("   Result: {}", result4);

    println!();
}

// 8. Practical examples
fn demonstrate_practical_examples() {
    println!("8. Practical Examples\n");

    // Example 1: Validation
    println!("   Example 1: Input validation");
    fn is_valid_age(age: i32) -> bool {
        age >= 0 && age <= 150
    }

    let age = 25;
    println!("   Is {} a valid age? {}", age, is_valid_age(age));
    println!("   Is -5 a valid age? {}", is_valid_age(-5));

    // Example 2: Range checking
    println!("\n   Example 2: Range checking");
    fn is_in_range(value: i32, min: i32, max: i32) -> bool {
        value >= min && value <= max
    }

    let score = 85;
    println!("   Is {} in range [0, 100]? {}", score, is_in_range(score, 0, 100));

    // Example 3: Multiple conditions
    println!("\n   Example 3: Multiple conditions");
    fn can_vote(age: i32, is_citizen: bool, is_registered: bool) -> bool {
        age >= 18 && is_citizen && is_registered
    }

    println!(
        "   Can vote (age=20, citizen=true, registered=true)? {}",
        can_vote(20, true, true)
    );
    println!(
        "   Can vote (age=16, citizen=true, registered=true)? {}",
        can_vote(16, true, true)
    );

    // Example 4: String validation
    println!("\n   Example 4: String validation");
    fn is_valid_username(username: &str) -> bool {
        !username.is_empty() && username.len() >= 3 && username.len() <= 20
    }

    println!("   Is 'john' valid? {}", is_valid_username("john"));
    println!("   Is 'ab' valid? {}", is_valid_username("ab"));
    println!("   Is '' valid? {}", is_valid_username(""));

    // Example 5: Combining conditions
    println!("\n   Example 5: Password strength");
    fn is_strong_password(password: &str) -> bool {
        let has_length = password.len() >= 8;
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());

        has_length && has_uppercase && has_lowercase && has_digit
    }

    println!("   Is 'Password123' strong? {}", is_strong_password("Password123"));
    println!("   Is 'weak' strong? {}", is_strong_password("weak"));

    // Example 6: Boolean flags
    println!("\n   Example 6: Feature flags");
    #[allow(dead_code)]
    struct Config {
        debug_mode: bool,
        verbose: bool,
        auto_save: bool,
    }

    let config = Config {
        debug_mode: true,
        verbose: false,
        auto_save: true,
    };

    if config.debug_mode {
        println!("   Debug mode is enabled");
    }

    if config.auto_save {
        println!("   Auto-save is enabled");
    }

    println!();
}