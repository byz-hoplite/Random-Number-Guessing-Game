use std::io;
use std::cmp::Ordering; // enum of 2 values being compared
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess number: ");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret number is: {}", secret_number);

    loop {
        println!("Input number to guess: ");

        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        println!("Your guess: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Input is smaller than number".red()),
            Ordering::Greater => println!("{}","Input is greater than number".red()),
            Ordering::Equal => {
            println!("{}", "CORRECT NUMBER! GAME OVER..".green());
            break;    
            },
        }
    }

}
