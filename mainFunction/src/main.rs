use std::io;
use rand::seq::SliceRandom;
use std::str::Chars;

fn CarryOn()->i8{
    println!("Do you want to continue ending words to the glossary?");
    println!("Press 1 for yes, enter any other number for no.");
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).expect("Failed");
    let n1:i8 = s1.trim().parse().expect("Not a valid Number");
    return n1;
}

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

fn Guess() -> char
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess = guess.chars().nth(0).unwrap();
    return guess;
}

fn CheckGuess(word: String, guess: char) -> usize{
    let mut CorrectLetter: usize = 0;
    for i in 0..word.len() {
        if word.chars().nth(i).unwrap() == guess {
            CorrectLetter += 1;
        }
    }
    return CorrectLetter;
}

fn CheckVector(vector: &mut Vec<char>, letter: char) -> bool{
    let mut RValue = false;
    for i in vector.iter_mut() {
       if *i == letter {
        RValue = true;
       }
    }
    return RValue;
}

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
