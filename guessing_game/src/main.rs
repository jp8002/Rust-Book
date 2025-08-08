use std::{ cmp::Ordering, io::stdin};
use rand::Rng;

fn main(){
    println!("Guess the number");

    let secret_number =  rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input you guess:");
    
        let mut guess = String::new();
    
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };
    
        println!("you guessed: {guess}");
    
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater  => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            },
        }
    }
}