use std::io::{stdin};
use std::collections::HashMap;


fn main() {

    println!("Input the Squence of Socks!");
    let mut user_input: String = String::new();
    while user_input.is_empty(){
        stdin().read_line(&mut user_input).unwrap();
    }

    //Use Modulo 2 to get the number of socks in pairs and Print a list of how many of each there are
    let pairings = pair(&user_input);
    for (key, value) in pairings.iter(){
        println!("{}: {}", key, value/2);
    }
    


}


fn pair(input:&str) -> HashMap<char, i32> {

    let  array = convert_to_array(input);
    let mut pairings= HashMap::new();

    for i in array{
        *pairings.entry(i).or_insert(0) += 1;
    }
    pairings

}


fn convert_to_array(input:&str) -> Vec<char> {
    let mut array: Vec<char> = vec![];
    for c in input.trim().chars(){
        array.push(c);
    }
    array
}