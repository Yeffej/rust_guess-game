use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("There is a random number between 0 and 100");
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(0..101);
    // println!("Secret number: {}", secret_number);
    
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new(); 
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Type a number");
                continue;
            }
        };

        println!("you guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            } 
        }

    }

}
