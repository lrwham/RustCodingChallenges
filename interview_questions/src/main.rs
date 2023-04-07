mod helpers;
mod leetcode;
mod other;



fn main() {
    println!("Hello, world!");
    let a = vec![1,2,3,3,4,7,8,9];
    let b = vec![];
    println!("{}", crate::leetcode::find_median_sorted_arrays(a,b));
}
