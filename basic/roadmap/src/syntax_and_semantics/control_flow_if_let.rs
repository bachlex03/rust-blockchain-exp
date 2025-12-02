// Define types for examples
#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::California => year >= 1850,
            UsState::Texas => year >= 1845,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("=== if let and let else Control Flow ===\n");

    // 1. Basic if let
    demonstrate_basic_if_let();

    // 2. if let vs match
    demonstrate_if_let_vs_match();

    // 3. if let with else
    demonstrate_if_let_else();

    // 4. let else syntax
    demonstrate_let_else();

    // 5. Practical examples
    demonstrate_practical_examples();
}

// 1. Basic if let
fn demonstrate_basic_if_let() {
    println!("1. Basic if let");
    println!("   Concise syntax for matching one pattern\n");

    let config_max = Some(3u8);

    // Using match (verbose)
    println!("   With match:");
    match config_max {
        Some(max) => println!("   The maximum is configured to be {}", max),
        _ => (),
    }

    // Using if let (concise)
    println!("\n   With if let:");
    if let Some(max) = config_max {
        println!("   The maximum is configured to be {}", max);
    }

    println!("\n   if let is syntax sugar for a match with one pattern\n");
}

// 2. if let vs match
fn demonstrate_if_let_vs_match() {
    println!("2. if let vs match");
    println!("   Trade-offs between conciseness and exhaustiveness\n");

    let number = Some(7);

    // match: exhaustive, must handle all cases
    println!("   match (exhaustive):");
    match number {
        Some(n) if n < 5 => println!("   Small number: {}", n),
        Some(n) => println!("   Large number: {}", n),
        None => println!("   No number"),
    }

    // if let: concise, only handles one case
    println!("\n   if let (concise, non-exhaustive):");
    if let Some(n) = number {
        println!("   Got a number: {}", n);
    }

    println!("\n   Use match for exhaustive checking");
    println!("   Use if let when you only care about one pattern\n");
}

// 3. if let with else
fn demonstrate_if_let_else() {
    println!("3. if let with else");
    println!("   Handle the non-matching case\n");

    let coin = Coin::Penny;
    let mut count = 0;

    // Using match
    println!("   With match:");
    match coin {
        Coin::Quarter(state) => println!("   State quarter from {:?}!", state),
        _ => {
            count += 1;
            println!("   Not a quarter, count = {}", count);
        }
    }

    // Using if let with else
    let coin2 = Coin::Quarter(UsState::Alaska);
    let mut count2 = 0;

    println!("\n   With if let...else:");
    if let Coin::Quarter(state) = coin2 {
        println!("   State quarter from {:?}!", state);
    } else {
        count2 += 1;
        println!("   Not a quarter, count = {}", count2);
    }
    println!();
}

// 4. let else syntax
fn demonstrate_let_else() {
    println!("4. let else - Staying on the Happy Path");
    println!("   Extract value or return early\n");

    // Example 1: Using if let with early return
    fn describe_quarter_v1(coin: Coin) -> Option<String> {
        let state = if let Coin::Quarter(state) = coin {
            state
        } else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{:?} is pretty old, for America!", state))
        } else {
            Some(format!("{:?} is relatively new.", state))
        }
    }

    // Example 2: Using let else (cleaner)
    fn describe_quarter_v2(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{:?} is pretty old, for America!", state))
        } else {
            Some(format!("{:?} is relatively new.", state))
        }
    }

    println!("   Testing with Quarter(Alaska):");
    if let Some(desc) = describe_quarter_v1(Coin::Quarter(UsState::Alaska)) {
        println!("   if let version: {}", desc);
    }

    if let Some(desc) = describe_quarter_v2(Coin::Quarter(UsState::Alaska)) {
        println!("   let else version: {}", desc);
    }

    println!("\n   Testing with Penny:");
    if let Some(desc) = describe_quarter_v1(Coin::Penny) {
        println!("   {}", desc);
    } else {
        println!("   Not a quarter, no description");
    }

    println!("\n   let else keeps code on the 'happy path'\n");
}

// 5. Practical examples
fn demonstrate_practical_examples() {
    println!("5. Practical Examples\n");

    // Example 1: Parsing configuration
    println!("   Example 1: Configuration parsing");
    let config: Option<&str> = Some("debug");

    if let Some(mode) = config {
        println!("   Running in {} mode", mode);
    } else {
        println!("   Running in default mode");
    }

    // Example 2: Nested if let
    println!("\n   Example 2: Nested if let");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("   Using favorite color: {}", color);
    } else if is_tuesday {
        println!("   Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("   Using purple (age > 30)");
        } else {
            println!("   Using orange (age <= 30)");
        }
    } else {
        println!("   Using blue as default");
    }

    // Example 3: while let for iteration
    println!("\n   Example 3: while let for iteration");
    let mut stack = vec![1, 2, 3];

    println!("   Popping from stack:");
    while let Some(top) = stack.pop() {
        println!("   {}", top);
    }
    println!("   Stack is empty");

    // Example 4: let else with validation
    println!("\n   Example 4: let else with validation");
    fn process_number(input: &str) -> Result<i32, String> {
        let Ok(num) = input.parse::<i32>() else {
            return Err(format!("'{}' is not a valid number", input));
        };

        let result = num * 2;
        Ok(result)
    }

    match process_number("42") {
        Ok(n) => println!("   Result: {}", n),
        Err(e) => println!("   Error: {}", e),
    }

    match process_number("abc") {
        Ok(n) => println!("   Result: {}", n),
        Err(e) => println!("   Error: {}", e),
    }

    println!();
}