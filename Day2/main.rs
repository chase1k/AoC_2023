use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashMap;

fn main() {

    println!("Day 2 hour 0\nNo idea what I'm doing");

    const RED_MAX :i32 = 12;
    const GREEN_MAX :i32 = 13;
    const BLUE_MAX :i32 = 14;

    let mut total = 0;
    // let mut scores: HashMap<String, i32> = HashMap::new();

    // scores.insert(String::from("red"), 0);
    // scores.insert(String::from("green"), 0);
    // scores.insert(String::from("blue"), 0);

    // File hosts.txt must exist in the current path
    let Ok(lines) = read_lines("./input.txt") else  {
            println!("Error reading file");
            return;
        };

    // Consumes the iterator, returns an (Optional) String

    for line in lines {
        let Ok(game) = line else{continue;};

        let mut red_highest = 0;
        let mut green_highest = 0;
        let mut blue_highest = 0;
        
        let no_game = &game[game.find(':').unwrap()+2..];
        let semicolon_split_values = no_game.split(';');

        for run in semicolon_split_values{

            let comma_split_values = run.split(',');
            for mut pair in comma_split_values{

                pair = pair.trim();
                if pair.contains("red"){
                    
                    // println!("Pair: {}", pair);
                    if red_highest < pair.split(' ').nth(0).unwrap().parse::<i32>().unwrap(){
                        red_highest = pair.split(' ').nth(0).unwrap().parse::<i32>().unwrap();
                    }
                } else if pair.contains("green"){
    
                    // println!("Pair: {}", pair);
                    if green_highest < pair.split(' ').nth(0).unwrap().parse::<i32>().unwrap(){
                        green_highest = pair.split(' ').nth(0).unwrap().parse::<i32>().unwrap();
                    }
                } else if pair.contains("blue"){
    
                    // println!("Pair: {}", pair);
                    // println!("Pair: {}", pair.split(' ').nth(0).unwrap());
                    if blue_highest < pair.split(' ').nth(0).unwrap().parse::<i32>().unwrap(){
                        blue_highest = pair.split(' ').nth(0).unwrap().parse::<i32>().unwrap();
                    }
                }
            }

            // println!("Red: {}", red_total);
            // println!("Green: {}", green_total);
            // println!("Blue: {}", blue_total);

        }

        if red_highest <= RED_MAX && green_highest <= GREEN_MAX && blue_highest <= BLUE_MAX{
            total += game.split(':').nth(0).unwrap().split(' ').last().unwrap().parse::<i32>().unwrap();
            println!("Total added: {}, Game: {}", total, game.split(':').nth(0).unwrap().split(' ').last().unwrap().parse::<i32>().unwrap());
        }

        // println!("Total: {}", total);
    }


    println!("Total: {}", total);
    // println!("Red {}", scores.get("red").unwrap());

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}