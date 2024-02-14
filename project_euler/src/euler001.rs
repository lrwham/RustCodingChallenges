// If we list all the natural numbers below 10 that are
// multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of
// these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

pub mod euler001 {
    pub fn solution() -> i32 {
        let mut sum = 0;
        for num in 1..1000 {
            if is_divisible(num, 3) || is_divisible(num, 5) {
                sum += num;
            }
        }
        sum
    }
    fn is_divisible(numerator: i32, divisor: i32) -> bool {
        numerator % divisor == 0
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_solution() {
            assert_eq!(solution(), 233168);
        }

        #[test]
        fn test_is_divisible() {
            assert_eq!(true, is_divisible(6, 2));
            assert_eq!(true, is_divisible(9, 3));
            assert_eq!(true, is_divisible(12, 6));
            assert_eq!(false, is_divisible(6, 4));
            assert_eq!(false, is_divisible(6, 5));
            assert_eq!(false, is_divisible(7, 12));
        }
    }
}
