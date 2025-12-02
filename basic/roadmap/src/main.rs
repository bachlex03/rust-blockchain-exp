
fn main() {
    tuple_examples();
}

#[allow(dead_code)]
fn tuple_examples() {
    println!("\n=== Tuple Type Examples ===\n");

    // Basic tuple operations
    basic_tuples();
    
    // Tuple destructuring
    tuple_destructuring();
    
    // Tuple indexing
    tuple_indexing();
    
    // Mutable tuples
    mutable_tuples();
    
    // Unit type
    unit_type();
    
    // Tuples as return values
    tuple_returns();
    
    // Nested tuples
    nested_tuples();
}

fn basic_tuples() {
    println!("--- Basic Tuples ---");
    
    // Tuple with different types (heterogeneous)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);
    
    // Tuple without type annotation (type inference)
    let person = ("Alice", 30, true);
    println!("Person: {:?}", person);
    
    // Single element tuple (note the comma)
    let single = (42,);
    println!("Single element tuple: {:?}", single);
    
    // Empty tuple (unit type)
    let empty = ();
    println!("Empty tuple (unit): {:?}", empty);
}

fn tuple_destructuring() {
    println!("\n--- Tuple Destructuring ---");
    
    let tup = (500, 6.4, 1);
    
    // Destructure into separate variables
    let (x, y, z) = tup;
    println!("Destructured values: x={}, y={}, z={}", x, y, z);
    
    // Partial destructuring with underscore
    let coordinates = (10, 20, 30);
    let (x, _, z) = coordinates;
    println!("Partial destructure: x={}, z={} (y ignored)", x, z);
    
    // Destructuring in function parameters
    let point = (3.5, 7.2);
    print_point(point);
}

fn print_point((x, y): (f64, f64)) {
    println!("Point coordinates: ({}, {})", x, y);
}

fn tuple_indexing() {
    println!("\n--- Tuple Indexing ---");
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    
    // Access elements by index using dot notation
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    
    println!("First element (x.0): {}", five_hundred);
    println!("Second element (x.1): {}", six_point_four);
    println!("Third element (x.2): {}", one);
    
    // Direct access in expressions
    println!("Sum of first and third: {}", x.0 + x.2 as i32);
}

fn mutable_tuples() {
    println!("\n--- Mutable Tuples ---");
    
    let mut x: (i32, i32) = (1, 2);
    println!("Initial tuple: {:?}", x);
    
    // Modify individual elements
    x.0 = 0;
    x.1 += 5;
    println!("After modification: {:?}", x);
    
    // Mutable tuple with different types
    let mut data = (String::from("Hello"), 42, true);
    println!("Before: {:?}", data);
    
    data.0.push_str(" World");
    data.1 *= 2;
    data.2 = !data.2;
    println!("After: {:?}", data);
}

fn unit_type() {
    println!("\n--- Unit Type ---");
    
    // The unit type () represents an empty value
    let unit = ();
    println!("Unit type: {:?}", unit);
    println!("Size of unit: {} bytes", std::mem::size_of_val(&unit));
    
    // Functions without return value implicitly return ()
    let result = returns_unit();
    println!("Function returning unit: {:?}", result);
    
    // Expressions that don't return a value return ()
    let x = if true { println!("This returns unit"); };
    println!("If expression result: {:?}", x);
}

fn returns_unit() {
    println!("This function returns unit type");
    // No explicit return, so returns ()
}

fn tuple_returns() {
    println!("\n--- Tuples as Return Values ---");
    
    // Return multiple values from a function
    let (sum, product) = calculate(5, 10);
    println!("Sum: {}, Product: {}", sum, product);
    
    // Return complex data
    let user = get_user_info();
    println!("User: name={}, age={}, active={}", user.0, user.1, user.2);
    
    // Swap values using tuple
    let (a, b) = (1, 2);
    let (a, b) = (b, a); // Swap
    println!("After swap: a={}, b={}", a, b);
}

fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

fn get_user_info() -> (String, u32, bool) {
    (String::from("Bob"), 25, true)
}

fn nested_tuples() {
    println!("\n--- Nested Tuples ---");
    
    // Tuple containing tuples
    let nested = ((1, 2), (3, 4), (5, 6));
    println!("Nested tuple: {:?}", nested);
    
    // Access nested elements
    println!("First inner tuple: {:?}", nested.0);
    println!("First element of first tuple: {}", nested.0.0);
    println!("Second element of third tuple: {}", nested.2.1);
    
    // Destructure nested tuples
    let ((a, b), (c, d), (e, f)) = nested;
    println!("Destructured: a={}, b={}, c={}, d={}, e={}, f={}", a, b, c, d, e, f);
    
    // Mixed nesting
    let complex = (1, (2.5, "hello"), vec![1, 2, 3]);
    println!("Complex nested tuple: {:?}", complex);
    println!("String from nested tuple: {}", complex.1.1);
}
