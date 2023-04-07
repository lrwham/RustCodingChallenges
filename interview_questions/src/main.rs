mod first_occurence_k_times;
mod sub_array_sum;
mod helpers;
mod two_sum;
mod leetcode;



fn main() {
    println!("Hello, world!");
    let a = vec![1,2,3,3,4,7,8,9];
    let b = vec![];
    println!("{}", crate::leetcode::find_median_sorted_arrays(a,b));
}
