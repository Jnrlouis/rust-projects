use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number from 1-10");
    println!("You have only 3 chances");
    let secret_number = rand::thread_rng().gen_range(1..11);
    let chances = 3;
    let mut attempts = 0;

    loop {
        println!("Input your guess: ");
        let mut guess = String::new();
        attempts += 1;
        let remaining_attempts = chances - attempts;
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
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
        };

        match attempts.cmp(&chances) {
            Ordering::Less => println!("You have {} more attempts.", remaining_attempts),
            Ordering::Greater => {
                println!("Game Over! You lose.");
                break;
            },
            Ordering::Equal => {
                println!("Game Over! You lose.");
                break;
            }

        }
    }
}
