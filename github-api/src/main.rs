// Importing the standard library
// for env which works with the process
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::header::USER_AGENT;

// Defining a structure with JSON compatibility
#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub name: String,
    pub public_repos: i64
}

// Defining a endpoint location as a constant
const GITHUB_ENDPOINT: &str = "https://api.github.com/users/";

// Addition of the get function 
// as part of the User Implementation
impl User{
    /*
     *
     * Arguments is the username in the form of a string
     * Returns the User object and a standard error?
     * Unsure why Box and dyn is used.
     *
     */
    async fn get(username: &String) -> Result<User, Box<dyn std::error::Error>>{
        // Formatting the string to create a path
        let url = format!(
            "{}{}", GITHUB_ENDPOINT, username
        );


        // Parses the url and provides it's value to the variable
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header(USER_AGENT, "gh simple cli")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // TODO there's a fix to be made in here.
        // Error("expected value")
        let user: User = serde_json::from_str(&res).unwrap();

        Ok(user)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Calling the async function
    let user = User::get(&username).await?;

    // Printing the arguments
    println!("User : {}\tRepositories : {}", user.name, user.public_repos);

    Ok(())
}
