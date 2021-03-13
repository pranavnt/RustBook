// io is part of rust std lib
use std::io;

fn main() {
    // this prints a string to teh screen
    println!("Guess the number!");
    println!("Enter your guess!");

    // initializes mutable String for teh guess
    // immutable variables are the default in rust
    // ::new() - new is associated function of the String type. Similar to static methods.
    // implemented on the type, not instance of type
    let mut guess = String::new();

    // calls stdin function of io library
    // calls readline method and passing `&mut guess` as parameter
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
