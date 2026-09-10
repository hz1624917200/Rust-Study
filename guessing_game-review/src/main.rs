use std::{cmp::Ordering, io};

fn main() {
    println!("Hello world!");

    let secret_rand = rand::random_range(1..=100);
    println!("rand: {}", secret_rand);

    println!("input your guess: ");

    loop {
        let mut guess = String::new();

        // let mut guess_2 = guess;     // move test
        // let guess_ref = &mut guess;     // reference test
        // guess_ref.split_off(3);

        io::stdin().read_line(&mut guess)
            .expect("read line error.");
        let guess = match guess.trim().parse::<u32>() {
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
            // _ => println!("Too big!")
        }
    }

    // println!("Hello, welcome to the guessing game!");

    // let secret_number = rand::random_range(1..=100);

    // loop {
    //     println!("Please input your guess:");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please input a number!");
    //             continue;
    //         }
    //     };

    //     println!("You guessed: {guess}");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }
}
