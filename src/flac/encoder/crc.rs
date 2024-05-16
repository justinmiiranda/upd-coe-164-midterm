/// Represents a kind of CRC encoding
/// 
/// This struct is used to configure the type of CRC encoding to use.
/// For example, if the generator polynomial for a CRC8 encoding is:
/// 
/// `x^8 + x^2 + x^1 + 1`
/// 
/// Then, the value of `poly` should be 0b0000_0111 (note the missing
/// MSB `1` bit) and `poly_len` should be `u8`.
pub struct CrcOptions <T> {
    poly: T,
    poly_len: T,
}


impl <T> CrcOptions <T> {
    /// Create a builder to the CRC encoder
    pub fn new(poly: T, poly_len: T) -> Self {
        Self {
            poly,
            poly_len
        }
    }
}

// http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html#ch41
impl CrcOptions <u8> {
    /// Encode data using CRC8 encoding
    /// 
    /// This method is available only if `CrcOptions` is of type `u8`.
    pub fn build_crc8(&self, data: &Vec <u8>) -> u8 {
        let mut crc: u8 = 0;

        for &curr_byte in data {
            crc ^= curr_byte;

            for _ in 0..self.poly_len {
                if (crc & 0x80) != 0 {
                    crc = (crc << 1) ^ self.poly;
                } else {
                    crc <<= 1;
                }
            }
        }

        crc & ((1 << (self.poly_len as usize)) - 1) as u8 // Mask CRC to poly_len bits
    }
}

impl CrcOptions <u16> {
    /// Encode data using CRC16 encoding
    /// 
    /// This method is available only if `CrcOptions` is of type `u16`.
    pub fn build_crc16(&self, data: &Vec <u16>) -> u16 {
        let mut crc: u16 = 0;

        for &curr_byte in data {
            crc ^= curr_byte;

            for _ in 0..self.poly_len {
                if (crc & 0x80) != 0 {
                    crc = (crc << 1) ^ self.poly;
                } else {
                    crc <<= 1;
                }
            }
        }

        crc & ((1 << (self.poly_len as usize)) - 1) as u16 // Mask CRC to poly_len bits
    }
}