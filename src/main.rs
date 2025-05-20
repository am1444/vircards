#![allow(non_snake_case)] // No warnings when using camelCase
use std::io;
use rand::Rng;

fn numInVec(x:usize, checkVec:Vec<i32>) -> bool {
    for checkIndex in 0..checkVec.len() {
        if checkVec[checkIndex] == x as i32 {
            return true;
        }
    }
    return false;
}

fn main() {
    //let si:i32 = rand::rng().random_range(0..100); example
    
    let clargs:Vec<String> = std::env::args().collect();
    let mut readPath:String = String::new();
    if clargs.len() < 2 {
        println!("Enter card file path:");
        io::stdin().read_line(&mut readPath).expect("Error reading user input.");
        readPath = readPath.trim().to_string();
    } else {
        readPath = clargs[1].clone();
    }

    let cardFileTextContentsRaw = std::fs::read_to_string(readPath).expect("rdfile_ld err"); // Load cardfile
    let cardFileContentsRaw = cardFileTextContentsRaw.trim().split("\n\n").collect::<Vec<&str>>(); // Split cardfile into cards
    let mut cardFileContents:Vec<Vec<&str>> = Vec::new();
    let cardCount = cardFileContentsRaw.len();
    for index in 0..cardCount { // Split cards into terms, definitions, and modes
        cardFileContents.push(cardFileContentsRaw[index].split("\n").collect::<Vec<&str>>());
    } // Cards are now stored as subvectors in vector cardFileContents.

    let mut currentCardIndex:usize;
    let mut forwardDir:bool;
    let mut doneCardsIndexes:Vec<i32> = Vec::new(); // Indexes of cards that the user has mastered and so that will not be used anymore.
    //Main loop
    while doneCardsIndexes.len() < cardCount {
        clearscreen::clear().expect("failed to clear screen");
        currentCardIndex = rand::rng().random_range(0..cardCount);
        while numInVec(currentCardIndex,doneCardsIndexes.clone()) { // Make sure chosen card isn't mastered
            currentCardIndex = rand::rng().random_range(0..cardCount);
        }
        let currentCard = cardFileContents[currentCardIndex].clone(); // Get which card will be needed.
        let term = currentCard[0];
        let def = currentCard[1];
        if currentCard[2] == "BOTHDIR" {
            forwardDir = rand::rng().random_bool(0.5);
        } else if currentCard[2] == "FORDIR" {
            forwardDir = true;
        } else {
            forwardDir = false;
        }
        
        if forwardDir {
            println!("\n{}",term);
        } else {
            println!("\n{}",def);
        }

        let mut buf:String = String::new();
        io::stdin().read_line(&mut buf).expect("pause err");

        if forwardDir {
            println!("{}",def);
        } else {
            println!("{}",term);
        }

        buf = String::new();
        io::stdin().read_line(&mut buf).expect("uinput err");
        buf = buf.trim().to_string();
        
        if buf == "g" { // If the user says they've got it right
            doneCardsIndexes.push(currentCardIndex.try_into().unwrap());
        }
    }
    println!("done!!! congrats");
}