use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {

    println!("\n**** Its... THE NUMBER GUESSING GAME ****");
    play_game();

    loop {
        println!("Would you like to play again? (y/n)");

        let mut user_will_play = String::new();
        io::stdin().read_line(&mut user_will_play)
            .expect("Failed to read line");

        match user_will_play.trim().to_lowercase()  == "y" {
            true => play_game(),
            false => {
                println!("Goodbye!");
                break;
            }
        }


    }
}


fn play_game() {

    let target_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("\nEnter a guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("\nYou guessed {}", guess);
        println!("....");

        match guess.cmp(&target_number) {
            Ordering::Less => println!("Try higher!"),
            Ordering::Greater => println!("Try lower!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }   
        }
    }
}
