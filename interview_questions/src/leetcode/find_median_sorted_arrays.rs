pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let merged = merge(nums1,nums2);

    if merged.len() % 2 == 0{
        let left_center_idx = (merged.len() / 2) - 1;
        let right_center_idx = merged.len() / 2;

        (merged[left_center_idx] + merged[right_center_idx]) as f64 / 2.0
    }else{
        merged[merged.len()/2] as f64
    }
}


fn merge(left_vec: Vec<i32>, right_vec: Vec<i32>) -> Vec<i32> {

    if left_vec.is_empty() { return right_vec; }
    if right_vec.is_empty() { return left_vec; }

    let (mut left_idx, mut right_idx) = (0, 0);
    let mut sorted = vec![];
    let remaining;
    let remaining_idx;

    loop {
        if left_vec[left_idx] < right_vec[right_idx] {
            sorted.push(left_vec[left_idx]);
            left_idx += 1;
            if left_idx == left_vec.len() {remaining = right_vec; remaining_idx = right_idx; break;}
        } else {
            sorted.push(right_vec[right_idx]);
            right_idx += 1;
            if right_idx == right_vec.len() {remaining = left_vec; remaining_idx = left_idx; break;}
        }
    }
    for item in remaining.iter().skip(remaining_idx) {
    // for i in remaining_idx..remaining.len() {
        sorted.push(*item);
    }

    sorted
}
