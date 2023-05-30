use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Welcome to 'Guess the number' game!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut remaining_guesses = 3;

    while remaining_guesses > 0 {
        println!("You have {} guesses left, the number is between 1 and 10, good luck :)", remaining_guesses);
        println!("Please, input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break
            },
        }

        remaining_guesses -= 1
    }

    if remaining_guesses == 0 {
        println!("You didn't guess :(\nThe secret number was: {}", secret_number);
    }
}
