extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

const YAY: &'static str = "Correct!";

pub fn random_comparison_win(guess: u32, answer: u32) -> &'static str {
	
	match guess.cmp(&answer) {
	// returns an answer string based on comparison with guess
	Ordering::Less =>	{"Too small!"},
	Ordering::Equal =>	{YAY},
	Ordering::Greater =>	{"Too big!"},
	}
}

fn main() {
	println!("Test your multiplication skills! Type 'quit' to leave the game.");
	let mut n_guesses: u32 = 0;
	
	loop {
		let int_1: u32 = rand::thread_rng().gen_range(2, 13);
		let int_2: u32 = rand::thread_rng().gen_range(2, 13);
		println!("What is {} times {}?", int_1, int_2);
		println!("Please input your answer:");
		
		let mut guess = String::new();  //Make an empty string to hold guesses

		io::stdin().read_line(&mut guess)
			.expect("Oh no! Failed to read input!");  
		// populate it with whatever the user types

		if guess.trim() == String::from("quit") {
			println!("Goodbye!");
			break;
		};
	    
		let guess_int: u32 = match guess.trim().parse() {
			Ok(num) => {n_guesses += 1; num},
			Err(_) => {println!("Please type an integer number only!");
					continue;}
		}; 
		// coerce to unsigned 32 bit integer, if you can

		let result = random_comparison_win(guess_int, int_1*int_2);
		println!("{}", result);

		if result != YAY {
		println!("You lose! Your record is {} successes!", n_guesses);
		break;		
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_when_numbers_same_returns_yay() {
	let num: u32 = rand::thread_rng().gen_range(1, 101);
	let result = random_comparison_win(num, num);
    	assert_eq!(result, YAY);
    }
    #[test]
    fn test_when_numbers_differ_you_do_not_win() {
	let num: u32 = rand::thread_rng().gen_range(1, 101);
	let other_num = num + 1;
	let result = random_comparison_win(num, other_num);
	assert_ne!(result, YAY);
    }
}
