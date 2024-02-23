fn max_difference_expo(nums: &Vec<i32>) -> i32 {
    let mut max_diff = i32::MIN;
    for i in 0..nums.len() - 1 {
        for j in i+1..nums.len() {
            let diff = nums[j] - nums[i];
            if diff > max_diff {
                max_diff = diff;
            }
        }
    }

    max_diff
}

fn main() {
    let nums = vec!{1,2,0,-9,3};
    let expected = 12;
    let calculated = max_difference_expo(&nums);
    println!("nums: {:?}, expected: {}, calc: {}", nums,expected,calculated);
}


