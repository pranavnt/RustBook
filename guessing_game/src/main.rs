// io is part of rust std lib
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // this prints a string to teh screen
    println!("Guess the number!");
    println!("Enter your guess!");

    // creating immutable variable
    //
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // initializes mutable String for teh guess
    // immutable variables are the default in rust
    // ::new() - new is associated function of the String type. Similar to static methods.
    // implemented on the type, not instance of type

    // loop keyword creates infinite loop
    loop {
        println!("Please enter your guess.");

        let mut guess = String::new();
        // calls stdin function of io library
        // calls readline method and passing `&mut guess` as parameter
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //  {} is a placeholder
        println!("You guessed: {}", guess);

        // we are shadowing the previous value of guess
        // guess refers to the original variable
        // trim eliminates whitespace at the start and end
        // the parse will parse teh string into a number
        //  parse can easily cause an error -- if it isnt a number
        // if an error is thrown, expect will be called
        // u32 is the type
        //let guess: u32 = guess.trim().parse().expect("Please type a number");

        // if result is Ok, return teh number
        // if not, continue will skip the current iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match expression consists of arms
        // an arm is a pattern
        // cmp method compares two values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                // breaking out of the infinite loop
                println!("You win!");
                break;
            }
        }
    }
}
