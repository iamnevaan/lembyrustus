use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let greet = "Hello, welcome to the guessing game.";
    let mut attempts = 10;
    println!("{}", greet);
    println!("Please guess a number from 1 - 100.");
    loop {
        if attempts < 1 {
            println!("Too many attempts. The secret number was: {}", secret_number);
            break;
        }
        let mut guess = String::new();
        

        io::stdin().read_line(&mut guess).expect("Failed to read line");
            
        let guess = guess.trim();

        if guess.eq_ignore_ascii_case("q") {
            println!("code exiting 0x0");
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number");
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win! Exiting...");
                break;
                }   
            }
        attempts -= 1;
        println!("Guess Again. Attempts left: {attempts} (q for exit)")
    }    

}
