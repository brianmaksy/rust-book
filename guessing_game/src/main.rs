use std::io; // imported io library, can call io directly
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); // thread_rng was enabled by rand::Rng imported. 
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); // mutable, empty string 

        io::stdin()
            .read_line(&mut guess) // calls the readline method. References (&) immutable by default. 
            .expect("Failed to read line"); // a method on io::Result, which is returned by the previous two 'lines'. Result types are enumerations. Has fixed set of possibilities known as variants. (used with match)
        
        // shadowing of variable. guess = guess."strtoi"
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Ok checks if parse is successful (giving a result of u32 indeed).
            Err(_) => {
                println!("Please enter a valid number.");
                continue; // parse, a result type, has these enum variants Ok and Err. 
            }
        };
        //[u seems to = unsigned]
        // parse = parse str to num
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
