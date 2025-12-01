fn main() {
    println!("=== Tuple Type in Rust ===\n");

    // 1. Creating tuples
    demonstrate_creating_tuples();

    // 2. Destructuring tuples
    demonstrate_destructuring();

    // 3. Accessing tuple elements by index
    demonstrate_index_access();

    // 4. Tuples with different types
    demonstrate_mixed_types();

    // 5. The unit type
    demonstrate_unit_type();
}

// Tuples group multiple values of different types into one compound type
fn demonstrate_creating_tuples() {
    println!("1. Creating Tuples");
    println!("   Tuples have a fixed length and can contain different types\n");

    // Tuple with type annotation
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("   With type annotation:");
    println!("   let tup: (i32, f64, u8) = (500, 6.4, 1);");
    println!(
        "   → tup contains: integer {}, float {}, unsigned {}\n",
        tup.0, tup.1, tup.2
    );

    // Tuple without type annotation (type inference)
    let tup2 = (42, 3.14, 'A', true);
    println!("   Without type annotation (inferred):");
    println!("   let tup2 = (42, 3.14, 'A', true);");
    println!("   → Types inferred: (i32, f64, char, bool)");
    println!("   → Access: tup2.0 = {}, tup2.2 = '{}'\n", tup2.0, tup2.2);

    // Empty tuple (unit type)
    let _unit = ();
    println!("   Empty tuple (unit type):");
    println!("   let unit = ();");
    println!("   → This is the unit type\n");
}

// Destructuring: breaking a tuple into separate variables
fn demonstrate_destructuring() {
    println!("2. Destructuring Tuples");
    println!("   Use pattern matching to extract values from tuples\n");

    // Create a tuple
    let tup = (500, 6.4, 1);
    println!("   let tup = (500, 6.4, 1);");

    // Destructure into separate variables
    let (x, y, z) = tup;
    println!("   let (x, y, z) = tup;");
    println!("   → x = {}, y = {}, z = {}\n", x, y, z);

    // Destructuring with different variable names
    let coordinates = (10, 20, 30);
    let (x_coord, y_coord, z_coord) = coordinates;
    println!("   Coordinates example:");
    println!("   let coordinates = (10, 20, 30);");
    println!("   let (x_coord, y_coord, z_coord) = coordinates;");
    println!("   → x: {}, y: {}, z: {}\n", x_coord, y_coord, z_coord);

    // Partial destructuring (using _ to ignore values)
    let person = ("Alice", 30, true);
    let (name, age, _) = person;
    println!("   Partial destructuring (ignoring some values):");
    println!("   let person = (\"Alice\", 30, true);");
    println!("   let (name, age, _) = person;");
    println!("   → name: {}, age: {} (ignored the boolean)\n", name, age);
}

// Accessing tuple elements directly by index
fn demonstrate_index_access() {
    println!("3. Accessing Tuple Elements by Index");
    println!("   Use a period (.) followed by the index (starting at 0)\n");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("   let x: (i32, f64, u8) = (500, 6.4, 1);\n");

    // Access each element by index
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("   Accessing by index:");
    println!("   x.0 = {} (first element)", five_hundred);
    println!("   x.1 = {} (second element)", six_point_four);
    println!("   x.2 = {} (third element)\n", one);

    // You can use tuple elements directly in expressions
    let sum = x.0 + x.2 as i32;
    println!("   Using tuple elements in expressions:");
    println!("   x.0 + x.2 = {} + {} = {}\n", x.0, x.2, sum);
}

// Tuples can contain different types
fn demonstrate_mixed_types() {
    println!("4. Tuples with Mixed Types");
    println!("   Each position can have a different type\n");

    // Tuple with various types
    let mixed: (i32, f64, bool, char, &str) = (42, 3.14, true, 'R', "Rust");
    println!("   let mixed: (i32, f64, bool, char, &str) = (42, 3.14, true, 'R', \"Rust\");");
    println!("   → Types: integer, float, boolean, character, string slice");
    println!(
        "   → Access: mixed.0 = {}, mixed.4 = \"{}\"\n",
        mixed.0, mixed.4
    );

    // Nested tuples
    let nested: ((i32, i32), (f64, f64)) = ((1, 2), (3.0, 4.0));
    println!("   Nested tuples:");
    println!("   let nested: ((i32, i32), (f64, f64)) = ((1, 2), (3.0, 4.0));");
    println!(
        "   → Access: nested.0.0 = {}, nested.1.1 = {}\n",
        nested.0.0, nested.1.1
    );

    // Practical example: returning multiple values from a function
    let (result, remainder) = divide_with_remainder(17, 5);
    println!("   Practical example: Function returning tuple");
    println!("   divide_with_remainder(17, 5)");
    println!("   → result: {}, remainder: {}\n", result, remainder);
}

// Helper function that returns a tuple
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

// The unit type: empty tuple
fn demonstrate_unit_type() {
    println!("5. The Unit Type");
    println!("   The empty tuple () is called 'unit'\n");

    // Unit type represents an empty value
    let _unit_value = ();
    println!("   let unit_value = ();");
    println!("   → This is the unit type and unit value\n");

    // Functions that don't return a value implicitly return unit
    let result = print_message();
    println!("   Functions without return value:");
    println!("   fn print_message() {{ println!(\"Hello\"); }}");
    println!("   → Returns: {:?}\n", result);

    // Explicit unit return
    fn explicit_unit() -> () {
        println!("      This function explicitly returns unit");
    }
    let _ = explicit_unit();
}

// Function that implicitly returns unit
fn print_message() {
    println!("      Hello from a function!");
}
