// Array Type in Rust
// A fixed-size collection of elements of the same type
// Arrays are allocated on the stack and have compile-time known size

fn main() {
    array_examples();
}

fn array_examples() {
    println!("\n=== Array Type Examples ===\n");

    // Basic array creation
    basic_arrays();
    
    // Array access and indexing
    array_indexing();
    
    // Mutable arrays
    mutable_arrays();
    
    // Array iteration
    array_iteration();
    
    // Array methods
    array_methods();
    
    // Array slices
    array_slices();
}

fn basic_arrays() {
    println!("--- Basic Arrays ---");
    
    // Array without explicit type
    let numbers = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    
    // Array with explicit type and size
    let typed_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Typed array: {:?}", typed_array);
    
    // Array with default values [value; size]
    let default_array = [3; 5];
    println!("Default array [3; 5]: {:?}", default_array);
    
    // Array of strings
    let months = ["January", "February", "March", "April", "May", "June",
                  "July", "August", "September", "October", "November", "December"];
    println!("Months array length: {}", months.len());
    
    // Empty array
    let empty: [i32; 0] = [];
    println!("Empty array: {:?}", empty);
}

fn array_indexing() {
    println!("\n--- Array Indexing ---");
    
    let colors = ["red", "green", "blue"];
    
    // Access elements by index
    println!("1st color (index 0): {}", colors[0]);
    println!("2nd color (index 1): {}", colors[1]);
    println!("3rd color (index 2): {}", colors[2]);
    
    // Array length
    println!("Array length: {}", colors.len());
    
    // First and last elements
    let numbers = [10, 20, 30, 40, 50];
    if let Some(first) = numbers.first() {
        println!("First element: {}", first);
    }
    if let Some(last) = numbers.last() {
        println!("Last element: {}", last);
    }
    
    // Get element safely
    let index = 2;
    match numbers.get(index) {
        Some(value) => println!("Element at index {}: {}", index, value),
        None => println!("Index {} out of bounds", index),
    }
}

fn mutable_arrays() {
    println!("\n--- Mutable Arrays ---");
    
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Original array: {:?}", numbers);
    
    // Modify individual elements
    numbers[2] = 0;
    println!("After changing index 2: {:?}", numbers);
    
    numbers[0] = 10;
    numbers[4] = 50;
    println!("After changing index 0 and 4: {:?}", numbers);
    
    // Modify using mutable reference
    let mut data = [1, 2, 3, 4, 5];
    for item in &mut data {
        *item *= 2;
    }
    println!("After doubling all elements: {:?}", data);
}

fn array_iteration() {
    println!("\n--- Array Iteration ---");
    
    let colors = ["red", "green", "blue"];
    
    // Iterate with index using range
    println!("Using index range:");
    for index in 0..colors.len() {
        println!("  Index {}: {}", index, colors[index]);
    }
    
    // Iterate by reference
    println!("\nUsing for..in (by reference):");
    for color in &colors {
        println!("  Color: {}", color);
    }
    
    // Iterate with enumerate
    println!("\nUsing enumerate:");
    for (index, color) in colors.iter().enumerate() {
        println!("  Index {}: {}", index, color);
    }
    
    // Iterate and modify (mutable)
    let mut numbers = [1, 2, 3, 4, 5];
    println!("\nOriginal: {:?}", numbers);
    for num in &mut numbers {
        *num += 10;
    }
    println!("After adding 10: {:?}", numbers);
}

fn array_methods() {
    println!("\n--- Array Methods ---");
    
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6];
    
    // Length
    println!("Length: {}", numbers.len());
    
    // Check if empty
    println!("Is empty: {}", numbers.is_empty());
    
    // Contains
    println!("Contains 5: {}", numbers.contains(&5));
    println!("Contains 10: {}", numbers.contains(&10));
    
    // Map (transform array)
    let squared = numbers.map(|x| x * x);
    println!("Original: {:?}", numbers);
    println!("Squared: {:?}", squared);
    
    // Reverse
    let mut rev = [1, 2, 3, 4, 5];
    rev.reverse();
    println!("Reversed: {:?}", rev);
    
    // Sort
    let mut unsorted = [5, 2, 8, 1, 9];
    unsorted.sort();
    println!("Sorted: {:?}", unsorted);
    
    // Split at index
    let arr = [1, 2, 3, 4, 5];
    let (left, right) = arr.split_at(2);
    println!("Split at 2: left={:?}, right={:?}", left, right);
}

fn array_slices() {
    println!("\n--- Array Slices ---");
    
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Full slice
    let full_slice = &numbers[..];
    println!("Full slice: {:?}", full_slice);
    
    // Slice from start
    let first_three = &numbers[..3];
    println!("First 3 elements: {:?}", first_three);
    
    // Slice to end
    let from_index_5 = &numbers[5..];
    println!("From index 5: {:?}", from_index_5);
    
    // Slice range
    let middle = &numbers[3..7];
    println!("Middle slice [3..7]: {:?}", middle);
    
    // Convert slice to array (if size matches)
    let slice = &numbers[0..5];
    if let Ok(arr) = <&[i32; 5]>::try_from(slice) {
        println!("Converted slice to array: {:?}", arr);
    }
}