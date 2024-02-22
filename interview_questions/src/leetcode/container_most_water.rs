struct Solution;

impl Solution{
    pub fn max_area(height: Vec<i32>) -> i32 {

        let mut max_area: i32 = 0;
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;

        while left < right {

            let w = (right - left) as i32;
            let h = std::cmp::min(height[left], height[right]);
            let area = w * h;

            max_area = std::cmp::max(max_area, area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }

    // fast enough for leetcode
    pub fn max_area_naive(height: Vec<i32>) -> i32 {
        let mut max_area: i32 = 0;

        let mut left_max: i32 = 0;

        for i in 0..height.len() {
            // only consider the height of left (i) if it is at least as high as the previous left_max
            // this foreshadows the two pointers solution which is much faster
            if height[i] < left_max {
                continue;
            }else{
                left_max = height[i];
            }
            for j in i..height.len() {
                let area = (j - i) as i32 * std::cmp::min(height[i], height[j]);
                max_area = std::cmp::max(max_area, area);
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_naive_leetcode_provided() {
        // Input: height = [1,8,6,2,5,4,8,3,7]
        // Output: 49
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;

        assert_eq!(expected, Solution::max_area_naive(input));

        // Input: height = [1,1]
        // Output: 1
        let input = vec![1, 1];
        let expected = 1;

        assert_eq!(expected, Solution::max_area_naive(input));
    }

    #[test]
    fn test_twopointers_leetcode_provided() {
        // Input: height = [1,8,6,2,5,4,8,3,7]
        // Output: 49
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;

        assert_eq!(expected, Solution::max_area(input));

        // Input: height = [1,1]
        // Output: 1
        let input = vec![1, 1];
        let expected = 1;

        assert_eq!(expected, Solution::max_area(input));
    }
}