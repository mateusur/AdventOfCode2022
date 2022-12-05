use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ptr::{null, null_mut};

fn main() {
    let mut sum=0;
    if let Ok(lines) = read_lines("input\\input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut first=-99;
                let mut second=-99;
                let mut third=-99;
                let mut fourth=-99;
                let mut split = ip.split('-');
                for s in split {
                    let mut split2 = s.split(',');
                    for s2 in split2{
                        print!("{} ", s2);
                        if first==-99{
                            first = s2.parse().unwrap();
                        }
                        else if second==-99 {
                            second = s2.parse().unwrap();

                        }
                        else if third==-99 {
                            third = s2.parse().unwrap();

                        }
                        else  {
                            fourth = s2.parse().unwrap();
                            if (third >= first && third<=second) || (fourth >=first &&fourth<=second){
                                sum+=1;
                                print!("Sum +1");
                            }
                            else if (first>=third && first <=fourth)  || (second>=third && second<=fourth) {
                                sum+=1;
                                print!("Sum +1");
                            }


                            first=-99;
                            second=-99;
                            third=-99;
                            fourth=-99;

                        }

                    }
                }
                println!("");
            }

        }
    }
    println!("Sum:{}",sum);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}