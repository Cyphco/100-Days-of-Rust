use std::io::{stdin};



fn main() {

    let user_input = get_input();
    println!("Input: {}", user_input);

    let find_pos = find_nemo(user_input);
    println!("Nemo was found at Position: {}", find_pos);


}




fn get_input() -> String {

    println!("Enter a string of text:");
    let mut input = String::new();
    while input.is_empty() {
        stdin().read_line(&mut input).unwrap();
    }
    input

}

fn find_nemo(input: String) -> i32 {
    let mut count = 0;
    let mut found = false;

    while !found{
        for word in input.split_whitespace() {
            if word == "Nemo" {
                found = true;
                count += 1;
                break;
            }
            count += 1;
        }
    }
    count
}