use std::io;
use rand::seq::SliceRandom;
use std::str::Chars;

/// The "CarryOn()" function is used for determining if the user wants to add more words to the glossary.
/// The user can input 1 to continue, or any other number to quit.
/// This function returns the number the player entered to the main function.
fn CarryOn()->i8{
    println!("Do you want to continue ending words to the glossary?");
    println!("Press 1 for yes, enter any other number for no.");
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).expect("Failed");
    let n1:i8 = s1.trim().parse().expect("Not a valid Number");
    return n1;
}

/// The "OrderLetters(&mut Chars, &mut Vec<chars>" function is a somewhat complicated function.
/// The Vector passed through gets compared with the Chars of the word being guessed in hang man.
/// If an element of the vector matches up with a character from the word being guessed,
/// that element has whatever letter it is printed.
/// Otherwise, a blank ' _ ' is printed where a correct letter would be.
/// This function prints the correctly guessed letters in order of how they are in the correct word, 
/// to make it easier for the user/player to guess what the word is.
/// This function is void.
fn OrderLetters(chars: &mut Chars, vector: &mut Vec<char>){
    for c in &mut *chars{
        let mut found: bool = false;
        for i in vector.iter_mut() {
            if *i == c{
                print!("{} ", i);
                found = true;
            }
        }
        if found == false {
            print!(r"_ ");
        }
    }
}

/// The "Guess() function is used to prompt the user/player for a guess of a letter of the unknown word."
/// When the player/user enters in a guess, the function will only take the first letter/character of their guess,
/// To ensure that they did not enter in too many letters.
/// This function returns their guess to the function that called it.
fn Guess() -> char
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess = guess.chars().nth(0).unwrap();
    return guess;
}

/// The "CheckGuess()" function is used to see if the player's guess matches a character of the word being used.
/// If it is, the function will check how many times that letter shows up in the word, and will retun that value.
fn CheckGuess(word: String, guess: char) -> usize{
    let mut CorrectLetter: usize = 0;
    for i in 0..word.len() {
        if word.chars().nth(i).unwrap() == guess {
            CorrectLetter += 1;
        }
    }
    return CorrectLetter;
}

/// This function checks a vector passed through it to see if a character that is also passed through it,
/// is within the vector. If it is, return a true bool value, and if not, return a false bool value.
fn CheckVector(vector: &mut Vec<char>, letter: char) -> bool{
    let mut RValue = false;
    for i in vector.iter_mut() {
       if *i == letter {
        RValue = true;
       }
    }
    return RValue;
}


/// This is the main function that handles the game. It prompts the user/player over and over again,
/// asking for guesses of what letters might be in the correct word. The function will keep prompting these questions
/// and evaluating what the player inputs, until the player either wins the game or loses it.
/// This script will display what letters the player has already used, which are wrong, and which are right.
/// The script will also tell the player if they have already guessed a letter or not.
/// In this function you will see comments of some old code I used before I moved onto better tactics.
fn GameOn(word: String){
    let mut CorrectLetter: Vec<char> = Vec::new();
    let mut TotalLetter: Vec<char> = Vec::new();
    let mut WrongLetter: Vec<char> = Vec::new();
    let mut InProgress: bool = true;
    let mut TotalCorrectLetters: usize = 0;
    let mut lives: i16 = 6;
    println!("The word has {} letters. Guess!", word.len());
    while InProgress == true {
        let guess = Guess();
        if CheckVector(&mut TotalLetter,guess) == false {
            println!("guess: {}", CheckGuess(word.clone(),guess));
            let RightLetters: usize = CheckGuess(word.clone(),guess);
            if RightLetters == 0 {
                lives = lives - 1;
                println!("Incorrect guess! You have {} lives left...", lives);
                TotalLetter.push(guess);
                WrongLetter.push(guess);
            } else {
                println!("Correct! Your guess {}, shows a total of {} times.", guess, CheckGuess(word.clone(),guess));
                TotalCorrectLetters = TotalCorrectLetters + CheckGuess(word.clone(),guess);
                /*
                for i in 0..CheckGuess(word.clone(),guess) {
                    CorrectLetter.push(guess);
                }
                */ 
                CorrectLetter.push(guess);
                TotalLetter.push(guess);
            }
            HangMan(lives);
            println!("Wrongly guessed letters:");
            for i in &WrongLetter {
                print!(" {}", i);
            }
            println!("");
            println!("Correctly guessed letters:");
            /*
            for i in &CorrectLetter {
                print!(" {}", i);
            }
            */
            OrderLetters(&mut word.chars(), &mut TotalLetter);
            println!("");
            println!("You have {} tries left!", lives);
            if lives <= 0 {
                InProgress = false;
                println!("Sorry, you lose!");
            } else if TotalCorrectLetters == word.len() {
                println!("You correctly guessed all the letters to the word {} ! Congratultions! You won!", word);
                InProgress = false;
            }
        } else if CheckVector(&mut TotalLetter,guess) == true {
            println!("You've already guessed {} !", guess);
        }    
    }
}

/// Main function of the program. Program starts with the basic question of "what is your name?"
/// The function then goes right into the point of this script. The main function asks the player to input
/// some words that they would like to be featured in the game, and the script will randomly pick one of those words
/// and tell the player how many letters/characters it has.
/// The script will then call the GameOn() function to begin the game. Once the game ends, no matter the outcome,
/// the script will thank the player for playing.
fn main(){
    let mut words: Vec<String> = Vec::new();
    println!("Welcome to hangman, the game! What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed");
    println!("Welcome {}",name);
    let mut LastWord = 1;
    let mut WordInput = String::new();
    words.push("hello".to_string()); // backup, just in case the user doesn't add a word.
    while LastWord == 1 {
        println!("Please type a word to add to the hangman glossary: ");
        io::stdin().read_line(&mut WordInput).expect("Failed");
        WordInput.make_ascii_lowercase();
        words.push(WordInput.clone());
        LastWord = CarryOn();
        WordInput.clear();
    }
    //println!("vector length: {}",words.len()); checking length
    println!("Here are your words!");
    for i in &words {
        // iterate immutably
        let i = String::from(i); // elements are immutable pointers
        println!("{}", i.trim());
    }
    println!("One of these words will be used for the hangman game. You have 6 guesses!");
    let string = words.choose(&mut rand::thread_rng());
    let string = string.unwrap().trim();
    let string = string.to_string();
    GameOn(string.clone());
    println!("Thanks for playing!");
}

/// This function displays the status of the hangman, depending on how many guesses/lives/mistakes left the player has.
/// With each mistake the player mistakes, a new body part of the hangman will be displayed, entire the entire body is shown,
/// and the player loses the game.
/// This function is void.
fn HangMan(mistakes: i16){
    if mistakes == 6 {
        println!("----|");
        println!("    |");
        println!("    |");
        println!("    |");
        println!("    |");
    } else if mistakes == 5 {
        println!("----|");
        println!(" o  |");
        println!("    |");
        println!("    |");
        println!("    |");
    } else if mistakes == 4 {
        println!("----|");
        println!(" o  |");
        println!(" |  |");
        println!("    |");
        println!("    |");
    } else if mistakes == 3 {
        println!("----|");
        println!(" o  |");
        println!("/|  |");
        println!("    |");
        println!("    |");
    } else if mistakes == 2 {
        println!(r"----|");
        println!(r" o  |");
        println!(r"/|\ |");
        println!(r"    |");
        println!(r"    |");
    } else if mistakes == 1 {
        println!(r"----|");
        println!(r" o  |");
        println!(r"/|\ |");
        println!(r"/   |");
        println!(r"    |");
    } else if mistakes <= 0 {
        println!(r"----|");
        println!(r" o  |");
        println!(r"/|\ |");
        println!(r"/ \ |");
        println!(r"    |");
    }
}
