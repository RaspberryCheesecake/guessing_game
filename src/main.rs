extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn random_comparison_win(guess: u32, secret: u32) -> std::string::String {
	println!("You guessed: {}", guess);
	println!("The secret number is {}", secret);
	
	match guess.cmp(&secret) {
	// returns an answer string based on comparison with guess
	Ordering::Less =>	{String::from("Too small!")},
	Ordering::Equal =>	{String::from("You win!")},
	Ordering::Greater =>	{String::from("Too big!")},
	}
}

fn main() {
	println!("Guess the number between 1 and 100!");
	println!("Please input your guess:");
    
	let mut guess = String::new();  //Make an empty string to hold guesses
	io::stdin().read_line(&mut guess)
		.expect("Oh no! Failed to read input!");  
	// populate it with whatever the user types
    
	let guess_int: u32 = guess.trim().parse()
		.expect("Please type an integer number only!");
	// coerce to unsigned 32 bit integer, if you can

	let secret_number = rand::thread_rng().gen_range(1, 101);
	
	let result = random_comparison_win(guess_int, secret_number);
	println!("{}", result);
}
