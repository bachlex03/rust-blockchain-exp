fn main() {
    println!("=== Structs in Rust ===\n");

    // 1. Defining and instantiating structs
    demonstrate_basic_structs();

    // 2. Mutable structs
    demonstrate_mutable_structs();

    // 3. Field init shorthand
    demonstrate_field_init_shorthand();

    // 4. Struct update syntax
    demonstrate_struct_update_syntax();

    // 5. Tuple structs
    demonstrate_tuple_structs();

    // 6. Unit-like structs
    demonstrate_unit_like_structs();

    // 7. Struct methods
    demonstrate_struct_methods();
}

// 1. Defining and instantiating structs
fn demonstrate_basic_structs() {
    println!("1. Defining and Instantiating Structs");
    println!("   Structs group related data with named fields\n");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Create an instance
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("   User created:");
    println!("   - Username: {}", user1.username);
    println!("   - Email: {}", user1.email);
    println!("   - Active: {}", user1.active);
    println!("   - Sign-in count: {}", user1.sign_in_count);
    println!();
}

// 2. Mutable structs
fn demonstrate_mutable_structs() {
    println!("2. Mutable Structs");
    println!("   Entire instance must be mutable to change fields\n");

    #[allow(dead_code)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("   Original email: {}", user1.email);

    // Change a field value
    user1.email = String::from("anotheremail@example.com");
    user1.sign_in_count += 1;

    println!("   Updated email: {}", user1.email);
    println!("   Updated sign-in count: {}", user1.sign_in_count);
    println!("\n   Note: Rust doesn't allow marking only certain fields as mutable\n");
}

// 3. Field init shorthand
fn demonstrate_field_init_shorthand() {
    println!("3. Field Init Shorthand");
    println!("   Avoid repetition when parameter names match field names\n");

    #[allow(dead_code)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Without shorthand (verbose)
    fn build_user_verbose(email: String, username: String) -> User {
        User {
            active: true,
            username: username, // Repetitive
            email: email,       // Repetitive
            sign_in_count: 1,
        }
    }

    // With shorthand (concise)
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // Shorthand!
            email,    // Shorthand!
            sign_in_count: 1,
        }
    }

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user2 = build_user_verbose(
        String::from("another@example.com"),
        String::from("anotherusername"),
    );

    println!("   User 1 (with shorthand): {}", user1.email);
    println!("   User 2 (without shorthand): {}", user2.email);
    println!();
}

// 4. Struct update syntax
fn demonstrate_struct_update_syntax() {
    println!("4. Struct Update Syntax");
    println!("   Create instances from other instances with ..\n");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Without update syntax (verbose)
    let user2_verbose = User {
        active: user1.active,
        username: String::from("user2name"), // Different
        email: String::from("another@example.com"), // Different
        sign_in_count: user1.sign_in_count,
    };

    // With update syntax (concise)
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3name"),
        ..user2_verbose // Use remaining fields from user2_verbose
    };

    println!("   User 3 email: {}", user3.email);
    println!("   User 3 username: {}", user3.username);
    println!("   User 3 active: {}", user3.active);
    println!("   User 3 sign-in count: {}", user3.sign_in_count);

    println!("\n   Note: .. moves data, so user2_verbose.username is no longer usable");
    println!("   But user2_verbose.active and sign_in_count are still usable (Copy trait)\n");
}

// 5. Tuple structs
fn demonstrate_tuple_structs() {
    println!("5. Tuple Structs");
    println!("   Structs without named fields, just types\n");

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("   Color and Point are different types!");
    println!("   Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("   Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Destructuring tuple structs
    let Color(r, g, b) = black;
    println!("\n   Destructured black: r={}, g={}, b={}", r, g, b);

    // Function that only accepts Color, not Point
    fn print_color(color: Color) {
        println!("   Color RGB: ({}, {}, {})", color.0, color.1, color.2);
    }

    let red = Color(255, 0, 0);
    print_color(red);
    // print_color(origin); // Error! Point is not Color

    println!();
}

// 6. Unit-like structs
fn demonstrate_unit_like_structs() {
    println!("6. Unit-Like Structs");
    println!("   Structs with no fields\n");

    struct AlwaysEqual;

    let _subject = AlwaysEqual;

    println!("   AlwaysEqual is a unit-like struct");
    println!("   Useful for implementing traits without data");
    println!("   Example: marker types, zero-sized types\n");

    // Another example
    struct Marker;

    let _marker1 = Marker;
    let _marker2 = Marker;

    println!("   Created two Marker instances");
    println!("   They take up zero bytes in memory!\n");
}

// 7. Struct methods
fn demonstrate_struct_methods() {
    println!("7. Struct Methods");
    println!("   Define methods using impl blocks\n");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Method (takes &self)
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Method that takes mutable reference
        fn double(&mut self) {
            self.width *= 2;
            self.height *= 2;
        }

        // Method that checks a condition
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // Associated function (no self) - like a constructor
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle::square(25);

    println!("   rect1: {:?}", rect1);
    println!("   rect1 area: {}", rect1.area());
    println!("   Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("   Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    rect1.double();
    println!("\n   After doubling rect1: {:?}", rect1);
    println!("   rect1 area: {}", rect1.area());

    println!("\n   Square (associated function): {:?}", rect3);
    println!();
}