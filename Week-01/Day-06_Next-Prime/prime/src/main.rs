use std::io::{stdin};





fn main() {
    //Get user input as u64
    let mut user_input = String::new();
    println!("Enter a number: ");
    while user_input.is_empty(){
        stdin().read_line(&mut user_input).unwrap();
    }
    let user_input:u64 = user_input.trim().parse().unwrap();

    //Check if user input is prime
    if is_prime(user_input){
        println!("{} is prime", user_input);
    }else{
        println!("{} is not prime", user_input);
    }

    //Find the next prime number
    let next_prime = find_next_prime(user_input);
    println!("The next prime number after {} is {}", user_input, next_prime);


}

fn is_prime(num:u64) -> bool {
    if num <= 1{
        return false;
    }
    for i in 2..num{
        if num % i == 0{
            return false;
        }
    }
    true
}

fn find_next_prime(num:u64) -> u64 {
    let mut next_prime = num + 1;
    while !is_prime(next_prime){
        next_prime += 1;
    }
    next_prime
}