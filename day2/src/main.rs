use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    first_part();
    second_part();
}

fn second_part(){
    let mut scores = HashMap::new();
    let mut score: u32 = 0;
    scores.insert("A X".to_string(),0+3); // Rock Lose Scissors
    scores.insert("A Y".to_string(),3+1); // Rock draw rock
    scores.insert("A Z".to_string(),6+2); // Rock win paper
    scores.insert("B X".to_string(),0+1); // Paper lose rock
    scores.insert("B Y".to_string(),3+2); // Paper draw paper
    scores.insert("B Z".to_string(),6+3); // Paper win Scissors
    scores.insert("C X".to_string(),0+2); // Scissors lose paper
    scores.insert("C Y".to_string(),3+3); // Scissors draw Scissors
    scores.insert("C Z".to_string(),6+1); // Scissors win rock

    if let Ok(lines) = read_lines("input\\input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                score += scores.get(&ip).copied().unwrap_or(0);
            }
        }
    }
    println!("Score:{}",score);
}
fn first_part(){
    let mut scores = HashMap::new();
    let mut score: u32 = 0;
    scores.insert("A X".to_string(),1+3); // Kamien Kamien
    scores.insert("A Y".to_string(),2+6); // Kamien Papier
    scores.insert("A Z".to_string(),3+0); // Kamien Nozyce
    scores.insert("B X".to_string(),1+0); // Papier kamien
    scores.insert("B Y".to_string(),2+3); // Papier Papier
    scores.insert("B Z".to_string(),3+6); // Papier Nozyce
    scores.insert("C X".to_string(),1+6); // Nozyce Kamien
    scores.insert("C Y".to_string(),2+0); // Nozyce Papier
    scores.insert("C Z".to_string(),3+3); // Nozyce Nozyce

    if let Ok(lines) = read_lines("input\\input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                score += scores.get(&ip).copied().unwrap_or(0);
            }
        }
    }
    println!("Score:{}",score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}