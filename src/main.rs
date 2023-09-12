use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("the secret numer is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let guess: i32 = guess.trim().parse().expect("please enter a number!");

    if guess == secret_number {
        println!("You guessed right!")
    } else {
        println!("You guessed wrong!")
    }

}
