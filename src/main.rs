use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!!!");

    loop {
        println!("Input an number");
        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1, 101);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The Secret Number is {}", secret_number);
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Less!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
        }
    }
}
