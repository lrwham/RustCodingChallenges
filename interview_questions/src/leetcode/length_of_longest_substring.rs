// Given a string s, find the length of the longest
// substring without repeating characters.

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() { return 0 }
    if s.len() == 1 { return 1 }

    let mut left_index = 0;
    let mut window_width = 2;
    let mut right_index = left_index + window_width - 1;
    let mut longest_found = 1;

    while right_index < s.len(){
        if has_repeated_char(&s[left_index..=right_index]){
            left_index += 1;
            right_index = left_index + window_width - 1;
        } else {
            longest_found = window_width;
            window_width += 1;
            left_index = 0;
            right_index = left_index + window_width - 1;
        }
    }

    longest_found as i32

}

use std::collections::HashMap;
pub fn fastest_length_of_longest_substring(s: String) -> i32 {
    let mut length = 0;
    let mut left = 0;
    let mut chars = HashMap::new();

    for (right, c) in s.bytes().enumerate() {
        chars.insert(c, right + 1).map(|i| left = left.max(i));
        length = length.max(right - left + 1);
    }
    length as i32
}

// fn brute_length_of_longest_substring(s: String) -> i32 {
//     if s == String::from(""){
//         return 0
//     }
//     // record the length of the the substring at each index
//     // of the vector as it relates to each index/char of the string
//     let mut largest_found: usize = 1;

//     for left_index in 0..s.len(){
//         for string_size in largest_found..s.len()-left_index{
//             let substring: String = s[left_index..=left_index + string_size].to_string();

//             if !has_repeated_char(&substring) {
//                 largest_found = substring.len();
//             }
//         }
//     }

//     largest_found as i32
// }

fn has_repeated_char(s: &str) -> bool {
    use std::collections::HashSet;    
    
    let mut chars_used: HashSet<char> = HashSet::new();

    for c in s.chars(){
            if chars_used.insert(c) { continue; }
            else { return true; }
    }

    false
}