use trap::user_input::get_input;
use trap::visualize::visualize;
use trap::calc::calc;

fn main() {
    let input_vector = get_input();
    println!("Input: {:?}", input_vector);
    let trapped_water = calc(&input_vector);
    println!("{} fields of water gets trapped!", trapped_water);
    visualize(&input_vector);
}
