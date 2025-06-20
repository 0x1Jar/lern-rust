use::std::io; //input/output libary

pub fn run_progame () {
println!("Guest The Number");

println!("Please input your guess number: ");
let mut gues = String::new();

io::stdin()
.read_line(&mut gues) //read imput
.expect("Failed to read line"); //handling possible failure msg

println!("You Guessed: {}", gues); //result var gues, {} -> values
 
}