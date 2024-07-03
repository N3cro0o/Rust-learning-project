// Libraries
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Basic stuff, 
    let mut a: i64 = 23; // let - varible declaration, I guess it's const without any modifications; mut - mutable
    let b: i64 = 123;
    a += b;
    println!("a = {a}"); // "{}" - to use variable as a string
    let b: i64 = 321; // shadowing
    a *= b;
    println!("New a = {a}");
    let text = "Some text";
    println!("\n{}", text);
    //text = text + ", and more"; - because text is a pointer, it's not possible to change value
    let mut text = String::new(); // Declare new String variable
    text.push_str("And now it works."); // And change its 'value'

    guessing_game();
}

fn guessing_game() {
    // Guessing game - rng
    let value = rand::thread_rng().gen_range(0..100); // Const var value. Thread engine is used to generate number between 0 and 99 ([0;99]). 0..=100 for [0;100]

    loop {
        // Guessing game - input
        println!("Pick a number between 0 - 99");
        print!("Input: ");
        let mut guess = String::new();
        io::stdin() // User input instance
            .read_line(&mut guess) // Call read_line method. Return value in guess reference
            .expect("Failed to read line"); // expect method is used to handle every state variant. It is overwritten if operation is successful
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { // _ as argument is a catchtail - match all Err values 
                println!("Please input a valid number.");
                continue;
            }
        };
            // trim() - remove whitespace at the beginning and at the end of the string
            // parse() - convert string to another type, u32 in this case
            // exoect(msg) - same as above
        println!("You guessed: {}", guess);

        // Guessing game - check
        match guess.cmp(&value){ // match one of the enum's variants
            Ordering::Equal => {println!("You got it!");
                break;},
            Ordering::Greater => println!("Answer too big"),
            Ordering::Less => println!("Answer too small")
        }
    }
}
