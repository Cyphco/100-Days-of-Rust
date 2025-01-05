use std::io::stdin;

// Gets user input and validates it
pub fn get_input() -> Vec<i32> {
    // Loops until valid input is provided
    let output = loop {
        println!("Input a series of numerals:");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let output = string_to_vec(input);
        let status = check_input(&output);
        match status {
            (true, 0) => println!("Input is valid"),
            (false, 1) => println!("Input is too long"),
            (false, 2) => println!("Input is too large"),
            _ => println!("Error"),
        }
        match status.0 {
            true => break output,
            false => continue,
        }
    };
    return output;
}

// Converts a string to a vector of integers
fn string_to_vec(input: String) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];

    // Check if the input contains commas
    if input.contains(',') {
        // Handle comma-separated values
        for segment in input.trim().split(',') {
            if let Ok(num) = segment.trim().parse::<i32>() {
                output.push(num);
            } else {
                println!("Error: Couldn't parse segment '{}' as i32", segment);
            }
        }
    } else {
        // Handle single-digit sequences
        for ch in input.trim().chars() {
            if let Ok(num) = ch.to_string().parse::<i32>() {
                output.push(num);
            } else if !ch.is_whitespace() {
                println!("Error: Invalid character '{}'", ch);
            }
        }
    }

    println!("Output: {:?}", output);
    return output;
}

// Checks if the input is valid
fn check_input(input: &Vec<i32>) -> (bool, i32) {
    // Checks if the input length is greater than 312
    if input.len() > 312 {
        return (false, 1);
    }
    // Checks if the max value is greater than 105
    let max_value = input.iter().max();

    if max_value > Option::Some(&105) {
        return (false, 2);
    } else {
        return (true, 0);
    }
}
