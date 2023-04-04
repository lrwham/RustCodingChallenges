// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

pub mod euler004 {
    pub fn solution() -> usize {
        let mut max: usize = 0;

        for i in 100..=999{
            for j in 100..=999{
                let test = i * j;

                if test > max && is_palindrome(test){
                    max = test;
                }
            }
        }

        max
    }

    fn is_palindrome(number: usize) -> bool{
        let word_vec: Vec<char> = number.to_string().chars().collect();


        for (index, value) in word_vec.iter().enumerate(){
            if *value == word_vec[word_vec.len() - 1 - index]{
                continue;
            } else {
                return false
            }
        }

        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_palindrome(){
            assert!(is_palindrome(121));
            assert!(is_palindrome(12321));
            assert!(is_palindrome(1221221));
            assert!(!is_palindrome(123));
            assert!(!is_palindrome(12345321));

        }

        #[test]
        fn test_solution(){
            assert_eq!(906609,solution());
        }
    }
}