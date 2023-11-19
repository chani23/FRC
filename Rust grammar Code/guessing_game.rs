use std::io;

fn main() {
    println!("Guess the number!")
    println!("Please input your guess.")

    let mut guess = String::new();
        expect("Failed to read line");
    println!("you guessed: {}", guess);
}