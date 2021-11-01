// Importing the standard library
// for env which works with the process
use std::env;

fn main(){
    // Defining a vector that can take arguments
    // and a collect that converts the iterable into the vector
    let args:Vec<String> = env::args().collect();
    
    // Defines a mutable default variable
    // for the username
    let mut username: String = "dat-adi".to_string();

    // Checks to see if there are any arguments passed
    if args.len() < 2 {
        println!("As there was no specification for a username, defaults to dat-adi");
    } else {
        // Takes the argument provided
        username = args[1].clone();
    }

    // Printing the arguments
    println!("{:?}", username);
}
