struct Solution;

impl Solution {
    // two pointer solution suggested by chatGPT
    #[allow(dead_code)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut result: Vec<Vec<i32>> = vec![];

        for i in 0..nums.len(){
            // skip duplicates
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let total = nums[i] + nums[left] + nums[right];

                if total == 0 {
                    let vec = vec![nums[i], nums[left], nums[right]];
                    // as explained by ChatGPT this &mut vec![vec] mess is meant to allow
                    // the result to be modified in place
                    result.append(&mut vec![vec]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1]
                    {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
                else if total < 0 {
                    left += 1;
                }
                else {
                    right -= 1
                }
            }

        }

        result
    }

    pub fn three_sum_too_slow(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // construct all possible combinations of 2 numbers
        let pairs = Self::all_pairs(&nums);
        let mut result: Vec<Vec<i32>> = vec![];
        for pair in pairs {
            let sum = pair.0 + pair.1;
            if (nums.contains(&-sum) && pair.0 != -sum && pair.1 != -sum) || (sum == 0 && nums.contains(&0)){
                let mut triplet = vec![pair.0, pair.1, -sum];
                triplet.sort();

                let mut is_valid = true;
                for i in triplet.iter() {
                    if triplet.iter().filter(|&x| x == i).count() > nums.iter().filter(|&x| x == i).count(){
                        is_valid = false;
                    }
                }

                if is_valid && !result.contains(&triplet) {
                    result.push(triplet);
                }
            }
        }
        result
    }

    // rather than test all triplets, we can test all pairs and then check if the negative sum of the pair is in the list
    pub fn all_pairs(nums: &Vec<i32>) -> Vec<(i32, i32)> {
        let mut pairs: Vec<(i32, i32)> = vec![];
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let a = nums[i];
                let b = nums[j];

                let pair = if a < b {
                    (a, b)
                } else {
                    (b, a)
                };

                if !pairs.contains(&pair) {
                    pairs.push((nums[i], nums[j]));
                }
            }
        }
        pairs
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    #[allow(unused_imports)]
    use super::*;

    pub fn hashset_from_vec(expected: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        expected.into_iter().map(|mut x| {
            x.sort();
            x
        }).collect::<HashSet<Vec<i32>>>().into_iter().collect()
    }
    #[test]
    fn test_leetcode_examples(){
        // Input: nums = [-1,0,1,2,-1,-4]
        // Output: [[-1,-1,2],[-1,0,1]]
        let input = vec![-1,0,1,2,-1,-4];
        let expected = vec![vec![-1,-1,2], vec![-1,0,1]];

        assert_eq!(hashset_from_vec(expected), hashset_from_vec(Solution::three_sum_too_slow(input)));

        // Input: nums = [0,1,1]
        // Output: []
        let input = vec![0,1,1];
        let expected = vec![];

        assert_eq!(hashset_from_vec(expected), hashset_from_vec(Solution::three_sum_too_slow(input)));

        // Input: nums = [0,0,0]
        // Output: [[0,0,0]]
        let input = vec![0,0,0];
        let expected = vec![vec![0,0,0]];

        assert_eq!(hashset_from_vec(expected), hashset_from_vec(Solution::three_sum_too_slow(input)));

        // Input: nums = [-1,0,1,0]
        // Expected: [[-1,0,1]]
        let input = vec![-1,0,1,0];
        let expected = vec![vec![-1,0,1]];

        assert_eq!(hashset_from_vec(expected), hashset_from_vec(Solution::three_sum_too_slow(input)));
    }
}