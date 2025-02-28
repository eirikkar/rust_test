use rand::Rng;
use std::{cmp::Ordering, io};
//Main function
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop {
        let mut guess = String::new();
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
