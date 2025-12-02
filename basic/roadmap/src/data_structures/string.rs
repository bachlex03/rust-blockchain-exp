// String in Rust
// A UTF-8 encoded, growable string stored on the heap
// String vs &str: String is owned, &str is a borrowed string slice

pub fn string_examples() {
    println!("\n=== String Examples ===\n");

    // Creating strings
    creating_strings();
    
    // String operations
    string_operations();
    
    // String slicing
    string_slicing();
    
    // Iterating over strings
    iterating_strings();
    
    // String concatenation
    string_concatenation();
    
    // String methods
    string_methods();
    
    // UTF-8 encoding
    utf8_examples();
}

fn creating_strings() {
    println!("--- Creating Strings ---");
    
    // Create from string literal
    let s1 = String::from("Hello, World!");
    println!("String::from(): {}", s1);
    
    // Using to_string() method
    let s2 = "Hello".to_string();
    println!("to_string(): {}", s2);
    
    // Create empty string
    let mut s3 = String::new();
    s3.push_str("Created empty");
    println!("String::new(): {}", s3);
    
    // With capacity (pre-allocate memory)
    let mut s4 = String::with_capacity(10);
    println!("Capacity before: {}", s4.capacity());
    s4.push_str("Hello");
    println!("String with capacity: {}, capacity: {}", s4, s4.capacity());
    
    // UTF-8 strings
    let hello_ar = String::from("السلام عليكم");
    let hello_ru = String::from("Здравствуйте");
    let hello_ja = String::from("こんにちは");
    println!("Arabic: {}", hello_ar);
    println!("Russian: {}", hello_ru);
    println!("Japanese: {}", hello_ja);
    
    println!();
}

fn string_operations() {
    println!("--- String Operations ---");
    
    // Mutable string
    let mut s = String::from("Hello");
    println!("Original: {}", s);
    
    // push_str - append string slice
    s.push_str(", World");
    println!("After push_str: {}", s);
    
    // push - append single character
    s.push('!');
    println!("After push: {}", s);
    
    // insert - insert at position
    let mut s2 = String::from("Hello World");
    s2.insert(5, ',');
    println!("After insert: {}", s2);
    
    // insert_str - insert string at position
    let mut s3 = String::from("Hello World");
    s3.insert_str(6, "Rust ");
    println!("After insert_str: {}", s3);
    
    // replace - replace pattern
    let s4 = String::from("Hello World World");
    let s5 = s4.replace("World", "Rust");
    println!("After replace: {}", s5);
    
    // remove - remove character at index
    let mut s6 = String::from("Hello!");
    s6.remove(5);
    println!("After remove: {}", s6);
    
    // pop - remove last character
    let mut s7 = String::from("Hello!");
    let popped = s7.pop();
    println!("After pop: {}, popped: {:?}", s7, popped);
    
    // clear - empty the string
    let mut s8 = String::from("Hello");
    s8.clear();
    println!("After clear: '{}', is_empty: {}", s8, s8.is_empty());
    
    println!();
}

fn string_slicing() {
    println!("--- String Slicing ---");
    
    let s = String::from("Hello, World!");
    
    // Slice with range
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("Full string: {}", s);
    println!("Slice [0..5]: {}", hello);
    println!("Slice [7..12]: {}", world);
    
    // Slice from start
    let hello2 = &s[..5];
    println!("Slice [..5]: {}", hello2);
    
    // Slice to end
    let world2 = &s[7..];
    println!("Slice [7..]: {}", world2);
    
    // Full slice
    let full = &s[..];
    println!("Slice [..]: {}", full);
    
    // UTF-8 slicing (be careful with multi-byte characters)
    let russian = String::from("Здравствуйте");
    let slice = &russian[0..4]; // Each Cyrillic char is 2 bytes
    println!("Russian slice [0..4]: {}", slice);
    
    println!();
}

fn iterating_strings() {
    println!("--- Iterating Over Strings ---");
    
    let s = String::from("Hello");
    
    // Iterate over characters
    print!("chars(): ");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();
    
    // Iterate over bytes
    print!("bytes(): ");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();
    
    // Iterate with indices
    print!("char_indices(): ");
    for (i, c) in s.char_indices() {
        print!("({}, {}) ", i, c);
    }
    println!();
    
    // Lines iterator
    let multiline = String::from("Line 1\nLine 2\nLine 3");
    println!("lines():");
    for line in multiline.lines() {
        println!("  {}", line);
    }
    
    // Split iterator
    let csv = String::from("apple,banana,cherry");
    print!("split(','): ");
    for item in csv.split(',') {
        print!("{} ", item);
    }
    println!("\n");
}

fn string_concatenation() {
    println!("--- String Concatenation ---");
    
    // Using + operator
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2; // s1 is moved here
    println!("Using +: {}", s3);
    // println!("{}", s1); // Error: s1 was moved
    
    // Using format! macro
    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = format!("{}, {}!", s4, s5);
    println!("Using format!: {}", s6);
    println!("s4 still valid: {}", s4); // s4 and s5 are still valid
    
    // Multiple concatenations
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let s10 = format!("{}-{}-{}", s7, s8, s9);
    println!("Multiple concat: {}", s10);
    
    println!();
}

fn string_methods() {
    println!("--- String Methods ---");
    
    let s = String::from("  Hello, Rust!  ");
    
    // Length
    println!("len(): {}", s.len());
    println!("is_empty(): {}", s.is_empty());
    
    // Trimming
    println!("trim(): '{}'", s.trim());
    println!("trim_start(): '{}'", s.trim_start());
    println!("trim_end(): '{}'", s.trim_end());
    
    // Case conversion
    let s2 = String::from("Hello World");
    println!("to_lowercase(): {}", s2.to_lowercase());
    println!("to_uppercase(): {}", s2.to_uppercase());
    
    // Checking content
    let s3 = String::from("Hello, Rust!");
    println!("contains('Rust'): {}", s3.contains("Rust"));
    println!("starts_with('Hello'): {}", s3.starts_with("Hello"));
    println!("ends_with('!'): {}", s3.ends_with("!"));
    
    // Finding
    println!("find('Rust'): {:?}", s3.find("Rust"));
    
    // Splitting
    let s4 = String::from("apple,banana,cherry");
    let parts: Vec<&str> = s4.split(',').collect();
    println!("split result: {:?}", parts);
    
    // Repeat
    let s5 = "Ha".repeat(3);
    println!("repeat(3): {}", s5);
    
    // Checking ASCII
    let s6 = String::from("Hello");
    let s7 = String::from("Hello 世界");
    println!("'{}' is_ascii(): {}", s6, s6.is_ascii());
    println!("'{}' is_ascii(): {}", s7, s7.is_ascii());
    
    println!();
}

fn utf8_examples() {
    println!("--- UTF-8 Encoding ---");
    
    // Different languages
    let hello_hi = String::from("नमस्ते");
    println!("Hindi: {}", hello_hi);
    println!("Length in bytes: {}", hello_hi.len());
    println!("Length in chars: {}", hello_hi.chars().count());
    
    // Bytes vs chars vs graphemes
    println!("\nAnalyzing 'नमस्ते':");
    print!("Bytes: ");
    for b in hello_hi.bytes() {
        print!("{} ", b);
    }
    println!();
    
    print!("Chars: ");
    for c in hello_hi.chars() {
        print!("{} ", c);
    }
    println!();
    
    // Emoji example
    let emoji = String::from("Hello 👋 World 🌍");
    println!("\nEmoji string: {}", emoji);
    println!("Byte length: {}", emoji.len());
    println!("Char count: {}", emoji.chars().count());
    
    // Why indexing doesn't work
    let s = String::from("Здравствуйте");
    // let c = s[0]; // Error: cannot index into a string
    let c = s.chars().nth(0); // Correct way
    println!("\nFirst char of 'Здравствуйте': {:?}", c);
    
    println!();
}
