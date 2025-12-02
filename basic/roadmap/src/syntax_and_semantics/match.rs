fn main() {
    println!("=== Match Control Flow in Rust ===\n");

    // 1. Basic match - like C switch but more powerful
    demonstrate_basic_match();

    // 2. Match as an expression
    demonstrate_match_expression();

    // 3. Match with multiple patterns
    demonstrate_multiple_patterns();

    // 4. Match with ranges
    demonstrate_range_matching();

    // 5. Match must be exhaustive
    demonstrate_exhaustive_matching();

    // 6. Match with guards
    demonstrate_match_guards();

    // 7. Match with binding
    demonstrate_binding();
}

// 1. Basic match - similar to C switch
fn demonstrate_basic_match() {
    println!("1. Basic Match");
    println!("   Pattern matching like C switch, but more powerful\n");

    let number = 13;

    println!("   Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("   One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("   This is a prime"),
        // Match an inclusive range
        13..=19 => println!("   A teen"),
        // Handle the rest of cases
        _ => println!("   Ain't special"),
    }
    println!();
}

// 2. Match as an expression
fn demonstrate_match_expression() {
    println!("2. Match as an Expression");
    println!("   Match returns a value\n");

    let boolean = true;

    // Match is an expression - it returns a value
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("   {} -> {}", boolean, binary);

    // Another example with more complex expressions
    let number = 3;
    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };

    println!("   {} is {}", number, description);
    println!();
}

// 3. Match with multiple patterns using |
fn demonstrate_multiple_patterns() {
    println!("3. Multiple Patterns with |");
    println!("   Match multiple values in one arm\n");

    let number = 7;

    match number {
        1 => println!("   One"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("   {} is a prime number", number),
        _ => println!("   {} is not a prime (or not in our list)", number),
    }

    // Multiple patterns with different types
    let character = 'x';
    match character {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("   '{}' is a vowel", character),
        'y' => println!("   '{}' is sometimes a vowel", character),
        _ => println!("   '{}' is a consonant", character),
    }
    println!();
}

// 4. Match with ranges
fn demonstrate_range_matching() {
    println!("4. Range Matching");
    println!("   Match inclusive ranges with ..=\n");

    let age = 17;

    match age {
        0 => println!("   Just born!"),
        1..=12 => println!("   {} years old - child", age),
        13..=19 => println!("   {} years old - teenager", age),
        20..=64 => println!("   {} years old - adult", age),
        65..=120 => println!("   {} years old - senior", age),
        _ => println!("   Age {} seems unlikely!", age),
    }

    // Ranges work with chars too
    let grade = 'B';
    match grade {
        'A'..='C' => println!("   Grade {} - passing", grade),
        'D' | 'F' => println!("   Grade {} - failing", grade),
        _ => println!("   Invalid grade"),
    }
    println!();
}

// 5. Match must be exhaustive
fn demonstrate_exhaustive_matching() {
    println!("5. Exhaustive Matching");
    println!("   All possible values must be covered\n");

    let number = 42;

    // This match is exhaustive because _ catches everything else
    match number {
        0 => println!("   Zero"),
        1..=10 => println!("   Between 1 and 10"),
        _ => println!("   {} - something else", number),
    }

    // For enums, all variants must be covered
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let direction = Direction::North;

    match direction {
        Direction::North => println!("   Going North"),
        Direction::South => println!("   Going South"),
        Direction::East => println!("   Going East"),
        Direction::West => println!("   Going West"),
        // No _ needed - all variants covered
    }
    println!();
}

// 6. Match with guards (additional conditions)
fn demonstrate_match_guards() {
    println!("6. Match Guards");
    println!("   Add conditions with if after the pattern\n");

    let pair = (2, -2);

    match pair {
        (x, y) if x == y => println!("   ({}, {}) - These are twins", x, y),
        (x, y) if x + y == 0 => println!("   ({}, {}) - These sum to zero", x, y),
        (x, _) if x % 2 == 0 => println!("   ({}, _) - First is even", x),
        _ => println!("   No special relationship"),
    }

    // Guard with range
    let number = 4;
    match number {
        n if n < 0 => println!("   {} is negative", n),
        n if n == 0 => println!("   {} is zero", n),
        n if n > 0 && n < 10 => println!("   {} is a small positive number", n),
        n => println!("   {} is a large positive number", n),
    }
    println!();
}

// 7. Match with binding using @
fn demonstrate_binding() {
    println!("7. Binding with @");
    println!("   Bind values while testing patterns\n");

    let age = 15;

    match age {
        0 => println!("   Not born yet? Impossible!"),
        n @ 1..=12 => println!("   Child of age {}", n),
        n @ 13..=19 => println!("   Teen of age {}", n),
        n => println!("   Adult of age {}", n),
    }

    // Binding in more complex patterns
    let point = (3, 10);
    match point {
        (0, 0) => println!("   Origin"),
        (x @ 0..=5, y @ 0..=5) => {
            println!("   Point ({}, {}) is in the small box", x, y);
        }
        (x, y) => println!("   Point ({}, {}) is outside the small box", x, y),
    }
    println!();
}