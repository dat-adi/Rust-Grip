use std::io;
use std::cmp::Ordering;
use rand::Rng;
// In order to utilize input/output, we bring the io library into scope

fn main() {
    // Few statements to let the user know what's up.
    println!("Guess the number!");

    // Here, we are utilizing the rand module's function
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // Taking an input
        println!("Please input your guess.");

        // Creating a mutable variable of the type String
        let mut guess = String::new();
        // Utilizing the standard io library to take in the input
        io::stdin()
            // Reading a line using the read_line function
            .read_line(&mut guess)
            // Exception Handling
            .expect("Failed to read line");

        // We shadow the guess variable
        // and proceed to trim, parse and convert it
        // into a unsigned 32 bit integer
        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Printing the guessed number using a format in the print statement
        println!("You guessed : {}", guess);

        // Here, we use the comparison function in order
        // to check if the user got the number correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
