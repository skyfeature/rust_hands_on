#![allow(dead_code, unused_variables, unused_assignments)]

use rand::Rng;
use std::cmp::Ordering;

pub fn entry_point() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

        println!("Existing guess: {guess}");

        // guess.push_str("Hello");
        // guess.push(',');
        // guess.push(' ');

        let input = std::io::stdin();
        let result = input.read_line(&mut guess);
        result.expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }


}
