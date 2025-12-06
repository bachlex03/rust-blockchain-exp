use rand::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game!");

    let secret_number = random_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("You guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guess {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
