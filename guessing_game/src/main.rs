use std::io;

fn main() {
    println!("Guess a Number Any Number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed {guess}");
}
