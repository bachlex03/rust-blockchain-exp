// Define some types for pattern matching examples
#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("=== Patterns and Matching in Rust ===\n");

    // 1. Basic match expressions
    demonstrate_basic_match();

    // 2. Matching literals
    demonstrate_literal_patterns();

    // 3. Destructuring tuples
    demonstrate_tuple_destructuring();

    // 4. Destructuring structs
    demonstrate_struct_destructuring();

    // 5. Destructuring enums
    demonstrate_enum_destructuring();

    // 6. Variables in patterns
    demonstrate_variables_in_patterns();

    // 7. Wildcards and placeholders
    demonstrate_wildcards();

    // 8. Multiple patterns
    demonstrate_multiple_patterns();

    // 9. Pattern guards
    demonstrate_pattern_guards();

    // 10. if let and while let
    demonstrate_if_let();

    // 11. Refutable vs Irrefutable patterns
    demonstrate_refutable_patterns();
}

// Basic match expression
fn demonstrate_basic_match() {
    println!("1. Basic Match Expressions");
    println!("   Match compares a value against patterns\n");

    let coin = Coin::Dime;
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };

    println!("   match coin {{");
    println!("       Coin::Penny => 1,");
    println!("       Coin::Nickel => 5,");
    println!("       Coin::Dime => 10,");
    println!("       Coin::Quarter => 25,");
    println!("   }}");
    println!("   → value = {}\n", value);

    println!("   Match must be exhaustive (cover all cases)");
    println!("   Each arm returns a value\n");
}

// Matching literals
fn demonstrate_literal_patterns() {
    println!("2. Matching Literals");
    println!("   Match against specific values\n");

    let number = 3;

    match number {
        1 => println!("   number is one"),
        2 => println!("   number is two"),
        3 => println!("   number is three"),
        _ => println!("   number is something else"),
    }
    println!();
}

// Destructuring tuples
fn demonstrate_tuple_destructuring() {
    println!("3. Destructuring Tuples");
    println!("   Extract values from tuples in patterns\n");

    let point = (3, 5);

    match point {
        (0, 0) => println!("   Point is at origin"),
        (0, y) => println!("   Point is on y-axis at y = {}", y),
        (x, 0) => println!("   Point is on x-axis at x = {}", x),
        (x, y) => println!("   Point is at ({}, {})", x, y),
    }
    println!();

    // Using tuple destructuring in let
    let (x, y) = point;
    println!("   let (x, y) = point;");
    println!("   → x = {}, y = {}\n", x, y);
}

// Destructuring structs
fn demonstrate_struct_destructuring() {
    println!("4. Destructuring Structs");
    println!("   Extract fields from structs\n");

    let point = Point { x: 0, y: 7 };

    match point {
        Point { x: 0, y } => println!("   Point is on y-axis at y = {}", y),
        Point { x, y: 0 } => println!("   Point is on x-axis at x = {}", x),
        Point { x, y } => println!("   Point is at ({}, {})", x, y),
    }
    println!();

    // Shorthand when variable names match field names
    let Point { x, y } = point;
    println!("   let Point {{ x, y }} = point;");
    println!("   → x = {}, y = {}\n", x, y);
}

// Destructuring enums
fn demonstrate_enum_destructuring() {
    println!("5. Destructuring Enums");
    println!("   Extract data from enum variants\n");

    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("   Quit message"),
        Message::Move { x, y } => {
            println!("   Move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("   Write: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("   Change color to RGB({}, {}, {})", r, g, b);
        }
    }
    println!();

    // Matching nested data
    let msg2 = Message::Write(String::from("Hello"));
    match msg2 {
        Message::Write(s) => println!("   Message::Write contains: \"{}\"\n", s),
        _ => (),
    }
}

// Variables in patterns
fn demonstrate_variables_in_patterns() {
    println!("6. Variables in Patterns");
    println!("   Bind values to variables in patterns\n");

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("   Got 50"),
        Some(y) => println!("   Matched, y = {} (shadows outer y)", y),
        _ => println!("   Default case, x = {:?}", x),
    }

    println!("   → Outer y = {} (not affected by match)", y);
    println!();

    // Using @ to bind and match
    println!("   @ Bindings - bind value while testing pattern:");

    #[derive(Debug)]
    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id @ 3..=7 } => {
            println!("   Found an id in range: {}", id);
        }
        Message2::Hello { id: 10..=12 } => {
            println!("   Found an id in another range");
        }
        Message2::Hello { id } => println!("   Found some other id: {}", id),
    }
    println!();
}

// Wildcards and placeholders
fn demonstrate_wildcards() {
    println!("7. Wildcards and Placeholders");
    println!("   Use _ to ignore values\n");

    let number = 42;

    match number {
        1 => println!("   number is one"),
        _ => println!("   number is something else (wildcard matches anything)"),
    }
    println!();

    // Ignoring parts of values
    let tuple = (1, 2, 3, 4, 5);
    match tuple {
        (first, _, third, _, fifth) => {
            println!("   First: {}, Third: {}, Fifth: {}", first, third, fifth);
            println!("   → Second and fourth ignored with _");
        }
    }
    println!();

    // Using .. to ignore remaining parts
    println!("   Using .. to ignore remaining parts:");

    #[allow(dead_code)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("   x is {} (y and z ignored with ..)", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!(
                "   First: {}, Last: {} (middle ignored with ..)",
                first, last
            );
        }
    }
    println!();

    // Ignoring unused variables with _
    let _unused = 5; // Prefix with _ to avoid unused variable warning
    println!("   Prefix unused variables with _ to avoid warnings");
    println!("   Note: _x binds value, _ doesn't bind at all\n");
}

// Multiple patterns with |
fn demonstrate_multiple_patterns() {
    println!("8. Multiple Patterns");
    println!("   Use | to match multiple patterns\n");

    let number = 3;

    match number {
        1 | 2 | 3 => println!("   number is 1, 2, or 3"),
        4 | 5 => println!("   number is 4 or 5"),
        _ => println!("   number is something else"),
    }
    println!();

    // Matching ranges with ..=
    println!("   Matching ranges with ..=:");
    let x = 5;

    match x {
        1..=5 => println!("   {} is in range 1 through 5", x),
        _ => println!("   {} is something else", x),
    }

    // Ranges work with char too
    let letter = 'c';
    match letter {
        'a'..='j' => println!("   '{}' is an early ASCII letter", letter),
        'k'..='z' => println!("   '{}' is a late ASCII letter", letter),
        _ => println!("   '{}' is something else", letter),
    }
    println!();

    // Multiple patterns with destructuring
    let point = (0, 5);
    match point {
        (0, y) | (y, 0) => {
            println!("   Point is on an axis, coordinate = {}", y);
        }
        (x, y) => {
            println!("   Point is at ({}, {})", x, y);
        }
    }
    println!();
}

// Pattern guards
fn demonstrate_pattern_guards() {
    println!("9. Pattern Guards");
    println!("   Add conditions with if after pattern\n");

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("   {} is less than 5", x),
        Some(x) => println!("   {} is 5 or greater", x),
        None => println!("   No value"),
    }
    println!();

    // Using guards with multiple patterns
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("   x is 4, 5, or 6 AND y is true"),
        _ => println!("   Other case"),
    }
    println!();
}

// if let and while let
fn demonstrate_if_let() {
    println!("10. if let and while let");
    println!("   Concise syntax for matching one pattern\n");

    // if let: match one pattern, ignore others
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("   Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("   Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("   Using purple as the background color");
        } else {
            println!("   Using orange as the background color");
        }
    } else {
        println!("   Using blue as the background color");
    }
    println!();

    // while let: loop while pattern matches
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("   Popping from stack:");
    while let Some(top) = stack.pop() {
        println!("   {}", top);
    }
    println!("   → Stack is now empty\n");

    // for loops also use patterns
    let v = vec!['a', 'b', 'c'];
    println!("   Iterating with pattern matching:");
    for (index, value) in v.iter().enumerate() {
        println!("   v[{}] = {}", index, value);
    }
    println!();
}

// Refutable vs Irrefutable patterns
fn demonstrate_refutable_patterns() {
    println!("11. Refutable vs Irrefutable Patterns");
    println!("   Patterns that can fail vs patterns that always match\n");

    // Irrefutable: always matches (used in let, function parameters)
    let x = 5; // This always works - irrefutable
    println!("   let x = 5;  // Irrefutable - always matches");
    println!("   → x = {}\n", x);

    // Refutable: might not match (used in if let, while let, match)
    let some_option: Option<i32> = Some(5);

    // This is refutable - might not match
    if let Some(x) = some_option {
        println!("   if let Some(x) = some_option {{");
        println!("       // This pattern is refutable");
        println!("       // x = {}", x);
        println!("   }}\n");
    }

    // Function parameters are irrefutable
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("   Function parameter pattern: &(x, y)");
        println!("   → Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
    println!();

    println!("   Key points:");
    println!("   - let statements require irrefutable patterns");
    println!("   - if let and while let accept refutable patterns");
    println!("   - match arms must be refutable (except _)\n");
}
