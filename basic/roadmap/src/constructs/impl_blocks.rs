fn main() {
    println!("=== impl Keyword in Rust ===\n");

    // 1. Basic methods with &self
    demonstrate_basic_methods();

    // 2. Mutable methods with &mut self
    demonstrate_mutable_methods();

    // 3. Methods with multiple parameters
    demonstrate_methods_with_parameters();

    // 4. Associated functions (no self)
    demonstrate_associated_functions();

    // 5. Method calls as syntactic sugar
    demonstrate_method_syntax_sugar();

    // 6. Multiple impl blocks
    demonstrate_multiple_impl_blocks();

    // 7. Getters and method naming
    demonstrate_getters();

    // 8. Methods that consume self
    demonstrate_consuming_methods();
}

// 1. Basic methods with &self
fn demonstrate_basic_methods() {
    println!("1. Basic Methods with &self");
    println!("   Methods that borrow the instance immutably\n");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Method that borrows self immutably
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }

        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("   Rectangle: {:?}", rect);
    println!("   Area: {}", rect.area());
    println!("   Perimeter: {}", rect.perimeter());
    println!("   Is square? {}", rect.is_square());
    println!();
}

// 2. Mutable methods with &mut self
fn demonstrate_mutable_methods() {
    println!("2. Mutable Methods with &mut self");
    println!("   Methods that borrow the instance mutably\n");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Method that modifies self
        fn set_width(&mut self, width: u32) {
            self.width = width;
        }

        #[allow(dead_code)]
        fn set_height(&mut self, height: u32) {
            self.height = height;
        }

        fn scale(&mut self, factor: u32) {
            self.width *= factor;
            self.height *= factor;
        }
    }

    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("   Original: {:?}, area: {}", rect, rect.area());

    rect.set_width(15);
    println!("   After set_width(15): {:?}, area: {}", rect, rect.area());

    rect.scale(2);
    println!("   After scale(2): {:?}, area: {}", rect, rect.area());
    println!();
}

// 3. Methods with multiple parameters
fn demonstrate_methods_with_parameters() {
    println!("3. Methods with Multiple Parameters");
    println!("   Methods can take additional parameters after self\n");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        #[allow(dead_code)]
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Method with another parameter
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // Method with multiple parameters
        #[allow(dead_code)]
        fn resize(&mut self, new_width: u32, new_height: u32) {
            self.width = new_width;
            self.height = new_height;
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("   rect1: {:?}", rect1);
    println!("   rect2: {:?}", rect2);
    println!("   rect3: {:?}", rect3);
    println!();
    println!("   Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("   Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!();
}

// 4. Associated functions (no self)
fn demonstrate_associated_functions() {
    println!("4. Associated Functions");
    println!("   Functions without self, often used as constructors\n");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Associated function - constructor
        fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }

        // Associated function - specialized constructor
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }

        // Associated function - factory method
        fn default() -> Self {
            Self {
                width: 1,
                height: 1,
            }
        }
    }

    // Call with :: syntax
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::square(25);
    let rect3 = Rectangle::default();

    println!("   Rectangle::new(30, 50): {:?}", rect1);
    println!("   Rectangle::square(25): {:?}", rect2);
    println!("   Rectangle::default(): {:?}", rect3);
    println!();
}

// 5. Method calls as syntactic sugar
fn demonstrate_method_syntax_sugar() {
    println!("5. Method Calls as Syntactic Sugar");
    println!("   Method calls are sugar for function calls\n");

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn set_width(&mut self, width: u32) {
            self.width = width;
        }
    }

    let mut r = Rectangle {
        width: 10,
        height: 20,
    };

    // Method syntax
    let area1 = r.area();

    // Equivalent function syntax
    let area2 = Rectangle::area(&r);

    println!("   Method syntax: r.area() = {}", area1);
    println!("   Function syntax: Rectangle::area(&r) = {}", area2);
    println!("   Both are equivalent!");

    println!("\n   Method syntax: r.set_width(15)");
    r.set_width(15);

    println!("   Function syntax: Rectangle::set_width(&mut r, 20)");
    Rectangle::set_width(&mut r, 20);

    println!("   Final width: {}", r.width);
    println!();
}

// 6. Multiple impl blocks
fn demonstrate_multiple_impl_blocks() {
    println!("6. Multiple impl Blocks");
    println!("   A type can have multiple impl blocks\n");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // First impl block - constructors
    impl Rectangle {
        fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }

        #[allow(dead_code)]
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    // Second impl block - measurements
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }
    }

    // Third impl block - comparisons
    impl Rectangle {
        #[allow(dead_code)]
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect = Rectangle::new(30, 50);
    println!("   Rectangle: {:?}", rect);
    println!("   Area: {}", rect.area());
    println!("   Perimeter: {}", rect.perimeter());
    println!("\n   Multiple impl blocks are valid but usually not necessary\n");
}

// 7. Getters and method naming
fn demonstrate_getters() {
    println!("7. Getters and Method Naming");
    println!("   Methods can have the same name as fields\n");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Getter method with same name as field
        fn width(&self) -> u32 {
            self.width
        }

        // Method that checks a condition
        fn has_width(&self) -> bool {
            self.width > 0
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Field access (no parentheses)
    println!("   Field access: rect.width = {}", rect.width);

    // Method call (with parentheses)
    println!("   Method call: rect.width() = {}", rect.width());

    // Condition check
    if rect.has_width() {
        println!("   Rectangle has a non-zero width");
    }

    println!("\n   Rust distinguishes fields from methods by parentheses\n");
}

// 8. Methods that consume self
fn demonstrate_consuming_methods() {
    println!("8. Methods that Consume self");
    println!("   Methods can take ownership of self\n");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        #[allow(dead_code)]
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Method that consumes self and returns a new value
        fn max(self, other: Rectangle) -> Rectangle {
            Rectangle {
                width: self.width.max(other.width),
                height: self.height.max(other.height),
            }
        }

        // Method that transforms self into something else
        fn into_square(self) -> Rectangle {
            let size = self.width.max(self.height);
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };

    println!("   rect1: {:?}", rect1);
    println!("   rect2: {:?}", rect2);

    // This consumes both rectangles
    let max_rect = rect1.max(rect2);
    println!("   max_rect: {:?}", max_rect);

    // rect1 and rect2 are no longer usable here
    // println!("{:?}", rect1); // Error: value borrowed after move

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("\n   rect3: {:?}", rect3);

    let square = rect3.into_square();
    println!("   Transformed into square: {:?}", square);

    // rect3 is no longer usable
    // println!("{:?}", rect3); // Error: value borrowed after move

    println!("\n   Consuming methods are useful for transformations\n");
}