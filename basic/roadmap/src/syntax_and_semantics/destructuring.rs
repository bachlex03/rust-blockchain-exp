// Define types for destructuring examples
#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u8, u8, u8),
    HSV(u8, u8, u8),
    CMYK(u8, u8, u8, u8),
}

#[allow(dead_code)]
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn main() {
    println!("=== Destructuring in Rust ===\n");

    // 1. Destructuring tuples
    demonstrate_tuple_destructuring();

    // 2. Destructuring arrays and slices
    demonstrate_array_slice_destructuring();

    // 3. Destructuring enums
    demonstrate_enum_destructuring();

    // 4. Destructuring pointers/references
    demonstrate_pointer_destructuring();

    // 5. Destructuring structures
    demonstrate_struct_destructuring();
}

// 1. Destructuring Tuples
fn demonstrate_tuple_destructuring() {
    println!("1. Destructuring Tuples");
    println!("   Extract values from tuples\n");

    let triple = (0, -2, 3);

    match triple {
        (0, y, z) => println!("   First is 0, y = {}, z = {}", y, z),
        (1, ..) => println!("   First is 1 and the rest doesn't matter"),
        (.., 2) => println!("   Last is 2 and the rest doesn't matter"),
        (3, .., 4) => println!("   First is 3, last is 4, middle doesn't matter"),
        _ => println!("   It doesn't matter what they are"),
    }

    // Nested tuple destructuring
    let nested = ((1, 2), (3, 4));
    match nested {
        ((a, b), (c, d)) => {
            println!("   Nested tuple: a={}, b={}, c={}, d={}", a, b, c, d);
        }
    }
    println!();
}

// 2. Destructuring Arrays and Slices
fn demonstrate_array_slice_destructuring() {
    println!("2. Destructuring Arrays and Slices");
    println!("   Pattern match on arrays and slices\n");

    let array = [1, -2, 6];

    match array {
        [0, second, third] => {
            println!("   array[0] = 0, array[1] = {}, array[2] = {}", second, third);
        }
        [1, _, third] => println!("   array[0] = 1, array[2] = {} (middle ignored)", third),
        [-1, second, ..] => {
            println!("   array[0] = -1, array[1] = {} (rest ignored)", second);
        }
        [first, middle @ .., last] => {
            println!(
                "   array[0] = {}, middle = {:?}, array[last] = {}",
                first, middle, last
            );
        }
    }

    // Slice destructuring with different patterns
    fn match_slice(slice: &[i32]) {
        match slice {
            [] => println!("   Empty slice"),
            [first] => println!("   Single element: {}", first),
            [first, second] => println!("   Two elements: {}, {}", first, second),
            [first, second, ..] => {
                println!("   Slice first two: {}, {} (and more)", first, second);
            }
        }
    }

    match_slice(&[1, 2, 3, 4, 5]);
    match_slice(&[10, 20]);
    match_slice(&[]);
    println!();
}

// 3. Destructuring Enums
fn demonstrate_enum_destructuring() {
    println!("3. Destructuring Enums");
    println!("   Extract data from enum variants\n");

    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red => println!("   The color is Red!"),
        Color::Blue => println!("   The color is Blue!"),
        Color::Green => println!("   The color is Green!"),
        Color::RGB(r, g, b) => {
            println!("   Red: {}, green: {}, blue: {}", r, g, b);
        }
        Color::HSV(h, s, v) => {
            println!("   Hue: {}, saturation: {}, value: {}", h, s, v);
        }
        Color::CMYK(c, m, y, k) => {
            println!("   Cyan: {}, magenta: {}, yellow: {}, key: {}", c, m, y, k);
        }
    }

    // Destructuring enum with different variant types
    let event = WebEvent::Click { x: 20, y: 80 };

    match event {
        WebEvent::PageLoad => println!("   Page loaded"),
        WebEvent::PageUnload => println!("   Page unloaded"),
        WebEvent::KeyPress(c) => println!("   Pressed '{}'", c),
        WebEvent::Paste(s) => println!("   Pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("   Clicked at x={}, y={}", x, y);
        }
    }
    println!();
}

// 4. Destructuring Pointers/References
fn demonstrate_pointer_destructuring() {
    println!("4. Destructuring Pointers/References");
    println!("   Dereference and match references\n");

    let reference = &4;

    match reference {
        &val => println!("   Got a value via destructuring: {:?}", val),
    }

    // Dereferencing vs destructuring
    match *reference {
        val => println!("   Got a value via dereferencing: {:?}", val),
    }

    // ref creates a reference in patterns
    let value = 5;
    match value {
        ref r => println!("   Got a reference to a value: {:?}", r),
    }

    // ref mut for mutable references
    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("   We added 10. `mut_value`: {:?}", m);
        }
    }
    println!("   Final mut_value: {}\n", mut_value);
}

// 5. Destructuring Structures
fn demonstrate_struct_destructuring() {
    println!("5. Destructuring Structures");
    println!("   Extract fields from structs\n");

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => {
            println!("   First of x is 1, b = {}, y = {}", b, y);
        }
        Foo { y: 2, x: i } => {
            println!("   y is 2, i = {:?}", i);
        }
        Foo { y, .. } => {
            println!("   y = {}, we don't care about x", y);
        }
    }

    // Destructuring with let
    let Foo { x: (a, b), y } = foo;
    println!("   Destructured with let: a={}, b={}, y={}", a, b, y);
    println!();
}