use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {

    println!("Day 2 hour 0\nNo idea what I'm doing");

    const RED_MAX :i32 = 12;
    const GREEN_MAX :i32 = 13;
    const BLUE_MAX :i32 = 14;

    let mut total = 0;
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("red"), 0);
    scores.insert(String::from("green"), 0);
    scores.insert(String::from("blue"), 0);

    // File hosts.txt must exist in the current path
    let Ok(lines) = read_lines("./input.txt") else  {
            println!("Error reading file");
            return;
        };

    // Consumes the iterator, returns an (Optional) String

    for line in lines {
        let Ok(game) = line else{continue;};
        
        

        println!("{}", game);
    }

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("Total: {}", total);
    // println!("Red {}", scores.get("red").unwrap());

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}