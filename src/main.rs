extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

const YAY: &'static str = "You WIN!!";

pub fn random_comparison_win(guess: u32, secret: u32) -> &'static str {
	//println!("You guessed: {}", guess);
	//println!("The secret number is {}", secret);
	
	match guess.cmp(&secret) {
	// returns an answer string based on comparison with guess
	Ordering::Less =>	{"Too small!"},
	Ordering::Equal =>	{YAY},
	Ordering::Greater =>	{"Too big!"},
	}
}

fn main() {
	println!("Guess the number between 1 and 100! Type 'quit' to leave the game.");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	let mut n_guesses: u32 = 0;
	
	loop {
		println!("Please input your guess:");
		
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

		let result = random_comparison_win(guess_int, secret_number);
		println!("{}", result);

		if result == YAY {
		println!("It took you {} guesses!", n_guesses);
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
