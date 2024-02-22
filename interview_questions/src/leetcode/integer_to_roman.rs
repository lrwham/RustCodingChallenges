// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
//
// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut result = String::new();
        let mut symbols = vec!['M', 'D', 'C', 'L', 'X', 'V', 'I'];
        let mut values = vec![1000, 500, 100, 50, 10, 5, 1];
        while num > 0 {
            if num >= values[0] {
                result.push(symbols[0]);
                num -= values[0];
            } else if values.len() > 2 && num >= values[0] - values[2] && values[2] != 0 {
                result.push(symbols[2]);
                result.push(symbols[0]);
                num -= values[0] - values[2];
            } else {
                values.remove(0);
                symbols.remove(0);
            }
        }
        result = result.replace("IIII","IV");
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial() {
        assert_eq!("I", Solution::int_to_roman(1));
        assert_eq!("III", Solution::int_to_roman(3));
        assert_eq!("V", Solution::int_to_roman(5));
        assert_eq!("VIII", Solution::int_to_roman(8));
        assert_eq!("X", Solution::int_to_roman(10));
        assert_eq!("XIII", Solution::int_to_roman(13));
        assert_eq!("XV", Solution::int_to_roman(15));
        assert_eq!("XVIII", Solution::int_to_roman(18));
        assert_eq!("XX", Solution::int_to_roman(20));
        assert_eq!("XXIII", Solution::int_to_roman(23));
        assert_eq!("XXV", Solution::int_to_roman(25));
        assert_eq!("XXVIII", Solution::int_to_roman(28));
        assert_eq!("XXX", Solution::int_to_roman(30));
        assert_eq!("XXXIII", Solution::int_to_roman(33));
        assert_eq!("XXXV", Solution::int_to_roman(35));
        assert_eq!("XXXVIII", Solution::int_to_roman(38));
        assert_eq!("L", Solution::int_to_roman(50));
        assert_eq!("LIII", Solution::int_to_roman(53));
        assert_eq!("LV", Solution::int_to_roman(55));
        assert_eq!("LVIII", Solution::int_to_roman(58));
        assert_eq!("LX", Solution::int_to_roman(60));
        assert_eq!("LXIII", Solution::int_to_roman(63));
        assert_eq!("LXV", Solution::int_to_roman(65));
        assert_eq!("LXVIII", Solution::int_to_roman(68));
        assert_eq!("LXX", Solution::int_to_roman(70));
        assert_eq!("LXXIII", Solution::int_to_roman(73));
        assert_eq!("LXXV", Solution::int_to_roman(75));
        assert_eq!("C", Solution::int_to_roman(100));
        assert_eq!("D", Solution::int_to_roman(500));
        assert_eq!("M", Solution::int_to_roman(1000));
    }

    #[test]
    fn test_leetcode_examples(){
        assert_eq!("III", Solution::int_to_roman(3));
        assert_eq!("IV", Solution::int_to_roman(4));
        assert_eq!("IX", Solution::int_to_roman(9));
        // 1994
        assert_eq!("MCMXCIV", Solution::int_to_roman(1994));
    }
}