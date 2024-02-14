// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143?

pub mod euler003 {
    use primes::primes::prime_vec_sieve_erastothenes;

    pub fn solution() -> usize {
        let test_num: usize = 600851475143;
        let prime_limit = 10000;

        let primes = prime_vec_sieve_erastothenes(prime_limit);

        let mut max = 0;
        for i in primes.iter() {
            if test_num % *i == 0 {
                max = *i;
            }
        }

        max
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sieve_of_erastothenes() {
            assert_eq!(
                vec![2, 3, 5, 7, 11, 13, 17],
                prime_vec_sieve_erastothenes(18)
            );
            assert_eq!(
                vec![2, 3, 5, 7, 11, 13, 17, 19],
                prime_vec_sieve_erastothenes(22)
            );
        }

        #[test]
        fn test_solution() {
            assert_eq!(6857, solution());
        }
    }
}
