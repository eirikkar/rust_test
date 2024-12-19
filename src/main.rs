use std::io;
//Main function
fn main() {
    println!("Guess the number");

    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {}", guess);
    let x = 5;
    println!("The value of x is: {x}");
    another_function(5);
}
//Another function
fn another_function(x: i32) {
    println!("The value in the function is {x}");
}
