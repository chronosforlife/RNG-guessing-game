use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Bad IO");

        println!("You guessed: {}", guess);
        println!("Guess the number!");
        let rng:u32 = rand::thread_rng().gen_range(1, 101);
        println!("The secret number was:  {}", rng);

}
