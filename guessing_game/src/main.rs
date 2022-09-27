use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Hello, welcome to guessing game!");

    let secret_rand = rand::thread_rng().gen_range(1..=100);  // new api in version(0.8.5)
    println!("rand: {}", secret_rand);
    
    
    println!("Please input your guess: ");

    loop {
		let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        	.expect("read line error.");
		// let guess: u32 = guess.trim().parse()
		// 	.expect("Parse error");
		// let guess = guess.trim().parse::<u32>()
		//	.expect("parse error");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please input number!");
				continue;
			}
		};


		println!("You guessed {}", guess);

		match guess.cmp(&secret_rand) {
			Ordering::Less => println!("Too small!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			},
			Ordering::Greater => println!("Too big!")
		}
    }
    
}
