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
        todo!()
    }
}

impl CrcOptions <u8> {
    /// Encode data using CRC8 encoding
    /// 
    /// This method is available only if `CrcOptions` is of type `u8`.
    pub fn build_crc8(&self, data: &Vec <u8>) -> u8 {
        todo!()
    }
}

impl CrcOptions <u16> {
    /// Encode data using CRC16 encoding
    /// 
    /// This method is available only if `CrcOptions` is of type `u16`.
    pub fn build_crc16(&self, data: &Vec <u16>) -> u16 {
        todo!()
    }
}