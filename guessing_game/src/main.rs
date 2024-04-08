use std::io;

fn main() {
    println!("Guess the number!");

    // create a new string type variable
    let mut guess = String::new();

    io::stdin()
        // read from stdin, assign to the mutable reference of the guess variable
        // the argument (guess) passed to read_line function needs to be mutable so the method can change the strings contents
        // the reference allows multiple parts of code access the one piece of data without needing to copying it into memory multiple times
        // references are imutable by default hence the &mut guess NOT &guess to make it mutable
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
