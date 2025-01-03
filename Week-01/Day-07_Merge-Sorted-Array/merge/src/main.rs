










fn main() {
    let mut nums1: Vec<i32> = vec![1,2,3];
    let nums2: Vec<i32> = vec![2,5,6];
    let mut index: usize = 0;

    for num in nums2 {
        while index < nums1.len() && nums1[index] < num {
            index += 1;
        }
        nums1.insert(index, num);
        index += 1;  // Move index past the newly inserted element
    }

    for i in nums1 {
        print!("{},", i);
    }
}