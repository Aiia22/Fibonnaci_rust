use std::io; // Import the standard input/output library

fn main() {
    println!("Entrez un nombre pour calculer le Fibonacci :"); // Prompt the user for input

    let mut input = String::new(); // Create a mutable String to store the input
    io::stdin()
        .read_line(&mut input) // Read a line from standard input
        .expect("Échec de la lecture"); // Handle potential read errors

    // Parse the input string to an unsigned 32-bit integer (u32)
    let n: u32 = match input.trim().parse() {
        Ok(num) => num, // If parsing is successful, use the parsed number
        Err(_) => {
            println!("Ce n'est pas un nombre !"); // If parsing fails, print an error message
            return; // Exit the program
        }
    };

    // Calculate the Fibonacci number for the given input
    let result = fibonacci(n);
    println!("Fibonacci({}) = {}", n, result); // Print the result
}

// Function to calculate the nth Fibonacci number iteratively
fn fibonacci(n: u32) -> u64 {
    // Handle base cases
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    // Initialize variables for the iterative calculation
    let mut prev = 0;
    let mut current = 1;

    // Iterate from 2 to n, updating the Fibonacci sequence
    for _ in 2..=n {
        let new = prev + current; // Calculate the next Fibonacci number
        prev = current; // Update the previous number
        current = new; // Update the current number
    }
    current // Return the nth Fibonacci number
}
