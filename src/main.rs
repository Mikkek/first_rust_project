use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let num = rand::thread_rng()
        .gen_range(1, 101);

    let mut res = false;
    let mut guess_total = 0;

    println!("Guess the number!\nPlease input your guess");

    while !res {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("that is not a number!");
                continue;
            },
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("too low!"),
            Ordering::Equal => {
                println!("correct! You win!");
                res = true},
            Ordering::Greater => println!("too high!")
        }
        guess_total += 1;
    }
    println!("Total number of guesses: {}", guess_total);
}