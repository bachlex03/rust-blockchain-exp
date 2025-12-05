use std::io;

fn main() {
    println!("Guessing game!");

    let mut guess = String::new();

    print!("You guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess {guess}")
}
