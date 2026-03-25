// This can be run with `cargo run` in the terminal
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    //variables
    let is_running: bool = true;
    // Vec data structure
    let items = vec!["apple", "strawberry", "camel", "rust", "cowboy", "dinosaur"];
    let mut hungman = 0;

    let mut rng = thread_rng();
    let random_item = *items.choose(&mut rng).unwrap(); // expression to select random word from list

    // println!(" (debug) word: {}", random_item); //for debugging

    let mut guess = String::new();

    // starts with number of letters in the word as underscores
    for _l in random_item.chars() {
        guess.push('_');
    }
    println!();
    println!("Hangman game");

    // game loop
    while is_running {
        println!("Enter a letter.");
        println!("------------------------------");

        let mut letter_guess = String::new();

        std::io::stdin()
            .read_line(&mut letter_guess)
            .expect("Failed to read line");
        
        let letter_guess = letter_guess.trim();

        // check if the input is a single letter
        if letter_guess.len() != 1 || !letter_guess.chars().all(|c| c.is_alphabetic()) {
            println!("Please enter a single letter.");
            continue;
        }

        // check if the guessed letter is in the word and replace underscore with that letter
        if random_item.contains(letter_guess) {
            for (i, c) in random_item.chars().enumerate() {
                if c == letter_guess.chars().next().unwrap() {
                    guess.replace_range(i..i + 1, &letter_guess);
                }
            }
            println!();
            println!("You guessed right!");
        } else {
            hungman += 1; // add 1 to hangman counter
            println!();
            println!("Wrong guess!");
        }

        // run stick figure function
        let stick_figure = get_stick_figure(hungman);

        println!();
        println!("Hangman: {}", stick_figure);
        println!("{}", guess);

        // conditionals to win or lose

        if hungman == 6 { // if all hangman parts are drawn, the game is lost
            println!("------------------------------");
            println!("You lost!");
            println!("------------------------------");
            break;
        }

        if guess == random_item { //if all letters from the word are guessed, the game is won
            println!("------------------------------");
            println!("You won!");
            println!("------------------------------");
            break;
        }
    }
}

// draw hangman based on number of guesses
fn get_stick_figure(hungman: i32) -> &'static str {
    match hungman {
        1 => "O",
        2 => "O--",
        3 => "O--/",
        4 => "O--<",
        5 => "O-_-<",
        6 => "💀-|-<",
        _ => "",
    }
}
