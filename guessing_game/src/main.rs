use std::io;
use std::cmp::Ordering;// it helps in comparison
use rand::Rng;
//  std::io library: here we use it  for getting user input
fn main()  { // this is just a convention
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);// gen_range takes a range 
    //as an argument and generates a number within the range, inclusive lower,
    //exclusive upper or one can write 1..=100

    //println!("The secret number is: {}",secret_number);

    loop {
       println!("Please input your guess!");//basic print macro, a function is 
       // without the !, while a macro is called with a !

        let mut guess = String::new();// mutable variable guess, 
        // unmutable let apple = 5; = binding sign, String is a data type, :: 
        // In full, the let mut guess = String::new(); line has created a mutable variable 
        //that is currently bound to a new, empty string variable.

        io::stdin()// user input
            .read_line(&mut guess)// where to store user input into the argument
            //The full job of read_line 
            //is to take whatever the user types into standard input and append 
            //that into a string (without overwriting its contents),
            // & guess would be mutable
            // &is a reference, allows the 
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
