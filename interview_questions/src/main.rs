mod helpers;
mod leetcode;
mod other;



fn main() {
    println!("Hello, world!");
    crate::leetcode::zigzag_conversion::convert_old("hellohello".to_string(), 3);
}
