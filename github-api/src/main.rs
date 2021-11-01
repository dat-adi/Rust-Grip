// Importing the standard library
// for env which works with the process
use exitfailure::ExitFailure;
use std::env;
use reqwest::Url;

struct User{
    username: String,
    repos: i64
}

impl User{
    async fn get(username: &String) -> Result<Self, ExitFailure>{
        let url = format!(
            "https://api.github.com/users/{}",
            username
        );

        let url = Url::parse(&url);
        let res = reqwest::get(url).await?.json::<User>().await?;

        Ok(res);
    }
}

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
