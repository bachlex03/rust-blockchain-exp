fn main() {
    println!("=== References, Borrowing, and Slices ===\n");

    // 1. Basic references (immutable borrowing)
    demonstrate_basic_references();

    // 2. Mutable references
    demonstrate_mutable_references();

    // 3. Rules of references
    demonstrate_reference_rules();

    // 4. Dangling references prevention
    demonstrate_no_dangling_references();

    // 5. String slices
    demonstrate_string_slices();

    // 6. Array slices
    demonstrate_array_slices();

    // 7. Practical examples
    demonstrate_practical_examples();
}

// 1. Basic references (immutable borrowing)
fn demonstrate_basic_references() {
    println!("1. Basic References (Immutable Borrowing)");
    println!("   References let you refer to a value without taking ownership\n");

    let s1 = String::from("hello");

    // Pass a reference instead of moving ownership
    let len = calculate_length(&s1);

    println!("   The length of '{}' is {}.", s1, len);
    println!("   s1 is still valid after the function call!\n");

    // Multiple immutable references are allowed
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;

    println!("   Multiple immutable references:");
    println!("   r1: {}, r2: {}, r3: {}", r1, r2, r3);
    println!();
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it doesn't own the data, so nothing is dropped

// 2. Mutable references
fn demonstrate_mutable_references() {
    println!("2. Mutable References");
    println!("   Mutable references allow you to modify borrowed data\n");

    let mut s = String::from("hello");
    println!("   Before: {}", s);

    change(&mut s);
    println!("   After: {}", s);

    println!("\n   Note: You can only have ONE mutable reference at a time\n");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 3. Rules of references
fn demonstrate_reference_rules() {
    println!("3. Rules of References");
    println!("   Rust enforces strict borrowing rules at compile time\n");

    let mut s = String::from("hello");

    // Rule 1: Multiple immutable references are OK
    {
        let r1 = &s;
        let r2 = &s;
        println!("   Multiple immutable refs: {} and {}", r1, r2);
    }

    // Rule 2: Only one mutable reference at a time
    {
        let r1 = &mut s;
        r1.push_str("!");
        println!("   One mutable ref: {}", r1);
        // let r2 = &mut s; // Error! Can't have two mutable refs
    }

    // Rule 3: Can't mix mutable and immutable references
    {
        let r1 = &s; // immutable
        let r2 = &s; // immutable
        println!("   Immutable refs: {} and {}", r1, r2);
        // r1 and r2 are no longer used after this point

        let r3 = &mut s; // mutable - OK because r1 and r2 are done
        r3.push_str(" world");
        println!("   Mutable ref: {}", r3);
    }

    println!("\n   Reference Rules:");
    println!("   1. Any number of immutable references");
    println!("   2. OR exactly one mutable reference");
    println!("   3. References must always be valid\n");
}

// 4. Dangling references prevention
fn demonstrate_no_dangling_references() {
    println!("4. No Dangling References");
    println!("   Rust prevents dangling references at compile time\n");

    let reference = no_dangle();
    println!("   Valid reference: {}", reference);

    println!("\n   Rust won't let you return a reference to local data");
    println!("   Instead, return the owned value\n");
}

// This would cause an error if we tried to return &String
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // Error! s will be dropped, leaving a dangling reference
// }

// Correct way: return the owned value
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership is moved out
}

// 5. String slices
fn demonstrate_string_slices() {
    println!("5. String Slices");
    println!("   Slices let you reference a contiguous sequence of elements\n");

    let s = String::from("hello world");

    // Create slices
    let hello = &s[0..5]; // or &s[..5]
    let world = &s[6..11]; // or &s[6..]
    let whole = &s[..]; // entire string

    println!("   Original: {}", s);
    println!("   First word: {}", hello);
    println!("   Second word: {}", world);
    println!("   Whole string: {}", whole);

    // Using first_word function
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("\n   First word using function: {}", word);

    // String literals are slices
    let s = "Hello, world!"; // type is &str
    println!("   String literal (already a slice): {}", s);

    println!();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 6. Array slices
fn demonstrate_array_slices() {
    println!("6. Array Slices");
    println!("   Slices work with arrays too\n");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("   Original array: {:?}", a);
    println!("   Slice [1..3]: {:?}", slice);
    println!("   Slice type: &[i32]\n");

    // Using slices in functions
    let sum = sum_slice(&a[..]);
    println!("   Sum of entire array: {}", sum);

    let sum = sum_slice(&a[1..4]);
    println!("   Sum of slice [1..4]: {}", sum);

    println!();
}

fn sum_slice(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &item in slice {
        total += item;
    }
    total
}

// 7. Practical examples
fn demonstrate_practical_examples() {
    println!("7. Practical Examples\n");

    // Example 1: Avoiding unnecessary clones
    println!("   Example 1: Efficient string processing");
    let text = String::from("The quick brown fox");
    let word_count = count_words(&text);
    println!("   '{}' has {} words", text, word_count);
    println!("   (text is still usable because we borrowed it)\n");

    // Example 2: Modifying data in place
    println!("   Example 2: Modifying data in place");
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("   Before: {:?}", numbers);
    double_values(&mut numbers);
    println!("   After doubling: {:?}", numbers);
    println!();

    // Example 3: Safe string manipulation
    println!("   Example 3: Safe string manipulation");
    let mut message = String::from("Hello");
    append_exclamation(&mut message);
    println!("   Message: {}", message);
    println!();

    // Example 4: Working with slices
    println!("   Example 4: Finding longest word");
    let sentence = "Rust is a systems programming language";
    let longest = find_longest_word(sentence);
    println!("   Longest word in '{}': {}", sentence, longest);
    println!();
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn double_values(vec: &mut Vec<i32>) {
    for item in vec.iter_mut() {
        *item *= 2;
    }
}

fn append_exclamation(s: &mut String) {
    s.push_str("!!!");
}

fn find_longest_word(s: &str) -> &str {
    let mut longest = "";
    for word in s.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest
}