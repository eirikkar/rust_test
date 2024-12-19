use rand::Rng;
use std::{cmp::Ordering, io};
//Main function
fn main() {
    println!("Guess the number");

    println!("Please input your guess: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    let number: i32 = guess.trim().parse().expect("Input is not a integer!");

    match number.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    let x = 5;
    println!("The value of x is: {x}");
    another_function(5);
    check_value(2);
}
//Another function
fn another_function(x: i32) {
    println!("The value in the function is {x}");
}
fn check_value(x: i32) {
    if x < 5 {
        println!("Value is less than 5")
    } else {
        println!("Value is more than 5")
    }
}
