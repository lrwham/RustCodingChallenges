#![feature(test)]

extern crate test;

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

fn max_difference_remember_min(nums: &Vec<i32>) -> i32 {
    let mut max_diff = i32::MIN;

    let mut min = nums[0];

    for i in 1..nums.len() {
        let diff = nums[i] - min;
        if diff > max_diff {
            max_diff = diff;
        }
        if nums[i] < min {
            min = nums[i];
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_max_difference_expo() {
        let nums = vec!{1,2,0,-9,3};
        let expected = 12;
        let calculated = max_difference_expo(&nums);
        assert_eq!(expected, calculated);

        let nums = vec!{1,2,0,-9,3,4};
        let expected = 13;
        let calculated = max_difference_expo(&nums);
        assert_eq!(expected, calculated);

        let nums = vec!{0,1,2,3,4,5};
        let expected = 5;
        let calculated = max_difference_expo(&nums);
        assert_eq!(expected, calculated);

        let nums = vec!{5,4,3,2,1,0};
        let expected = -1;
        let calculated = max_difference_expo(&nums);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_max_difference_remember_min() {
        let nums = vec!{1,20,7,3,5,6,8,9};
        let expected = 19;
        let calculated = max_difference_remember_min(&nums);
        assert_eq!(expected, calculated);

        let nums = vec!{1,2,0,-9,3};
        let expected = 12;
        let calculated = max_difference_remember_min(&nums);
        assert_eq!(expected, calculated);

        let nums = vec!{1,2,0,-9,3,4};
        let expected = 13;
        let calculated = max_difference_remember_min(&nums);
        assert_eq!(expected, calculated);

        let nums = vec!{0,1,2,3,4,5};
        let expected = 5;
        let calculated = max_difference_remember_min(&nums);
        assert_eq!(expected, calculated);

        let nums = vec!{5,4,3,2,1,0};
        let expected = -1;
        let calculated = max_difference_remember_min(&nums);
        assert_eq!(expected, calculated);
    }

    #[bench]
    fn bench_expo(b: &mut Bencher) {
        let nums = vec! {1, 2, 0, -9, 3, 99, 65, 21, 32, 54, 65, 0};
        b.iter(|| max_difference_expo(&nums));

        let nums = vec!{11,2,0,-9,65,6532,45,8,52,13};
        b.iter(|| max_difference_expo(&nums));

        // generate a vector of 1000 elements with random values
        let nums: Vec<i32> = (0..1000).map(|_| rand::random::<i32>()).collect();
        b.iter(|| max_difference_expo(&nums));
    }

    #[bench]
    fn bench_remember_min(b: &mut Bencher) {
        let nums = vec! {1, 2, 0, -9, 3, 99, 65, 21, 32, 54, 65, 0};
        b.iter(|| max_difference_remember_min(&nums));

        let nums = vec!{11,2,0,-9,65,6532,45,8,52,13};
        b.iter(|| max_difference_remember_min(&nums));

        // generate a vector of 1000 elements with random values
        let nums: Vec<i32> = (0..1000).map(|_| rand::random::<i32>()).collect();

        b.iter(|| max_difference_remember_min(&nums));

    }
}


