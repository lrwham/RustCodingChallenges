
pub fn random_array_intergers(mod_value: usize) -> [usize; 32] {
    let mut arr1: [usize; 32] = rand::random();

    for element in &mut arr1 {
        *element %= mod_value;
    }

    arr1
}
