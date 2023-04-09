#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    // vec of strings!
    // This algorithm is in the 50th percentile for both time and memory
    // adding this comment apparently makes it worse on Leetcode!

    if num_rows == 1 { return s; }

    let mut descending = true;

    let num_rows: usize = num_rows as usize;

    let mut chars_iter = s.chars();

    let mut strings = vec![String::from(""); num_rows];

    let mut row = 0;

    while let Some(c) = chars_iter.next(){
        strings[row].push(c);

        if descending {
            if row < num_rows - 1 {
                row += 1;
            } else {
                row -= 1;
                descending = false;
            }
        } else {            
            if row > 0 {
                row -= 1;
            } else { 
                row += 1;
                descending = true;
            }
        }
    }

    let result: String = strings.join("");
    // println!("{:?}", result);

    result
}

#[allow(dead_code)]
pub fn convert_old(s: String, num_rows: i32) -> String {
    // try with a 2D array (1D array with 2D indexing)
    // when in row one, descend in same column to lowest row
    // after descending and hitting lowest row,
    // ascend one row, move one column right each char
    // waste memory and time, but passes!
    //
    // 10th percentile for time, 5th percentile for memory
    // yikes
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
    fn test_convert_old() {
        assert_eq!(convert_old("PAYPALISHIRING".to_string(),3), "PAHNAPLSIIGYIR");
        assert_eq!(convert_old("PAYPALISHIRING".to_string(),4), "PINALSIGYAHRPI");

    }
    #[test]
    fn test_convert() {
        assert_eq!(convert("PAYPALISHIRING".to_string(),3), "PAHNAPLSIIGYIR");
        assert_eq!(convert("PAYPALISHIRING".to_string(),4), "PINALSIGYAHRPI");

    }

}