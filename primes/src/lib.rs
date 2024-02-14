/// Calculates the nth prime by iterating over all prime numbers until the solution.
///
/// # Arguments
///
/// * 'n' - An integer representing the ordinal position of the prime number.
///
/// # Examples
///
/// ```
/// use primes::primes::nth_prime;
/// let result = nth_prime(4);
/// assert_eq!(result, 7);
/// ```
///
pub mod primes {
    pub fn nth_prime(n: u64) -> u64 {
        if n == 1 {
            return 2;
        };
        if n == 2 {
            return 3;
        };

        let mut count = 2;
        let mut test = 3;

        while count < n {
            test += 2;
            if is_prime(test) {
                count += 1;
            }
        }

        test
    }

    pub fn is_prime(n: u64) -> bool {
        if n == 2 || n == 3 {
            return true;
        };

        let mut divisor = 2;

        if n % divisor == 0 {
            return false;
        };

        divisor += 1;

        let limit: u64 = ((n as f64).sqrt()) as u64 + 1;

        while divisor < limit {
            if n % divisor == 0 {
                return false;
            };
            divisor += 2;
        }

        true
    }

    pub fn prime_vec_sieve_erastothenes(n: usize) -> Vec<usize> {
        let mut primes: Vec<usize> = (0..=n).collect();

        primes[0] = 0;
        primes[1] = 0;

        let mut index = 2;

        // check up to sqrt(n)
        while index * index <= n {
            if primes[index] != 0 {
                let mut multiple = index + index;
                while multiple <= n {
                    primes[multiple] = 0;
                    multiple += index;
                }
            }
            index += 1;
        }

        primes.into_iter().filter(|&x| x != 0).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_nth_prime() {
            assert_eq!(nth_prime(1), 2);
            assert_eq!(nth_prime(2), 3);
            assert_eq!(nth_prime(3), 5);
            assert_eq!(nth_prime(4), 7);
            assert_eq!(nth_prime(5), 11);
            assert_eq!(nth_prime(347), 2341);
        }

        #[test]
        fn is_prime_with_primes() {
            assert_eq!(is_prime(2), true);
            assert_eq!(is_prime(3), true);
            assert_eq!(is_prime(5), true);
            assert_eq!(is_prime(7), true);
            assert_eq!(is_prime(11), true);
        }

        #[test]
        fn is_prime_with_non_primes() {
            assert_eq!(is_prime(4), false);
            assert_eq!(is_prime(6), false);
            assert_eq!(is_prime(8), false);
            assert_eq!(is_prime(9), false);
            assert_eq!(is_prime(12), false);
        }
    }
}
