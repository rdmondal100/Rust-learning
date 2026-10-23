use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! ");
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
        println!("Please input your gusses: ");

        let mut guss: String = String::new();
        io::stdin()
            .read_line(&mut guss)
            .expect("Failed to read the line");

        println!("You guessed {}", guss.trim());

        let gussed_number: u32 = match guss.trim().parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("Please input a number!");
                continue;
            }
        };

        match gussed_number.cmp(&secret_number) {
            Ordering::Equal => {    
                println!("You win!");
                break;
            },
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}
