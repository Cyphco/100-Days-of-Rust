use std::io::{stdin};

mod simple;


fn main() {	
    println!("How would you like to convert your age to days?");
    println!("1. Simple (365 days per year going off Age)");
    println!("2. More Accurate (365 days per year Going off Birth date)");
    println!("3. Precise (including leap years and days between last birthday and now)");

    let mut input_selection = String::new();
    while input_selection.is_empty(){
        stdin().read_line(&mut input_selection).unwrap();
    }

    match input_selection.trim(){
        "1" => simple::simple(),
        "2" => println!("Not implemented yet"),
        "3" => println!("Not implemented yet"),
        _ => println!("Invalid input"),
    }



}