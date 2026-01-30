const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    example_mutability();
    example_constants();
    example_shadowing();
}

fn example_mutability() {
    let mut x = 5;
    println!("mutability: {x}");
    x = 6;
    println!("mutability: {x}");
}

fn example_constants() {
    println!("constant: {THREE_HOURS_IN_SECONDS}");
    let seconds = 60 * 60 * 3;
    println!("constant (expr): {seconds}");
}

fn example_shadowing() {
    let x = 5;
    let x = x + 1;
    println!("shadowing: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("shadowing transform: {spaces}");
}
