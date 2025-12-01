fn main() {
    println!("=== Control Flow in Rust ===\n");

    // 1. if expressions
    demonstrate_if_expressions();

    // 2. else if
    demonstrate_else_if();

    // 3. Using if in let statements
    demonstrate_if_in_let();

    // 4. loop (infinite loops)
    demonstrate_loop();

    // 5. Returning values from loops
    demonstrate_loop_return();

    // 6. Loop labels
    demonstrate_loop_labels();

    // 7. while loops
    demonstrate_while();

    // 8. for loops
    demonstrate_for();

    // 9. Ranges
    demonstrate_ranges();
}

// if expressions: branch code based on conditions
fn demonstrate_if_expressions() {
    println!("1. if Expressions");
    println!("   Condition must be a bool (not like JavaScript/Python)\n");

    let number = 3;

    if number < 5 {
        println!("   number < 5: condition was true");
    } else {
        println!("   number < 5: condition was false");
    }

    // Rust requires explicit boolean - this won't compile:
    // if number { ... }  // ERROR: expected bool, found integer

    // Must be explicit:
    if number != 0 {
        println!("   number != 0: number is not zero\n");
    }

    // Without else
    let value = 7;
    if value > 5 {
        println!("   value > 5: condition was true (no else needed)\n");
    }
}

// Handling multiple conditions with else if
fn demonstrate_else_if() {
    println!("2. else if - Multiple Conditions");
    println!("   Check multiple conditions in sequence\n");

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

    println!("   → Only the FIRST true condition executes\n");
}

// Using if in a let statement (if is an expression!)
fn demonstrate_if_in_let() {
    println!("3. Using if in let Statements");
    println!("   if is an expression, so it can return a value\n");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("   let number = if condition {{ 5 }} else {{ 6 }};");
    println!("   → number = {}\n", number);

    // Both arms must return the same type
    // This would cause an error:
    // let number = if condition { 5 } else { "six" };  // ERROR!

    // Blocks evaluate to the last expression
    let result = if condition {
        let x = 10;
        x * 2 // This is the value returned (no semicolon!)
    } else {
        0
    };
    println!("   Block example: result = {}\n", result);
}

// loop: infinite loop until break
fn demonstrate_loop() {
    println!("4. loop - Infinite Loops");
    println!("   Loop until you explicitly break\n");

    let mut counter = 0;

    loop {
        counter += 1;
        println!("   Counter: {}", counter);

        if counter >= 3 {
            break; // Exit the loop
        }
    }

    println!("   → Loop exited with break\n");
}

// Returning values from loops
fn demonstrate_loop_return() {
    println!("5. Returning Values from Loops");
    println!("   Use break with a value to return from loop\n");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 2; // Return this value
        }
    };

    println!("   let result = loop {{ ... break counter * 2; }};");
    println!("   → result = {} (returned from loop)\n", result);
}

// Loop labels: disambiguate between nested loops
fn demonstrate_loop_labels() {
    println!("6. Loop Labels");
    println!("   Label loops to break/continue outer loops\n");

    let mut count = 0;
    'counting_up: loop {
        println!("   Outer loop: count = {}", count);
        let mut remaining = 3;

        loop {
            println!("     Inner loop: remaining = {}", remaining);
            if remaining == 2 {
                break; // Breaks inner loop only
            }
            if count == 2 {
                break 'counting_up; // Breaks outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("   → End count = {}\n", count);
}

// while: conditional loop
fn demonstrate_while() {
    println!("7. while - Conditional Loops");
    println!("   Loop while condition is true\n");

    let mut number = 3;

    while number != 0 {
        println!("   {}!", number);
        number -= 1;
    }

    println!("   LIFTOFF!!!\n");

    // while loop through array (less safe)
    println!("   Looping through array with while:");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("   a[{}] = {}", index, a[index]);
        index += 1;
    }
    println!("   → Note: Easy to make mistakes with index bounds\n");
}

// for: safest way to loop through collections
fn demonstrate_for() {
    println!("8. for - Looping Through Collections");
    println!("   Safest and most idiomatic way to iterate\n");

    let a = [10, 20, 30, 40, 50];

    println!("   for element in a:");
    for element in a {
        println!("   the value is: {}", element);
    }
    println!();

    // Using iter() explicitly
    println!("   for element in a.iter():");
    for element in a.iter() {
        println!("   the value is: {}", element);
    }
    println!();

    // With index using enumerate
    println!("   for (index, element) in a.iter().enumerate():");
    for (index, element) in a.iter().enumerate() {
        println!("   a[{}] = {}", index, element);
    }
    println!();
}

// Ranges: generating sequences of numbers
fn demonstrate_ranges() {
    println!("9. Ranges - Generating Number Sequences");
    println!("   Use ranges with for loops\n");

    // Range: start..end (exclusive end)
    println!("   Range (1..4) - exclusive end:");
    for number in 1..4 {
        print!("   {} ", number);
    }
    println!("\n   → Prints: 1, 2, 3 (not 4)\n");

    // Range: start..=end (inclusive end)
    println!("   Range (1..=4) - inclusive end:");
    for number in 1..=4 {
        print!("   {} ", number);
    }
    println!("\n   → Prints: 1, 2, 3, 4\n");

    // Countdown using rev()
    println!("   Countdown using (1..4).rev():");
    for number in (1..4).rev() {
        println!("   {}!", number);
    }
    println!("   LIFTOFF!!!\n");

    // Practical example: iterate array indices
    let arr = [100, 200, 300];
    println!("   Iterating array indices with range:");
    for i in 0..arr.len() {
        println!("   arr[{}] = {}", i, arr[i]);
    }
}
