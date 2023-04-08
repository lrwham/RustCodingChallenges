use core::num;

#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    // try with a 2D array (1D array with 2D indexing)
    // when in row one, descend in same column to lowest row
    // after descending and hitting lowest row,
    // ascend one row, move one column right each char
    // waste memory and time, but passes!
    if num_rows == 1 { return s; }

    let mut descending = true;

    let num_rows: usize = num_rows as usize;

    let mut chars_iter = s.chars();

    let width = s.len();

    let mut arr = vec!['\0'; num_rows * width];

    let mut column = 0;
    let mut row = 0;

    while let Some(c) = chars_iter.next(){
        arr[column + row*width] = c;

        if descending {
            if row < num_rows - 1 {
                row += 1;
            } else {
                row -= 1;
                column += 1;
                descending = false;
            }
        } else {            
            if row > 0 {
                row -= 1;
                column += 1;
            } else { 
                row += 1;
                descending = true;
            }
        }
    }

    arr.retain(|&c| c != '\0');

    let result: String = arr.into_iter().collect();
    println!("{:?}", result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert() {
        assert_eq!(convert("PAYPALISHIRING".to_string(),3), "PAHNAPLSIIGYIR");
        assert_eq!(convert("PAYPALISHIRING".to_string(),4), "PINALSIGYAHRPI");

    }
}