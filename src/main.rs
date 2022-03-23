use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101); // generating random number

    println!("Guess the number!");

    loop {
        println!("Secret number is {secret_number}");
        // getting input
        let mut guess = String::new();
        println!("Please input your number");
        io::stdin().read_line(&mut guess).expect("Cant read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
