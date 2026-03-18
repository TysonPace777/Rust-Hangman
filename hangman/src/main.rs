// This can be run with `cargo run` in the terminal
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    //variables
    let is_running: bool = true;
    let items = vec!["apple", "strawberry", "camel", "rust", "cowboy", "dinosaur"];

    let mut rng = thread_rng();
    let random_item = *items.choose(&mut rng).unwrap();

    println!(" (debug) word: {}", random_item);

    while is_running {
        println!("Hangman game");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess = guess.trim();

        if guess == random_item {
            println!("You won!");
            break;
        }
    }
}
