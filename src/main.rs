use std::io;
use rand::Rng;
fn main() {
    // initialize options
    let options = ["rock","paper","scissor"];

    let mut input = String::new();

    println!("Enter rock, paper or scissor");

    // choose random option from array
    let cpu_option  = options[rand::thread_rng().gen_range(0..options.len())];

    // read user input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let user_option = input.trim().to_lowercase();

    println!("USER : {}, CPU : {}",user_option,cpu_option);

    // game logic here
    if (cpu_option == "rock" && user_option == "paper") ||
    (cpu_option == "scissor" && user_option == "rock") ||
    (cpu_option == "paper" && user_option == "scissor") {
        println!("YOU WON");
    } else if (cpu_option == "paper" && user_option == "rock") ||
            (cpu_option == "rock" && user_option == "scissor") ||
            (cpu_option == "scissor" && user_option == "paper") {
        println!("CPU WON");
    } else {
        println!("DRAW");
    }
}
