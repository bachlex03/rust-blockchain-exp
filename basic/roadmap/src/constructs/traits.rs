use std::fmt::{Debug, Display};

fn main() {
    println!("=== Traits in Rust ===\n");

    // 1. Defining and implementing traits
    demonstrate_basic_traits();

    // 2. Default implementations
    demonstrate_default_implementations();

    // 3. Traits as parameters
    demonstrate_traits_as_parameters();

    // 4. Trait bounds
    demonstrate_trait_bounds();

    // 5. Multiple trait bounds
    demonstrate_multiple_trait_bounds();

    // 6. Where clauses
    demonstrate_where_clauses();

    // 7. Returning traits
    demonstrate_returning_traits();

    // 8. Conditional implementations
    demonstrate_conditional_implementations();
}

// 1. Defining and implementing traits
fn demonstrate_basic_traits() {
    println!("1. Defining and Implementing Traits");
    println!("   Traits define shared behavior\n");

    // Define a trait
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[allow(dead_code)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    #[allow(dead_code)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    // Implement the trait for NewsArticle
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    // Implement the trait for Tweet
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins are the best hockey team."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("   Article: {}", article.summarize());
    println!("   Tweet: {}", tweet.summarize());
    println!();
}

// 2. Default implementations
fn demonstrate_default_implementations() {
    println!("2. Default Implementations");
    println!("   Traits can provide default method implementations\n");

    pub trait Summary {
        fn summarize_author(&self) -> String;

        // Default implementation that calls another method
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    #[allow(dead_code)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
    }

    // Only implement the required method
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    println!("   Using default implementation:");
    println!("   {}", tweet.summarize());
    println!();
}

// 3. Traits as parameters
fn demonstrate_traits_as_parameters() {
    println!("3. Traits as Parameters");
    println!("   Use impl Trait syntax for function parameters\n");

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub author: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    // Function that accepts any type implementing Summary
    pub fn notify(item: &impl Summary) {
        println!("   Breaking news! {}", item.summarize());
    }

    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released!"),
        author: String::from("Rust Team"),
    };

    notify(&article);
    println!();
}

// 4. Trait bounds
fn demonstrate_trait_bounds() {
    println!("4. Trait Bounds");
    println!("   Full syntax for constraining generic types\n");

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct Article {
        pub title: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            self.title.clone()
        }
    }

    // impl Trait syntax (sugar)
    fn notify_short(item: &impl Summary) {
        println!("   Short syntax: {}", item.summarize());
    }

    // Trait bound syntax (full form)
    fn notify_long<T: Summary>(item: &T) {
        println!("   Long syntax: {}", item.summarize());
    }

    // When you need same type for multiple parameters
    fn notify_same<T: Summary>(item1: &T, item2: &T) {
        println!("   Same type: {} and {}", item1.summarize(), item2.summarize());
    }

    let article1 = Article {
        title: String::from("First Article"),
    };
    let article2 = Article {
        title: String::from("Second Article"),
    };

    notify_short(&article1);
    notify_long(&article1);
    notify_same(&article1, &article2);
    println!();
}

// 5. Multiple trait bounds
fn demonstrate_multiple_trait_bounds() {
    println!("5. Multiple Trait Bounds");
    println!("   Require multiple traits with + syntax\n");

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    pub struct Article {
        pub title: String,
        pub content: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{}: {}", self.title, self.content)
        }
    }

    impl Display for Article {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.title)
        }
    }

    // Multiple trait bounds with impl Trait
    fn notify_display(item: &(impl Summary + Display)) {
        println!("   Display: {}", item);
        println!("   Summary: {}", item.summarize());
    }

    // Multiple trait bounds with generic syntax
    fn notify_generic<T: Summary + Display>(item: &T) {
        println!("   Generic - Display: {}", item);
        println!("   Generic - Summary: {}", item.summarize());
    }

    let article = Article {
        title: String::from("Rust Traits"),
        content: String::from("Traits are awesome!"),
    };

    notify_display(&article);
    println!();
    notify_generic(&article);
    println!();
}

// 6. Where clauses
fn demonstrate_where_clauses() {
    println!("6. Where Clauses");
    println!("   Cleaner syntax for complex trait bounds\n");

    #[allow(dead_code)]
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug, Clone)]
    pub struct Article {
        pub title: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            self.title.clone()
        }
    }

    impl Display for Article {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.title)
        }
    }

    // Without where clause (hard to read)
    fn some_function_ugly<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> String {
        format!("{} and {:?}", t, u)
    }

    // With where clause (cleaner)
    fn some_function_clean<T, U>(t: &T, u: &U) -> String
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        format!("{} and {:?}", t, u)
    }

    let article = Article {
        title: String::from("Where Clauses"),
    };
    let number = 42;

    println!("   Without where: {}", some_function_ugly(&article, &number));
    println!("   With where: {}", some_function_clean(&article, &number));
    println!();
}

// 7. Returning traits
fn demonstrate_returning_traits() {
    println!("7. Returning Traits");
    println!("   Return types that implement traits\n");

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[allow(dead_code)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // Return impl Trait (must be single concrete type)
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        }
    }

    let tweet = returns_summarizable();
    println!("   Returned: {}", tweet.summarize());

    println!("\n   Note: impl Trait can only return a single concrete type");
    println!("   For multiple types, use trait objects (Box<dyn Trait>)\n");
}

// 8. Conditional implementations
fn demonstrate_conditional_implementations() {
    println!("8. Conditional Implementations");
    println!("   Implement methods only for types with certain traits\n");

    struct Pair<T> {
        x: T,
        y: T,
    }

    // Always available for any T
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Only available when T implements Display + PartialOrd
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("   The largest member is x = {}", self.x);
            } else {
                println!("   The largest member is y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(10, 20);
    pair.cmp_display();

    // Blanket implementation example
    println!("\n   Blanket implementations:");
    println!("   Any type implementing Display also gets ToString");
    let num = 42;
    let s = num.to_string(); // Works because i32 implements Display
    println!("   42.to_string() = \"{}\"", s);
    println!();
}