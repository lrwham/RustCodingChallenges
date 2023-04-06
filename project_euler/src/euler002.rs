// Each new term in the Fibonacci sequence is generated by
// adding the previous two terms. By starting with 1 and 2,
// the first 10 terms will be:
//
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//
// By considering the terms in the Fibonacci sequence whose
// values do not exceed four million, find the sum of the even-valued terms.

pub mod euler002{

    pub fn solution() -> i32 {
        let limit = 4000000;

        let mut sum = 0;

        let mut last_fib = 0;
        let mut current_fib = 1;

        while current_fib < limit {
            if is_even(current_fib){
                sum += current_fib;
            }

            let temp = last_fib;
            last_fib = current_fib;
            current_fib += temp;
        }
        
        sum
    }

    fn is_even(number: i32) -> bool {
        number % 2 == 0
    }

    fn next_fibonacci(n_minus_one: i32,n_minus_two: i32) -> i32{
        n_minus_one + n_minus_two
    }

    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_even() {
            assert_eq!(true, is_even(2));
            assert_eq!(true, is_even(4));
            assert_eq!(true, is_even(10));
            assert_eq!(false, is_even(7));
            assert_eq!(false, is_even(13));
            assert_eq!(false, is_even(27));
        }

        #[test]
        fn test_next_fibonacci(){
            assert_eq!(2,next_fibonacci(1,1));
            assert_eq!(8,next_fibonacci(3,5));
        }

        #[test]
        fn test_solution(){
            assert_eq!(4613732,solution());
        }
    }
}