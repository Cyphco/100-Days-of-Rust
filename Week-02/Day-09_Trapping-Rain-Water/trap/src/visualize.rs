///Commandline visualization of the input vector 
pub fn visualize(input_vector: &Vec<i32>) {
    if input_vector.is_empty() {
        println!("Input vector is empty. No visualization available.");
        return;
    }
    let height: i32 = input_vector.iter().max().copied().unwrap();
    let length: i32 = input_vector.len() as i32;
    let current_line: i32 = height + 1;

    // Calculate water trapped at each position
    let mut water_trapped = vec![0; input_vector.len()];
    let mut left_max = 0;
    let mut right_max = 0;
    let mut left = 0;
    let mut right = input_vector.len() - 1;

    while left < right {
        if input_vector[left] < input_vector[right] {
            if input_vector[left] >= left_max {
                left_max = input_vector[left];
            } else {
                water_trapped[left] = left_max - input_vector[left];
            }
            left += 1;
        } else {
            if input_vector[right] >= right_max {
                right_max = input_vector[right];
            } else {
                water_trapped[right] = right_max - input_vector[right];
            }
            right -= 1;
        }
    }

    line_handler(current_line, &input_vector, &water_trapped, length);

    // Print the input vector at the bottom
    println!(" {:?}", input_vector);
}

fn line_handler(current_line: i32, input_vector: &[i32], water_trapped: &[i32], length: i32) {
    let mut current_line = current_line;
    while current_line > 0 {
        match current_line {
            1 => bottom_line(&length),
            _ => draw_line(current_line, input_vector.to_vec(), water_trapped.to_vec()),
        }
        current_line -= 1;
    }
}

fn draw_line(current_line: i32, input_vector: Vec<i32>, water_trapped: Vec<i32>) {
    let mut linestr = String::new();
    for (i, &value) in input_vector.iter().enumerate() {
        if value < current_line - 1 {
            if water_trapped[i] > 0 && current_line - 1 <= value + water_trapped[i] {
                linestr.push('💧');
            } else {
                linestr.push('　');
            }
        } else {
            linestr.push('⬛');
        }
    }
    println!("|{}", linestr);
}

fn bottom_line(length: &i32) {
    let length: i32 = *length;
    let mut linestr = String::new();
    for _ in 0..length {
        linestr.push('▔');
        linestr.push('▔');
    }
    println!(" {}", linestr);
}
