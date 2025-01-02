fn main() {
    let stats: Vec<i32> = vec![
    10, 11, 12, 9, 10
    ];

    let progress = check_progress(&stats);
    println!("Progress: {}", progress);
}

fn check_progress(stats: &Vec<i32>) -> i32 {
    let mut current = 0;
    let mut progress = 0;

    while current < stats.len() - 1 {
        if stats[current] < stats[current + 1] {
            progress += 1;
        }
        current += 1;
    }
    progress
}