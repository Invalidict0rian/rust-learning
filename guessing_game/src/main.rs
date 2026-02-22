use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let answer: u32 = rand::thread_rng().gen_range(1..=100);
    println!("answer is : {answer}");

    loop{
    println!("Guess a Number Any Number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Failed to Parse Guess");

    println!("you guessed {guess}");

    match guess.cmp(&answer){
        Ordering::Less => println!("guess too small"),
        Ordering::Equal => {
            println!("Hooray You Win!");
            break;
            },
        Ordering::Greater => println!("guess too big"),
    }
}
}
