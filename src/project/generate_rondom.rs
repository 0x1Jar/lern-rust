use rand::Rng; //random libary
use std::io; //input/output libary

pub fn run_rondom () {
println!("Guest The Number");

let secret_number = rand::rng().random_range(1..1000); //generate random number from 1 to 100

println!("The secret number is: {secret_number}");
println!("Please input your guess number: ");
let mut gues = String::new();

io::stdin()
.read_line(&mut gues) //read input
.expect("Failed to read line"); //handling possible failure msg

println!("You Guessed: {}", gues); //result var gues, {} -> values
 
}