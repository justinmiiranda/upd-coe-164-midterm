pub struct RiceEncoder;

impl RiceEncoder {
    // maybe handle negative numbers
    pub fn encode(s: u32, m: u32) -> String {
        // Calculate K, U, and B
        let k: u32 = m.leading_zeros();
        let u = s >> k;
        let b = s & (m - 1);
        
        // Encode u in unary
        let mut unary_code = String::new();
        for _ in 0..u {
            unary_code.push('1');
        }
        unary_code.push('0');

        
        // Concatenate unary and binary codes
        let result = unary_code + &format!("{:0width$b}", b, width = k as usize);
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::RiceEncoder;
    
    #[test]
    fn test_encode() {
        // Test case 1
        let encoded_value = RiceEncoder::encode(18, 16);
        assert_eq!(encoded_value, "100010".to_string());

        // Add more test cases as needed
    }
}