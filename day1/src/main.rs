use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let mut num_of_elves: i32 = 0;
    let mut calories = 0;
    let mut first_max_calories = 0;
    let mut second_max_calories = 0;
    let mut third_max_calories = 0;

    if let Ok(lines) = read_lines("input\\input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty(){
                    num_of_elves += 1;
                    if calories > first_max_calories {
                        third_max_calories = second_max_calories;
                        second_max_calories = first_max_calories;
                        first_max_calories = calories;
                    }
                    else if  calories > second_max_calories {
                        third_max_calories = second_max_calories;
                        second_max_calories = calories;
                    }
                    else if calories > third_max_calories {
                        third_max_calories = calories;
                    }
                    println!("Calories:{}, First max:{}, second:{}, third:{} ",calories,first_max_calories,second_max_calories,third_max_calories);
                    calories = 0;
                }
                else {
                    // println!("{}", ip);
                    let my_int = ip.parse::<i32>().unwrap();
                    calories += my_int;
                }

            }
        }
    }
    println!("Number of elves {}",num_of_elves);
    let sum_of_calories = first_max_calories+second_max_calories+third_max_calories;
    println!("First calories {}, second {}, third {} and the sum {}", first_max_calories, second_max_calories, third_max_calories,sum_of_calories);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
