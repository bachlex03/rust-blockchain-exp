use std::mem;

fn main() {
    println!("=== Box, Stack, and Heap ===\n");

    // 1. Stack allocation basics
    demonstrate_stack_allocation();

    // 2. Heap allocation with Box
    demonstrate_heap_allocation();

    // 3. Stack vs Heap memory size
    demonstrate_memory_sizes();

    // 4. Box dereferencing
    demonstrate_box_dereferencing();

    // 5. When to use Box
    demonstrate_when_to_use_box();

    // 6. Stack frames and function calls
    demonstrate_stack_frames();

    // 7. Box and ownership
    demonstrate_box_ownership();
}

// 1. Stack allocation basics
fn demonstrate_stack_allocation() {
    println!("1. Stack Allocation Basics");
    println!("   Values are stack allocated by default\n");

    let x = 5; // Allocated on stack
    let y = 10; // Allocated on stack
    let z = x + y; // Allocated on stack

    println!("   x = {} (on stack)", x);
    println!("   y = {} (on stack)", y);
    println!("   z = {} (on stack)", z);

    println!("\n   Stack allocation is:");
    println!("   - Fast (just move stack pointer)");
    println!("   - Automatic (compiler manages it)");
    println!("   - Limited in size");
    println!("   - LIFO (Last In, First Out)\n");
}

// 2. Heap allocation with Box
fn demonstrate_heap_allocation() {
    println!("2. Heap Allocation with Box");
    println!("   Box<T> allocates data on the heap\n");

    let x = 5; // Stack allocated
    let y = Box::new(5); // Heap allocated

    println!("   x = {} (stack)", x);
    println!("   y = {} (heap via Box)", y);

    // Box with larger data
    let _large_array = Box::new([0; 1000]);
    println!("   Large array allocated on heap");

    println!("\n   Heap allocation is:");
    println!("   - Slower than stack");
    println!("   - Manually requested (Box::new)");
    println!("   - Unlimited in size");
    println!("   - Can outlive function scope\n");
}

// 3. Stack vs Heap memory size
fn demonstrate_memory_sizes() {
    println!("3. Stack vs Heap Memory Size");
    println!("   Box stores pointer on stack, data on heap\n");

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    // Stack allocated
    let point: Point = Point { x: 0.0, y: 0.0 };
    let rectangle: Rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Heap allocated
    let boxed_point: Box<Point> = Box::new(Point { x: 0.0, y: 0.0 });
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    println!("   Point occupies {} bytes on stack", mem::size_of_val(&point));
    println!(
        "   Rectangle occupies {} bytes on stack",
        mem::size_of_val(&rectangle)
    );
    println!(
        "   Boxed point occupies {} bytes on stack (just pointer)",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "   Boxed rectangle occupies {} bytes on stack (just pointer)",
        mem::size_of_val(&boxed_rectangle)
    );

    println!("\n   Box size = pointer size (8 bytes on 64-bit systems)");
    println!("   Actual data lives on the heap\n");
}

// 4. Box dereferencing
fn demonstrate_box_dereferencing() {
    println!("4. Box Dereferencing");
    println!("   Use * to access the value inside a Box\n");

    let boxed_value = Box::new(42);
    println!("   boxed_value = {}", boxed_value);

    // Dereference to get the value
    let unboxed_value = *boxed_value;
    println!("   *boxed_value = {}", unboxed_value);

    // Box with struct
    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let boxed_point = Box::new(Point { x: 10, y: 20 });
    println!("\n   boxed_point = {:?}", boxed_point);

    let unboxed_point = *boxed_point;
    println!("   *boxed_point = {:?}", unboxed_point);
    println!();
}

// 5. When to use Box
fn demonstrate_when_to_use_box() {
    println!("5. When to Use Box");
    println!("   Common use cases for heap allocation\n");

    // Use case 1: Large data that would overflow the stack
    println!("   Use Case 1: Large data");
    let large_data = Box::new([0u8; 1_000_000]);
    println!("   Allocated 1MB array on heap");

    // Use case 2: Recursive types (like linked lists)
    println!("\n   Use Case 2: Recursive types");
    #[allow(dead_code)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("   Created recursive list: 1 -> 2 -> 3 -> Nil");

    // Use case 3: Trait objects (dynamic dispatch)
    println!("\n   Use Case 3: Trait objects");
    trait Animal {
        fn make_sound(&self) -> &str;
    }

    struct Dog;
    impl Animal for Dog {
        fn make_sound(&self) -> &str {
            "Woof!"
        }
    }

    let animal: Box<dyn Animal> = Box::new(Dog);
    println!("   Animal says: {}", animal.make_sound());

    drop(large_data); // Explicitly drop to avoid unused warning

    println!();
}

// 6. Stack frames and function calls
fn demonstrate_stack_frames() {
    println!("6. Stack Frames and Function Calls");
    println!("   Each function call creates a new stack frame\n");

    fn foo() {
        let y = 5;
        let z = 100;
        println!("   Inside foo: y = {}, z = {}", y, z);
    }

    let x = 42;
    println!("   In main: x = {}", x);

    foo();

    println!("   Back in main: x = {}", x);
    println!("\n   Stack frames are created and destroyed automatically");
    println!("   LIFO order: Last In, First Out\n");
}

// 7. Box and ownership
fn demonstrate_box_ownership() {
    println!("7. Box and Ownership");
    println!("   Box owns its data and cleans up when dropped\n");

    {
        let boxed = Box::new(String::from("Hello, heap!"));
        println!("   Created: {}", boxed);
    } // boxed goes out of scope, heap memory is freed

    println!("   Box dropped, heap memory freed automatically");

    // Moving a Box
    println!("\n   Moving a Box:");
    let box1 = Box::new(100);
    println!("   box1 = {}", box1);

    let box2 = box1; // Ownership moved
    println!("   box2 = {} (ownership moved from box1)", box2);
    // println!("   box1 = {}", box1); // Error! box1 no longer valid

    // Passing Box to function
    println!("\n   Passing Box to function:");
    fn take_ownership(b: Box<i32>) {
        println!("   Function received: {}", b);
    } // b is dropped here, heap memory freed

    let box3 = Box::new(200);
    take_ownership(box3);
    // println!("   box3 = {}", box3); // Error! box3 was moved

    println!("\n   Box follows ownership rules:");
    println!("   - One owner at a time");
    println!("   - Automatic cleanup when owner goes out of scope");
    println!("   - Prevents memory leaks and double-free errors\n");
}