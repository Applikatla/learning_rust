use std::io;

fn main() {
    println!("Hello, world!");
    
    let target: i32 = 10; // The correct number to guess
    let mut count: i32 = 0;

    while count < 10 {  // Loop until 10 attempts
        println!("Enter your guess (integer only): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Convert input string to an integer
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter an integer.");
                continue; // Skip to the next iteration
            }
        };

        // // alter for match 

        // let guess: i32 = input.trim().parse().expect("Please enter a valid number!");


        println!("Your guess: {}", guess);

        count += 1; // Increment attempt count

        if guess == target {
            println!("You won!!! 🎉");
            return; // Exit the program
        } 
    }

    println!("You lost, sorry! Try again!!!");
}
