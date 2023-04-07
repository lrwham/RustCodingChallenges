#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    let mut size = s.len();
    let mut left = 0;
    let mut right = s.len();

    loop {
        let sub_string = &s[left..right];

        if is_palindrome(sub_string) { return sub_string.to_string(); }

        if right < s.len() {
            left += 1;
            right += 1;
        } else {
            size -= 1;
            left = 0;
            right = size;
        }

        if size == 1 { return s[..1].to_string() }
    }
}

pub fn is_palindrome(s: &str) -> bool {
    let mut chars = s.chars();
    while let (Some(left), Some(right)) = (chars.next(), chars.next_back()) {
        if left != right {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("s"));
        assert!(is_palindrome("aa"));
        assert!(is_palindrome("aba"));
        assert!(is_palindrome("abaaba"));


        assert!(!is_palindrome("ab"));
        assert!(!is_palindrome("abc"));
        assert!(!is_palindrome("abcddcbaa"));

    }

    #[test]
    fn test_longest_palindrome(){
        let s = String::from("abcba");
        assert_eq!(s, longest_palindrome(s.clone()));

        let s = String::from("abcbaaaaaa");
        assert_eq!("aaaaaa", longest_palindrome(s.clone()));

        let s = String::from("abcbaaaaaaab");
        assert_eq!("baaaaaaab", longest_palindrome(s.clone()));
    }
}