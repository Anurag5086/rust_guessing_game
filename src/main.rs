use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guessgame() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The Random Number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("Yup! You found it!"),
            Ordering::Greater => println!("Your number is Greater."),
            Ordering::Less => println!("Your number is Smaller."),
        }
    }
}

fn main() {
    guessgame();
}
