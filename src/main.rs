use random::Source;
use std::io::{stdin, stdout, ErrorKind, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_ATTEMPTS: usize = 3;
const LOW: usize = 1;
const HIGH: usize = 100;

fn main() {
    loop {
        println!("I generated a Random Number from {LOW} to {HIGH}.");
        println!("You have {MAX_ATTEMPTS} attempts to guess it.");
        let mut source = random::default(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );

        let num = source.read::<usize>();
        let answer = num % (HIGH - LOW + 1) + LOW;
        let mut won = false;

        println!("Hint: The time right now is {num}");

        for attempt in 1..=MAX_ATTEMPTS {
            print!("[{attempt}/{MAX_ATTEMPTS}] ");
            stdout().flush().unwrap();
            let mut guess = String::new();
            match stdin().read_line(&mut guess) {
                Ok(_) => {}
                Err(err) => match err.kind() {
                    ErrorKind::InvalidData => {
                        println!("Stop typing gibberish and play the game already ._.");
                    }
                    _ => panic!("Something went wrong: {err}"),
                },
            }
            match guess.trim().parse::<usize>() {
                Ok(guess) => {
                    if guess == answer {
                        won = true;
                        break;
                    } else if guess < answer {
                        println!("No! The answer is ᵇᶦᵍᵍᵉʳ!");
                    } else if guess > answer {
                        println!("No! The answer is ₛₘₐₗₗₑᵣ!");
                    }
                }
                Err(_) => {
                    println!("This is not even a number, come on ._.");
                }
            }
        }

        if won {
            println!("Yes! It was {answer}! You won!");
            print!("Play again? [Y/n] ");
        } else {
            println!("Sorry you ran out of attempts! The answer was {answer}.");
            print!("Try again? [Y/n] ");
        }

        stdout().flush().unwrap();
        let mut yorn = String::new();
        match stdin().read_line(&mut yorn) {
            Ok(_) => match yorn.trim().to_uppercase().as_str() {
                "N" | "NO" => break,
                _ => {}
            },
            Err(err) => match err.kind() {
                ErrorKind::InvalidData => {}
                _ => panic!("Something went wrong: {err}"),
            },
        }
    }
}
