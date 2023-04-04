// 2520 is the smallest number that can be divided by each of the
// numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly
// divisible by all of the numbers from 1 to 20?

pub mod euler005{
    use std::ops::RangeInclusive;

    pub fn alt_solution() -> usize{
        let mut numbers: Vec<usize> = (9999..333333333).collect();

        for i in 11..=20 {
            for index in 0..numbers.len(){
                if numbers[index] % i != 0{
                    numbers[index] = 0;
                }
            }

            numbers = numbers.into_iter().filter(|&x| x != 0).collect();
        }

        let result: Vec<usize> = numbers.into_iter().filter(|&x| x != 0).collect();

        result[0]
    }

    pub fn solution() -> usize{
        // note 
        let limit: usize = (1..=20).product();

        let mut i: usize = 1000;

        loop {
            if is_divisible_by_range(i, 11..=20){
                return i;
            } else if i > limit {
                panic!();
            }

            i += 20;
        }
    }

    fn is_divisible_by_range(test_num: usize, range_divisors: RangeInclusive<usize>) -> bool{
        for i in range_divisors{
            if test_num % i != 0{
                return false;
            }
        }

        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solution(){
            assert_eq!(232792560, solution());
        }

    }

}