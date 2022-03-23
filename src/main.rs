use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
        // println!("Secret number is {secret_number}");
        // getting input
        let mut guess = String::new();
        println!("Please input your number");
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("{}", "You entered some gibberish 👾".red());
                println!("{}", "Hint! Enter positive integer number 😉".yellow());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}", "You win 🙌 🏆".green());
                break;
            }
        }
    }
}
