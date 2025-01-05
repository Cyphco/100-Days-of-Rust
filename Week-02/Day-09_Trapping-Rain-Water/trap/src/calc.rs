


pub fn calc(input:&Vec<i32>) -> i32{
    if input.is_empty() {
        return 0;
    }
    let mut left = 0;
    let mut right = input.len() - 1;
    let mut max_left = 0;
    let mut max_right = 0;
    let mut result = 0;

    while left < right {
        let left_height = input[left];
        let right_height = input[right];

        if left_height < right_height {
            if left_height >= max_left {
                max_left = left_height;
            } else {
                result += max_left - left_height;
            }
            left += 1;
        } else {
            if right_height >= max_right {
                max_right = right_height;
            } else {
                result += max_right - right_height;
            }
            right -= 1;
        }
    }

    result
}