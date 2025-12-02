// Character Type in Rust
// A char represents a Unicode scalar value (4 bytes)
// Range: U+0000 to U+D7FF and U+E000 to U+10FFFF

#[allow(dead_code)]
pub fn character_examples() {
    println!("\n=== Character Type Examples ===\n");

    // Basic character declarations
    basic_chars();
    
    // Unicode and special characters
    unicode_chars();
    
    // Character methods
    char_methods();
    
    // Character encoding
    char_encoding();
    
    // ASCII operations
    ascii_operations();
}

fn basic_chars() {
    println!("--- Basic Characters ---");
    
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    
    println!("Simple char: {}", c);
    println!("Unicode char: {}", z);
    println!("Emoji char: {}", heart_eyed_cat);
    
    // Size of char is always 4 bytes
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}

fn unicode_chars() {
    println!("\n--- Unicode Characters ---");
    
    // Various Unicode characters
    let accented = 'é';
    let chinese = '中';
    let japanese = 'あ';
    let korean = '한';
    let emoji = '🦀'; // Rust crab!
    
    println!("Accented: {}", accented);
    println!("Chinese: {}", chinese);
    println!("Japanese: {}", japanese);
    println!("Korean: {}", korean);
    println!("Emoji: {}", emoji);
    
    // Unicode scalar value ranges
    println!("\nValid Unicode ranges:");
    println!("U+0000 to U+D7FF and U+E000 to U+10FFFF");
}

fn char_methods() {
    println!("\n--- Character Methods ---");
    
    let letter = 'A';
    let digit = '5';
    let space = ' ';
    
    // Check if alphabetic
    println!("'{}' is alphabetic: {}", letter, letter.is_alphabetic());
    println!("'{}' is alphabetic: {}", digit, digit.is_alphabetic());
    
    // Check if numeric
    println!("'{}' is numeric: {}", digit, digit.is_numeric());
    println!("'{}' is numeric: {}", letter, letter.is_numeric());
    
    // Check if alphanumeric
    println!("'{}' is alphanumeric: {}", letter, letter.is_alphanumeric());
    println!("'{}' is alphanumeric: {}", space, space.is_alphanumeric());
    
    // Check if whitespace
    println!("' ' is whitespace: {}", space.is_whitespace());
    
    // Case conversion
    println!("'{}' to lowercase: {}", letter, letter.to_lowercase());
    println!("'a' to uppercase: {}", 'a'.to_uppercase());
    
    // Check if digit in radix
    println!("'5' is digit in radix 10: {}", digit.is_digit(10));
    println!("'F' is digit in radix 16: {}", 'F'.is_digit(16));
}

fn char_encoding() {
    println!("\n--- Character Encoding ---");
    
    let c = '🦀';
    
    // Encode to UTF-8
    let mut utf8_buf = [0u8; 4];
    let utf8_str = c.encode_utf8(&mut utf8_buf);
    println!("'{}' encoded to UTF-8: {:?}", c, utf8_str.as_bytes());
    
    // Encode to UTF-16
    let mut utf16_buf = [0u16; 2];
    let utf16_slice = c.encode_utf16(&mut utf16_buf);
    println!("'{}' encoded to UTF-16: {:?}", c, utf16_slice);
    
    // Get Unicode code point
    println!("'{}' as u32: U+{:X}", c, c as u32);
    
    // Create char from u32
    if let Some(ch) = char::from_u32(0x1F980) {
        println!("char from U+1F980: {}", ch);
    }
}

fn ascii_operations() {
    println!("\n--- ASCII Operations ---");
    
    let ascii_char = 'A';
    let non_ascii = '中';
    
    // Check if ASCII
    println!("'{}' is ASCII: {}", ascii_char, ascii_char.is_ascii());
    println!("'{}' is ASCII: {}", non_ascii, non_ascii.is_ascii());
    
    // ASCII specific checks
    println!("'A' is ASCII alphabetic: {}", ascii_char.is_ascii_alphabetic());
    println!("'5' is ASCII digit: {}", '5'.is_ascii_digit());
    println!("'a' is ASCII lowercase: {}", 'a'.is_ascii_lowercase());
    println!("'A' is ASCII uppercase: {}", ascii_char.is_ascii_uppercase());
    println!("'!' is ASCII punctuation: {}", '!'.is_ascii_punctuation());
    println!("' ' is ASCII whitespace: {}", ' '.is_ascii_whitespace());
    
    // ASCII case conversion
    println!("'a' to ASCII uppercase: {}", 'a'.to_ascii_uppercase());
    println!("'A' to ASCII lowercase: {}", ascii_char.to_ascii_lowercase());
}
