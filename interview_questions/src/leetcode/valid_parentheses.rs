struct Solution;

impl Solution{
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            } else {
                match stack.last() {
                    Some(&last) => {
                        if last == '(' && c == ')' || last == '[' && c == ']' || last == '{' && c == '}' {
                            stack.pop();
                        } else {
                            stack.push(c);
                        }
                    }
                    None => {
                        stack.push(c);
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_trivial() {
        assert_eq!(true, Solution::is_valid(String::from("()")));
        assert_eq!(true, Solution::is_valid(String::from("()[]{}")));
        assert_eq!(false, Solution::is_valid(String::from("(])")));
        assert_eq!(false, Solution::is_valid(String::from("(")));
        assert_eq!(true, Solution::is_valid(String::from("")));
    }

    #[test]
    fn test_odd_length() {
        assert_eq!(false, Solution::is_valid(String::from("()(")));
    }

    #[test]
    fn test_long() {
        assert_eq!(true, Solution::is_valid(String::from("(()){[()]}")));
    }
}
