use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one();
    part_two();
}
fn part_one(){
    static ASCII_ALPHABET: [char; 52] = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y',
        'z','A','B','C','D','E',
        'F','G','H','I','J','K',
        'L','M','N','O','P','Q',
        'R','S','T','U','V','W',
        'X','Y','Z'
    ];
    let mut sum = 0;
    let mut all_char: Vec<char> = Vec::new();
    if let Ok(lines) = read_lines("input\\input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut word:String = String::from(ip);
                let word_count = word.chars().count();
                let (first_half,second_half)= word.split_at_mut(word_count/2);
                let mut vec_of_chars:Vec<char> = Vec::new();
                for cf in first_half.chars(){
                    for cs in second_half.chars(){
                        if cf ==cs {
                            vec_of_chars.push(cf);
                            break;
                        }
                    }
                }
                vec_of_chars.dedup();
                for c in vec_of_chars{
                    all_char.push(c);
                }
            }
        }
    }
    for c in all_char{
        for (index,ch) in ASCII_ALPHABET.iter().enumerate(){
            if c == *ch {
                sum+=index+1;
            }
        }
    }
    println!("Sum part one: {}",sum);
}

fn part_two(){
    static ASCII_ALPHABET: [char; 52] = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y',
        'z','A','B','C','D','E',
        'F','G','H','I','J','K',
        'L','M','N','O','P','Q',
        'R','S','T','U','V','W',
        'X','Y','Z'
    ];
    let mut sum = 0;
    let mut counter =1;
    let mut first_line = String::new();
    let mut second_line = String::new();
    let mut third_line = String::new();
    let mut group_vec_of_chars:Vec<char> = Vec::new();
    if let Ok(lines) = read_lines("input\\input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if counter == 1{
                    first_line = ip;
                    counter +=1;
                }
                else if counter == 2 {
                    second_line= ip;
                    counter +=1;
                }
                else {
                    third_line = ip;
                    counter = 0;
                }
                if counter == 0{

                    'outer: for cf in first_line.chars(){
                        for cs in second_line.chars(){
                            for ct in third_line.chars(){
                                if cf ==cs && cf == ct {
                                    group_vec_of_chars.push(cf);
                                    break 'outer;
                                }
                            }
                        }
                    }
                    counter+=1;
                }
            }
        }
    }
    for c in group_vec_of_chars{
        for (index,ch) in ASCII_ALPHABET.iter().enumerate(){
            if c == *ch {
                // println!("First char {} and second {} and value {}",c,ch,index+1);
                sum+=index+1;
            }
        }
    }
    println!("Sum part two: {}",sum);
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
