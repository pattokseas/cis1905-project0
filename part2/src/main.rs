// you'll need these two imports to read input form the user
use std::io;
use std::io::Write;
// additional imports
use std::fs;
use rand::Rng;

/**
 * prompt the user for a letter, re-prompting until a valid input is recieved
 * @param prompt - printed before reading the letter
 * @param tried - an array of flags for letters tried. tried[a] is true iff 'a'
 *  has been tried, etc
 * @return the first letter given by the user
 * */
fn get_guess(prompt: &str, tried: &[bool]) -> char {
    print!("{}", prompt);
    // Make sure the prompt from the previous line gets displayed:
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");
    // look at the first character of user input
    match guess.to_lowercase().chars().nth(0) {
        // if it's a letter, it might be a valid input
        Some(c) if c >= 'a' && c <= 'z' => {
            // but we need to check that they haven't already tried that letter
            if tried[(c as usize) - ('a' as usize)] {
                get_guess("You already tried that letter. Try a different one: ", tried)
            } else { c }
        }
        // if it's not a letter, reprompt
        _ => { get_guess("Please enter a valid letter to guess: ", tried) }
    }
}

/** chooses a word from a file containing a list of words, defaulting to 
 * "lobster" if choosing from the file fails
 * @param filename - the file containing the word list
 * @param min_length - the minimum length for a chosen word
 * @param max_length - the maximum length
 * @return a word from the list, or "lobster"
 * */
fn choose_word(filename: &str, min_length: usize, max_length: usize) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            // get list of words and choose a position in the list
            let words: Vec<&str> = contents.split_whitespace().collect();
            let mut rng = rand::rng();
            let mut index: usize = rng.random_range(0..words.len());
            // try 1000 times to get a valid word
            let mut attempts = 1;
            while words[index].len() < min_length || words[index].len() > max_length {
                index = rng.random_range(0..words.len());
                attempts += 1;
                // default to "lobster" if a valid word isn't found
                if attempts > 1000 { return String::from("lobster"); }
            }
            // return the chosen word
            String::from(words[index].to_lowercase())
        }
        // default to "lobster"
        _ => String::from("lobster")
    }
}

/** covers a word according to discovered letters, checking if it's been 
 * completely discovered
 * @param word: the word to be covered
 * @param tried: an array of flags for letters tried. tried[a] is true iff 'a' 
 *  has been tried, etc
 * @return a tuple with the covered word and a flag which is true iff the 
 *  entire word is uncovered
 * */
fn cover_word(word: &str, tried: &[bool; 26]) -> (String, bool) {
    /* strategy: initialize to all ------, then go through each position in the
     * word, and if that letter has been tried, uncover it */
    let mut covered = vec!['-'; word.len()];
    let word_chars = word.chars();
    let mut all_uncovered = true;
    let mut i = 0;
    for c in word_chars {
        if tried[(c as usize) - ('a' as usize)] {
            covered[i] = c;
        } else {
            all_uncovered = false;
        }
        i += 1;
    }
    (covered.into_iter().collect(), all_uncovered)
}

fn main() {
    // initialize game state
    // word list from https://www.mit.edu/~ecprice/wordlist.10000
    let word = choose_word("wordlist.10000", 5, 9);
    let mut tried = [false; 26];
    let mut guesses = 5;
    
    println!("Welcome to cis1905 Hangman!");

    while guesses > 0 {
        // get game status and print to user
        let (covered, won) = cover_word(&word, &tried);
        if won { break; }
        println!("The word so far is {}", covered);
        println!("You have {} guess{} left", guesses, if guesses > 1 {"es"} else {""});
        // get the guess and update game state
        let c = get_guess("Please guess a letter: ", &tried);
        tried[(c as usize) - ('a' as usize)] = true;
        // if the letter isn't in the word, give feedback and take a guess
        if !word.contains(c) {
            println!("Sorry, that letter is not in the word");
            guesses -= 1;
        }
        // go to the next round
        println!("");
    }

    // if the game ended without running out of guesses, they won
    if guesses > 0 {
        println!("Congratulations you guessed the secret word: {}!", word);
    } else {
        println!("Sorry, you ran out of guesses!");
        println!("The word was \"{}\"", word);
    }

}
