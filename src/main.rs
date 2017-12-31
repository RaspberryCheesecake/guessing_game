extern crate rand;

use std::io;
use rand::Rng;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1, 101);
	println!("Guess the number between 1 and 100!");
	println!("Please input your guess:");
    
	let mut guess = String::new();  //Make an empty string to hold guesses
	io::stdin().read_line(&mut guess)
		.expect("Oh no! Failed to read input!");  
	// populate it with whatever the user types
    
	let guess_int: u32 = guess.trim().parse()
		.expect("Please type an integer number only!");
	// coerce to unsigned 32 bit integer, if you can
	
	println!("You guessed: {}", guess_int);
	println!("The secret number is {}", secret_number);
}
