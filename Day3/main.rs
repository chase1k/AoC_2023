use std::fs::read_to_string;
use std::str::Chars;

fn main() {
    println!("Day 3 hour 0\nStill no idea what I'm doing");

    let mut result = Vec::new();
    let mut total = 0;

    // read all lines
    for line in read_to_string("input.txt").unwrap().lines() {
        result.push(line.to_string())
    }

    // looping through file
    for (line_index, _line) in result.iter().enumerate() {

        // Need to grab second line in file to not error
        let Some(current_line) = result.get(line_index + 1) else{
            println!("End of file");
            println!("Total {}", total);
            break;
        };

        println!("Line {}: {}", line_index + 1, current_line);

        // loop through characters 
        for (char_index, character) in current_line.chars().enumerate() {

            if character.is_digit(10) || character == '.' {continue;}

            // println!("Symbol: {}", character);
            // loop through current line
            // println!("Checking current");
            total = symbol_check(char_index as usize, current_line.chars(), total as i32);

            // loop through previous line
            let Some(previous_line) = result.get(line_index) else{
                println!("Previous line not found, should never reach this");
                return;
            };

            // println!("Checking up");
            total = symbol_check(char_index as usize, previous_line.chars(), total as i32);

            // loop through next
            let Some(next_line) = result.get(line_index + 2) else{
                println!("End of file");
                println!("Total {}", total);
                return;
            };

            // println!("Checking down");
            total = symbol_check(char_index as usize, next_line.chars(), total as i32);
            // println!("Symbol checked, Total {}", total);
        }
        // println!("Total {}", total);
    }
}

fn symbol_check(symbol_index: usize, mut line: Chars , mut total: i32) -> i32{

    let mut number_to_add: String = String::new();

    // Checking back three from symbol
    // Get numbers above symbol, concat and add them

    // If third left is number
    if let Some(digit) = line.nth(symbol_index - 3) {

        // println!("Left character: {}", digit);
        if digit.is_digit(10) {
            number_to_add = format!("{}{}", number_to_add, digit);
    
            // if second back is number
            if let Some(digit) = line.nth(0) {

                if digit.is_digit(10) {
                    number_to_add = format!("{}{}", number_to_add, digit);
                    
                    // if 1st back is number
                    if let Some(digit) = line.nth(0) {
                        if digit.is_digit(10) {
                            number_to_add = format!("{}{}", number_to_add, digit);
                        }else{
                            number_to_add = String::new(); 
                        }
                    // if 1st back isn't number reset number to add
                    }

                // if second back isn't reset number to add
                }else{
                    number_to_add = String::new();
                    if let Some(digit) = line.nth(0) {
                        if digit.is_digit(10) {
                            number_to_add = format!("{}{}", number_to_add, digit);
                        }
                    }
                }
            } 
        }else{
            // if third left isn't number check 2nd
            if let Some(digit) = line.nth(0) {
                if digit.is_digit(10) {
                    number_to_add = format!("{}{}", number_to_add, digit);

                    // if 2nd back is number check 1st back
                    if let Some(digit) = line.nth(0) {
                        if digit.is_digit(10) {
                            number_to_add = format!("{}{}", number_to_add, digit);
                        // if 1st back isn't number reset number to add
                        }else{
                        number_to_add = String::new(); 
                        }
                    }
                // if 2nd back isn't check 1st back
                }else{
                if let Some(digit) = line.nth(0) {
                    if digit.is_digit(10) {
                        number_to_add = format!("{}{}", number_to_add, digit);
                        }
                    }
                }
            }
        }
    }
    
    // If middle isn't a '.' or other symbol
    if let Some(digit) = line.nth(0){
        // println!("Middle character: {}", digit);
        if digit.is_digit(10){
            number_to_add = format!("{}{}",number_to_add, digit);
        }else{
            if let Ok(n) = number_to_add.parse::<i32>(){
                println!("Number vert left: {}", n);
                total += n;
                number_to_add = String::new();
            }
        }
    }

    // println!("Checking line: {}", line.as_str());
    if let Some(digit) = line.nth(0) {
        if digit.is_digit(10) {
            number_to_add = format!("{}{}", number_to_add, digit);
    
            if let Some(digit) = line.nth(0) {
                if digit.is_digit(10) {
                    number_to_add = format!("{}{}", number_to_add, digit);
    
                    if let Some(digit) = line.nth(0) {
                        if digit.is_digit(10) {
                            number_to_add = format!("{}{}", number_to_add, digit);
                        }
                    }
                }
            }
        }
    }
    
    if let Ok(n) = number_to_add.parse::<i32>(){
        println!("Number vert right: {}", n);
        total += n;
    }

    return total;
}