fn main() {

    let user_input = get_user_input();
    let user_input_array = string_to_array(user_input);
    let combinations = get_combinations(user_input_array);
    for i in combinations {
        print!("{}", i);
    }
}

fn get_combinations(user_input_array: Vec<char>) -> Vec<String> {
    
    let mut letters_1: Option<Vec<char>> = None;
    let mut letters_2: Option<Vec<char>> = None;
    let mut letters_3: Option<Vec<char>> = None;
    let mut letters_4: Option<Vec<char>> = None;

    for i in user_input_array {
        if letters_1.is_none() {
            letters_1 = Some(get_letters(i));
        } else if letters_2.is_none() {
            letters_2 = Some(get_letters(i));
        } else if letters_3.is_none() {
            letters_3 = Some(get_letters(i));
        } else if letters_4.is_none() {
            letters_4 = Some(get_letters(i));
        }
    }

    let mut combined_letters: Vec<String> = Vec::new();
    if let (Some(l1), Some(l2), Some(l3), Some(l4)) = (&letters_1, &letters_2, &letters_3, &letters_4) {
        for i in l1 {
            for j in l2 {
                for k in l3 {
                    for l in l4 {
                        let combination = format!("{}{}{}{},", i, j, k, l);
                        combined_letters.push(combination);
                    }
                }
            }
        }
    }
    combined_letters
}

fn get_letters(letter: char) -> Vec<char> {
    match letter {
        '2' => vec!['a','b','c'],
        '3' => vec!['d','e','f'],
        '4' => vec!['g','h','i'],
        '5' => vec!['j','k','l'],
        '6' => vec!['m','n','o'],
        '7' => vec!['p','q','r','s'],
        '8' => vec!['t','u','v'],
        '9' => vec!['w','x','y','z'],
        _ => vec![],

    }
}

fn string_to_array(user_input: String) -> Vec<char> {
    let mut user_input_char = Vec::new();
    for i in user_input.chars() {
        user_input_char.push(i);
    }
    user_input_char
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    println!("Please enter a 2-4 digit number");
    while user_input.is_empty() {
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input = user_input.trim().to_string();
        if user_input.len() > 4 || user_input.is_empty() || user_input.contains('1') || user_input.contains('0') {
            user_input = String::new();
            println!("Please enter a valid input");
        }
    }
    user_input
}
