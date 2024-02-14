// Given an array of N integers. Find the first element that occurs at least K number of times.

use crate::helpers::random_array_intergers;

use std::collections::HashMap;

#[allow(dead_code)]
pub fn solution() {
    let z = random_array_intergers(5);

    for count in 1..=5 {
        match first_occurrence(count, &z) {
            Some(x) => println!("{} was the first key to occur {} times.", x, count),
            None => println!("No key occured {} time/s", count),
        }
    }
}

pub fn first_occurrence(count_to_look_for: usize, slice: &[usize]) -> Option<usize> {
    let mut counts_map: HashMap<usize, usize> = HashMap::new();

    for value in slice {
        let count = counts_map.entry(*value).or_insert(0);
        *count += 1;

        if *count >= count_to_look_for {
            return Some(*value);
        }
    }

    None
}

#[cfg(test)]
mod tests {
}
