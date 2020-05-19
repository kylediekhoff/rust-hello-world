use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the NUMBER!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("TOO LOW"),
            Ordering::Equal => {
                println!("YOU WIN");
                break;
            },
            Ordering::Greater => println!("TOO HIGH"),
        }
    }
}
