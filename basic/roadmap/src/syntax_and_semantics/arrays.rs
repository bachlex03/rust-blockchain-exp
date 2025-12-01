fn main() {
    println!("=== Array Type in Rust ===\n");

    // 1. Creating arrays
    demonstrate_creating_arrays();

    // 2. Array type syntax
    demonstrate_array_type_syntax();

    // 3. Initializing arrays with same value
    demonstrate_array_initialization();

    // 4. Accessing array elements
    demonstrate_accessing_elements();

    // 5. Arrays vs tuples
    demonstrate_arrays_vs_tuples();

    // 6. Practical example: months
    demonstrate_months_example();

    // 7. Array bounds checking
    demonstrate_bounds_checking();
}

// Arrays: fixed-length collection of same type elements
fn demonstrate_creating_arrays() {
    println!("1. Creating Arrays");
    println!("   Arrays have fixed length and all elements must be the same type\n");

    // Basic array creation
    let a = [1, 2, 3, 4, 5];
    println!("   let a = [1, 2, 3, 4, 5];");
    println!("   → Array with 5 elements, type inferred as [i32; 5]");
    println!("   → First element: a[0] = {}\n", a[0]);

    // Arrays are allocated on the stack (not heap)
    println!("   Key characteristics:");
    println!("   - Fixed length (cannot grow or shrink)");
    println!("   - All elements must be the same type");
    println!("   - Stored on the stack (fast access)");
    println!("   - Useful when you know the size at compile time\n");
}

// Array type syntax: [type; length]
fn demonstrate_array_type_syntax() {
    println!("2. Array Type Syntax");
    println!("   Format: [element_type; length]\n");

    // Explicit type annotation
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("   let a: [i32; 5] = [1, 2, 3, 4, 5];");
    println!("   → [i32; 5] means: array of i32, with 5 elements");
    println!("   → Example access: a[0] = {}\n", a[0]);

    // Different types
    let floats: [f64; 3] = [1.1, 2.2, 3.3];
    println!("   let floats: [f64; 3] = [1.1, 2.2, 3.3];");
    println!("   → Array of f64 with 3 elements");
    println!("   → Example access: floats[1] = {}\n", floats[1]);

    let chars: [char; 4] = ['R', 'u', 's', 't'];
    println!("   let chars: [char; 4] = ['R', 'u', 's', 't'];");
    println!("   → Array of char with 4 elements");
    println!("   → Example access: chars[0] = '{}'\n", chars[0]);
}

// Initialize array with same value for all elements
fn demonstrate_array_initialization() {
    println!("3. Initializing Arrays with Same Value");
    println!("   Syntax: [value; length]\n");

    // Create array with 5 elements, all set to 3
    let a = [3; 5];
    println!("   let a = [3; 5];");
    println!("   → Creates: [3, 3, 3, 3, 3]");
    println!("   → Equivalent to: let a = [3, 3, 3, 3, 3];");
    println!("   → Example: a[0] = {}, a[4] = {}\n", a[0], a[4]);

    // More examples
    let zeros: [i32; 10] = [0; 10];
    println!("   let zeros: [i32; 10] = [0; 10];");
    println!("   → Creates array of 10 zeros");
    println!(
        "   → Example: zeros[0] = {}, zeros[9] = {}\n",
        zeros[0], zeros[9]
    );

    let trues = [true; 4];
    println!("   let trues = [true; 4];");
    println!("   → Creates: [true, true, true, true]");
    println!(
        "   → Example: trues[0] = {}, trues[3] = {}\n",
        trues[0], trues[3]
    );
}

// Accessing array elements using indexing
fn demonstrate_accessing_elements() {
    println!("4. Accessing Array Elements");
    println!("   Use square brackets with index: array[index]\n");

    let a = [1, 2, 3, 4, 5];
    println!("   let a = [1, 2, 3, 4, 5];\n");

    // Access elements by index (starting at 0)
    let first = a[0];
    let second = a[1];
    let last = a[4];

    println!("   Accessing elements:");
    println!("   a[0] = {} (first element)", first);
    println!("   a[1] = {} (second element)", second);
    println!("   a[4] = {} (last element)\n", last);

    // Using array elements in expressions
    let sum = a[0] + a[1] + a[2];
    println!("   Using in expressions:");
    println!(
        "   a[0] + a[1] + a[2] = {} + {} + {} = {}\n",
        a[0], a[1], a[2], sum
    );

    // Iterating over array
    println!("   Iterating over array:");
    print!("   Elements: ");
    for element in a.iter() {
        print!("{} ", element);
    }
    println!("\n");
}

// Arrays vs tuples
fn demonstrate_arrays_vs_tuples() {
    println!("5. Arrays vs Tuples");
    println!("   Key differences:\n");

    // Array: same type, fixed length
    let array: [i32; 3] = [1, 2, 3];
    println!("   Array: [i32; 3] = {:?}", array);
    println!("   → All elements must be the same type (i32)\n");

    // Tuple: different types, fixed length
    let tuple: (i32, f64, char) = (1, 2.0, '3');
    println!("   Tuple: (i32, f64, char) = {:?}", tuple);
    println!("   → Elements can be different types\n");

    // Access syntax
    println!("   Access syntax:");
    println!("   Array: array[0] = {}", array[0]);
    println!("   Tuple: tuple.0 = {}\n", tuple.0);
}

// Practical example: months array
fn demonstrate_months_example() {
    println!("6. Practical Example: Months Array");
    println!("   Arrays are perfect when you know the size won't change\n");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("   let months = [\"January\", \"February\", ...];");
    println!("   → Always 12 months, perfect for an array!\n");

    println!("   Accessing months:");
    println!("   months[0] = {} (first month)", months[0]);
    println!("   months[11] = {} (last month)\n", months[11]);

    // Type of months array
    println!("   Type: [&str; 12]");
    println!("   → Array of string slices, 12 elements\n");
}

// Array bounds checking - Rust's memory safety
fn demonstrate_bounds_checking() {
    println!("7. Array Bounds Checking");
    println!("   Rust checks array bounds at runtime for memory safety\n");

    let a = [1, 2, 3, 4, 5];
    println!("   let a = [1, 2, 3, 4, 5];");
    println!("   → Valid indices: 0, 1, 2, 3, 4\n");

    // Valid access
    let valid = a[2];
    println!("   Valid access: a[2] = {} ✓", valid);

    // Invalid access would cause panic
    println!("\n   Invalid access example:");
    println!("   If you try: a[10]");
    println!("   → Rust will PANIC at runtime:");
    println!("   → 'index out of bounds: the len is 5 but the index is 10'\n");

    // Safe way to access (using get method)
    println!("   Safe access using .get() method:");
    match a.get(2) {
        Some(value) => println!("   a.get(2) = Some({}) ✓", value),
        None => println!("   a.get(2) = None (out of bounds)"),
    }

    match a.get(10) {
        Some(value) => println!("   a.get(10) = Some({})", value),
        None => println!("   a.get(10) = None (out of bounds) ✓"),
    }

    println!("\n   Memory safety:");
    println!("   - Rust prevents invalid memory access");
    println!("   - Panics instead of allowing undefined behavior");
    println!("   - Use .get() for safe access that returns Option\n");

    // Array length
    println!("   Getting array length:");
    println!("   a.len() = {} (number of elements)", a.len());
}
