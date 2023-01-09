/* https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let mut count: u16 = 0;

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line.");

        count += 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed {}.", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("({}) {} is too low.", count, guess),
            Ordering::Greater => println!("({}) {} is too high.", count, guess),
            Ordering::Equal => {
                println!("({}) You win!  {} is correct.", count, guess);
                break;
            }
        }
    }
}
