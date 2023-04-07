#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    
    s
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