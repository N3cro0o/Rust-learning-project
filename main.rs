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

    calculator_stuff();
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
        let guess : u32 = match guess.trim().parse() { // ALWAYS declare var type
            Ok(num) => num,
            Err(_) => { // _ as argument is a catchtail - match all Err values 
                println!("Please input a valid number.");
                continue;
            }
        };
            // trim() - remove whitespace at the beginning and at the end of the string
            // parse() - convert string to another type, u32 in this case
            // expect(msg) - same as above
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

fn calculator_stuff(){
    const ERR_TEXT1 : &str = "Wrong number, please input 0-9.";
    const SYMB_TEXT : &str = "Which operation?\n0-add\n1-subtract\n2-multiply\n3-divide";
    let mut var_a = 0; // First variable
    let mut var_b = 0; // Second variable
    let mut var_symb = 0; // 0 - add, 1 - sub, 2 - mul, 3 - div
    let mut var_neg = false;
    let mut var_check = false; // Check which variable to modify

    'calc: loop {
        let var_name;
        let mut var_val : i32;
        // Setup
        if !var_check
        {
            var_name = "A";
            var_val = var_a;
        }
        else
        {
            var_name = "B";
            var_val = var_b;
        }
        // Negation
        if var_neg
        {
            var_val = -var_val;
        }
        println!("Variable {var_name} = {var_val}");
        var_val = var_val.abs();
        // Get value
        let mut input_digit = String::new();
        io::stdin()
            .read_line(&mut input_digit)
            .expect("Failed to read the line");
        // Check if end request
        if input_digit.trim().is_empty()
        {
            if !var_check
            {
                if var_neg {    // Negation val_a
                    var_a *= -1;
                }
                'symb: loop // Get operation
                {
                    println!("{SYMB_TEXT}");
                    let mut input_symb = String::new();
                    io::stdin()
                        .read_line(&mut input_symb)
                        .expect("Failed to read the line");
                    let mut symb:u8 = match input_symb.trim().parse(){
                        Ok(num) => {
                            num
                        },
                        Err(_) =>
                        {
                            continue;
                        }
                    };
                    symb = symb % 4;
                    println!("{symb}");
                    var_symb = symb;
                    break 'symb;
                }
                // Reset bool vars
                var_check = true;
                var_neg = false;
            }
            else
            {
                if var_neg { // Negation val_b
                    var_b *= -1;
                }
                break 'calc;
            }
        }
        // Check if inverse
        if input_digit.chars().nth(0).unwrap() == '-'
        {
            var_neg = !var_neg;
            continue;
        }
        // Get digit
        let digit : i32 = match input_digit.trim().parse() {
            Ok(num) => {
                if num >= 10 {
                    println!("{ERR_TEXT1}");
                    continue;
                }
                else{
                    num
                }
            },
            Err(_) => { // _ as argument is a catchtail - match all Err values 
                println!("{ERR_TEXT1}");
                continue;
            }
        };
        var_val = var_val * 10 + digit;
        if !var_check
        {
            var_a = var_val;
        }
        else
        {
            var_b = var_val;
        }
    }
    // Random bullshit go!
    // Still don't know how match works
    let var_answ;
    if var_symb == 0 {
        var_answ = var_a + var_b;
    }
    else if var_symb == 0 {
        var_answ = var_a - var_b;
    }
    else if var_symb == 0 {
        var_answ = var_a * var_b;
    }
    else {
        var_answ = var_a / var_b;
    }
    // Answer
    println!("Answer = {}", var_answ);
}