use std::io;
use rand::Rng; // external library

fn main() {
    // stdout 
    println!("Guess the number!");
    
    // generate a number between 1 and 100 using an external
    // library rand
    let number = rand::thread_rng().gen_range(1..=100);
    
    // some tests to understand how &mut works 

    // let mut guess2 = String::new();
    //let mut x = &mut guess;
    //x = &mut guess2;
    
    // initialize a new string
    // which is the input from stdin
    let mut guess = String::new();   
    
    // get stdin and update guess with the current
    // value, fail with expect if read_line raise an error
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed");
    
    // cast guess with the type of number 
    // with pars, clear the input with trim
    // raise an error if parse fails
    let guess: i32 = guess
        .trim()
        .parse()
        .expect("Not a number");

    // if to compare input (guess) and number
    if guess == number {
        println!("Good");
    }
    else {
        println!("Bad");
    }
}

