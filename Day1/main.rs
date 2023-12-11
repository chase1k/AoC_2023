use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    println!("Day 1 hour 0\nNo idea what I'm doing");

    // Open file and initialize total
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total= 0;

    // For every line in file
    for line in reader.lines() {
        // println!("{}", line?);

        // Initializing first and last number
        let mut first_num: char = 'a';
        let mut last_num: char = 'a';

        // Char array
        let char_vec: Vec<char> = line?.chars().collect();

        // For every char in array
        for character in char_vec{
            // println!("{}", character);

            // Is valid number
            if character.is_digit(10){
                // println!("{}", character);
                
                // Set first and last num
                match first_num{
                    'a' => {
                        first_num = character;
                        last_num = character;
                    }
                    _ => {
                        last_num = character;
                    }
                }
            }
        }
        // println!("fnum: {} lnum: {}", first_num.unwrap(), last_num.unwrap());

        // Convert char to String
        let combined_number = format!("{}{}",first_num, last_num);
        
        // println!("Combined number: {}", combined_number);

        // Convert String to int then add
        total += combined_number.parse::<i32>().unwrap();
    }   

    println!("Total: {}", total);
    Ok(())
    
}

