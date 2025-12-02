fn main() {
    println!("=== Enums in Rust ===\n");

    // 1. Basic enum definition
    demonstrate_basic_enum();

    // 2. Enums with data
    demonstrate_enum_with_data();

    // 3. Different data types in variants
    demonstrate_different_data_types();

    // 4. Methods on enums
    demonstrate_enum_methods();

    // 5. The Option enum
    demonstrate_option_enum();

    // 6. Working with Option
    demonstrate_working_with_option();
}

// 1. Basic enum definition
fn demonstrate_basic_enum() {
    println!("1. Basic Enum Definition");
    println!("   Enums let you define a type by enumerating its possible variants\n");

    enum IpAddrKind {
        V4,
        V6,
    }

    fn route(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => println!("   Routing IPv4 packet"),
            IpAddrKind::V6 => println!("   Routing IPv6 packet"),
        }
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("   Created two IP address kinds:");
    route(four);
    route(six);

    println!("\n   Variants are namespaced under the enum identifier\n");
}

// 2. Enums with data
fn demonstrate_enum_with_data() {
    println!("2. Enums with Data");
    println!("   Attach data directly to enum variants\n");

    // Old way: using struct
    #[allow(dead_code)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[allow(dead_code)]
    struct IpAddrStruct {
        kind: IpAddrKind,
        address: String,
    }

    println!("   Old way (struct + enum):");
    println!("   struct IpAddr {{ kind: IpAddrKind, address: String }}");

    // Better way: data in enum
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("\n   Better way (data in enum):");
    println!("   enum IpAddr {{ V4(String), V6(String) }}");

    match home {
        IpAddr::V4(addr) => println!("   Home: IPv4 address {}", addr),
        IpAddr::V6(addr) => println!("   Home: IPv6 address {}", addr),
    }

    match loopback {
        IpAddr::V4(addr) => println!("   Loopback: IPv4 address {}", addr),
        IpAddr::V6(addr) => println!("   Loopback: IPv6 address {}", addr),
    }

    println!("\n   Enum variants become constructor functions\n");
}

// 3. Different data types in variants
#[allow(dead_code)]
fn demonstrate_different_data_types() {
    println!("3. Different Data Types in Variants");
    println!("   Each variant can have different types and amounts of data\n");

    // IP address with different data types
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("   enum IpAddr {{");
    println!("       V4(u8, u8, u8, u8),  // Four numbers");
    println!("       V6(String),          // One string");
    println!("   }}");

    match home {
        IpAddr::V4(a, b, c, d) => println!("\n   Home: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("\n   Home: {}", addr),
    }

    match loopback {
        IpAddr::V4(a, b, c, d) => println!("   Loopback: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("   Loopback: {}", addr),
    }

    // Message enum with various types
    println!("\n   Message enum with different variant types:");
    #[allow(dead_code)]
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },    // Named fields (like struct)
        Write(String),              // Single String
        ChangeColor(i32, i32, i32), // Three i32s
    }

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    fn process_message(msg: Message) {
        match msg {
            Message::Quit => println!("   Quit message"),
            Message::Move { x, y } => println!("   Move to ({}, {})", x, y),
            Message::Write(text) => println!("   Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("   Change color to RGB({}, {}, {})", r, g, b),
        }
    }

    for msg in messages {
        process_message(msg);
    }
    println!();
}

// 4. Methods on enums
fn demonstrate_enum_methods() {
    println!("4. Methods on Enums");
    println!("   Define methods using impl, just like structs\n");

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("   Calling Quit"),
                Message::Move { x, y } => println!("   Calling Move({}, {})", x, y),
                Message::Write(text) => println!("   Calling Write(\"{}\")", text),
                Message::ChangeColor(r, g, b) => {
                    println!("   Calling ChangeColor({}, {}, {})", r, g, b)
                }
            }
        }

        fn describe(&self) -> String {
            match self {
                Message::Quit => String::from("Quit message"),
                Message::Move { x, y } => format!("Move to ({}, {})", x, y),
                Message::Write(text) => format!("Write: {}", text),
                Message::ChangeColor(r, g, b) => format!("RGB({}, {}, {})", r, g, b),
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    println!("   Description: {}\n", m.describe());
}

// 5. The Option enum
fn demonstrate_option_enum() {
    println!("5. The Option Enum");
    println!("   Rust doesn't have null - it has Option<T>\n");

    println!("   enum Option<T> {{");
    println!("       Some(T),");
    println!("       None,");
    println!("   }}");

    // Creating Option values
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("\n   let some_number = Some(5);           // Option<i32>");
    println!("   let some_char = Some('e');           // Option<char>");
    println!("   let absent_number: Option<i32> = None;");

    println!("\n   some_number: {:?}", some_number);
    println!("   some_char: {:?}", some_char);
    println!("   absent_number: {:?}", absent_number);

    // Option prevents null pointer errors
    println!("\n   Why Option is better than null:");
    println!("   - Compiler forces you to handle the None case");
    println!("   - Can't use Option<T> as if it were T");
    println!("   - Must explicitly convert Option<T> to T\n");
}

// 6. Working with Option
fn demonstrate_working_with_option() {
    println!("6. Working with Option");
    println!("   Must handle both Some and None cases\n");

    // Example 1: Using match
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("   Using match:");
    println!("   plus_one(Some(5)) = {:?}", six);
    println!("   plus_one(None) = {:?}", none);

    // Example 2: Using if let
    println!("\n   Using if let:");
    if let Some(value) = six {
        println!("   Got value: {}", value);
    } else {
        println!("   Got None");
    }

    // Example 3: Using unwrap_or
    println!("\n   Using unwrap_or (provide default):");
    let some_value = Some(10);
    let no_value: Option<i32> = None;

    println!("   Some(10).unwrap_or(0) = {}", some_value.unwrap_or(0));
    println!("   None.unwrap_or(0) = {}", no_value.unwrap_or(0));

    // Example 4: Using map
    println!("\n   Using map (transform the value):");
    let some_string = Some("hello");
    let string_length = some_string.map(|s| s.len());
    println!("   Some(\"hello\").map(|s| s.len()) = {:?}", string_length);

    // Example 5: Real-world example
    println!("\n   Real-world example - finding an item:");
    fn find_item(items: &[&str], search: &str) -> Option<usize> {
        for (index, &item) in items.iter().enumerate() {
            if item == search {
                return Some(index);
            }
        }
        None
    }

    let items = ["apple", "banana", "cherry"];
    match find_item(&items, "banana") {
        Some(index) => println!("   Found 'banana' at index {}", index),
        None => println!("   'banana' not found"),
    }

    match find_item(&items, "orange") {
        Some(index) => println!("   Found 'orange' at index {}", index),
        None => println!("   'orange' not found"),
    }

    println!();
}