// Given an unsorted array A of size N that contains
// only positive integers, find a continuous sub-array
// that adds to a given number S and return the left and right
// index of that subarray.

// In case of multiple subarrays, return the subarray indexes
// which come first on moving from left to right.
pub mod sub_array_sum{
    use crate::helpers::random_array_intergers;
    pub struct array_indexes{
        left_index: usize,
        right_index: usize
    }

    pub fn solution(){
        let test_arr: Vec<usize> = random_array_intergers(5).to_vec();
        let test_sum: usize = 9;

        match sub_array_sum(&test_arr, test_sum){
            None => println!("None found!"),
            Some(x) => {
                println!("{}, {}", x.left_index, x.right_index);
                println!("{:?}",&test_arr);
                println!("{:?}",&test_arr[x.left_index..x.right_index + 1]);
            }
            
        }

    }

    pub fn sub_array_sum(arr: &Vec<usize>, sum_to_find: usize) -> Option<array_indexes>{
        for l_index in 0..arr.len(){
            if arr[l_index] == sum_to_find{
                return Some(array_indexes{left_index:l_index,right_index:l_index});
            }

            let mut sum = arr[l_index];

            for r_index in l_index + 1 .. arr.len(){
                sum += arr[r_index];

                if sum > sum_to_find { break; }
                if sum == sum_to_find { return Some(array_indexes { left_index: l_index, right_index: r_index })}

            }

        }

        None
    }
}