pub struct Utf8Encoder;

impl Utf8Encoder {
    /// Encode a number into its UTF-9 equivalent encoding
    /// 
    /// Although UTF-8 encoding is for characters, characters are
    /// mapped to certain numbers.
    pub fn encode(num: u64) -> Vec<u8> {
        let min_num_bits = Self::ceiling_log(num);
        let num_bin_string = String::from(format!("{min_num_bits:b}"));

        let template = Self::template_generator(min_num_bits);
        let unicode_binary = Self::replace_x(template, num_bin_string);
        
        return Self::binary_string_to_u8_vector(&unicode_binary.to_string());
    }

    // Helper Functions
    fn template_generator(min: u64) -> String {
        if min > 0 && min <= 7 {
            "0xxxxxx".to_string()
        } else if min >= 8 && min <= 11 {
            "110xxxxx10xxxxxx".to_string()
        } else if min >= 12 && min <= 16 {
            "1110xxxx10xxxxxx10xxxxxx".to_string()
        } else if min >= 17 && min <= 21 {
            "11110xxx10xxxxxx10xxxxxx10xxxxxx".to_string()
        } else if min >= 22 && min <= 26 {
            "111110xx10xxxxxx10xxxxxx10xxxxxx10xxxxxx".to_string()
        } else if min >= 27 && min <= 31 {
            "1111110x10xxxxxx10xxxxxx10xxxxxx10xxxxxx10xxxxxx".to_string()
        } else if min >= 32 && min <= 40 {
            "1111111010xxxxxx10xxxxxx10xxxxxx10xxxxxx10xxxxxx10xxxxxx".to_string()
        } else {
            String::new()
        }
    }

    fn ceiling_log(num: u64) -> u64 {
        return (num as f64).log2().ceil() as u64;
    }

    fn replace_x(template_string: String, binary_string: String) -> String {
        let mut result = String::with_capacity(template_string.len());
        let mut index_binary_string = binary_string.len();
    
        for c in template_string.chars().rev() {
            if c == 'x' {
                if index_binary_string != 0 {
                    result.push(binary_string.chars().nth(index_binary_string - 1).unwrap());
                    index_binary_string -= 1;
                } else {
                    result.push('0');
                }
            } else {
                result.push(c);
            }
        }
    
        result.chars().rev().collect()
    }
    
    fn binary_string_to_u8_vector(binary_string: &str) -> Vec<u8> {
        let mut result = Vec::new();
        let mut current_byte = 0;
        let mut bits_in_current_byte = 0;
    
        for c in binary_string.chars() {
            if let Some(bit) = c.to_digit(2) {
                current_byte = (current_byte << 1) | bit;
                bits_in_current_byte += 1;
    
                if bits_in_current_byte == 4 {
                    result.push(current_byte as u8);
                    current_byte = 0;
                    bits_in_current_byte = 0;
                }
            } else {
                continue;
            }
        }

        if bits_in_current_byte > 0 {
            result.push(current_byte as u8);
        }
    
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = 2766;
        let expected_output = vec![14, 0, 10, 11, 8, 14];
        assert_eq!(Utf8Encoder::encode(input), expected_output);
    }
}