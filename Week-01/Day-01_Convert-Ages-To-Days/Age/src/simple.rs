use std::io::{stdin};


pub fn simple (){

    println!("Enter your current age: ");
    let mut input = String::new(); 
    while input.is_empty(){ 
        stdin().read_line(&mut input).unwrap();
    }
    println!("You wrote: {}", input);
    let age:i32 = &input.trim().parse().unwrap() * 365; 
    println!("Your age is {} days", age);

}