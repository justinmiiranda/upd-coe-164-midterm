pub struct VarPredictor;

impl VarPredictor {
    /// Get the autocorrelation of a vector of data
    ///
    /// The function computes the first `lag`+1 autocorrelations of the
    /// provided vector of data. 
    pub fn get_autocorrelation(data: &Vec<i32>, lag: u32) -> Vec<f64> {
        // Initialize autocorrelation vector
        let mut autoc = vec![0.0; lag as usize + 1];

        // Iterate over each lag
        for l in 0..=lag {
            let mut d = 0.0;
            // Compute autocorrelation for current lag
            for i in l..data.len() as u32 {
                d += data[i as usize] as f64 * data[(i - l) as usize] as f64;
            }
            // Store autocorrelation for current lag
            autoc[l as usize] = d;
        }

        autoc
    }

    /// Get the predictor coefficients
    /// 
    /// The coefficients are computed using the Levinson-Durbin algorithm.
    pub fn get_predictor_coeffs(autoc: &Vec<f64>, predictor_order: u32) -> Vec<f64> {
        let mut lpc: Vec<f64> = vec![0.0; predictor_order as usize];
        let mut err = autoc[0];
    
        for i in 0..predictor_order {
            let mut r = -autoc[(i + 1) as usize];
            for j in 0..i {
                r -= lpc[j as usize] * autoc[(i - j) as usize];
            }
            r /= err;
    
            lpc[i as usize] = r;
    
            for j in 0..(i >> 1) {
                let tmp = lpc[j as usize];
                lpc[j as usize] += r * lpc[(i - 1 - j) as usize];
                lpc[(i - 1 - j) as usize] += r * tmp;
            }
            if i & 1 == 1 {
                lpc[(i >> 1) as usize] += lpc[(i >> 1) as usize] * r;
            }
    
            err *= 1.0 - r * r;
        }
    
        lpc
    }
    

    /// Quantize the predictor coefficients and find their shift factor
    /// 
    /// The shift factor `S` is computed from the maximum absolute value of a coefficient
    /// `L_max`. This value is computed as `precision - lg(L_max)` or to
    /// the maximum shift value of 1 << 5 = 31, whichever is smaller. Note that it is
    /// possible for this shift factor to be negative. In that case, the shift value
    /// will still be used in quantizing the coefficients but its effective value
    /// will be zero.
    /// 
    /// Quantization involves converting the provided floating-point coefficients
    /// into integers. Each of the values are rounded up or down depending on
    /// some accummulated rounding error `\epsilon`. Initially, this error is zero.
    /// For each coefficient `L_i`, the coefficient is multiplied (for positive shift)
    /// or divided (for negative shift) by `1 << abs(S)` to get the raw value `L_i_r + \epsilon`.
    /// Then, `L_i_r + \epsilon` is rounded away from zero to get the quantized coefficient.
    /// The new rounding error `\epsilon = L_i_r + \epsilon - round(L_i_r)` is then updated for the
    /// next coefficient.
    pub fn quantize_coeffs(lpc_coefs: &Vec <f64>, mut precision: u32) -> (Vec <u32>, u32) {
        todo!()
    }

    /// Compute the residuals from a given linear predictor
    /// 
    /// The residuals are computed with the provided quantized coefficients
    /// `qlp_coefs` and shift factor `qlp_shift`.
    pub fn get_residuals(data: &Vec <i32>, qlp_coefs: &Vec <u32>, predictor_order: u32, qlp_shift: u32) -> Option <Vec <i32>> {
        todo!()
    }

    /// Get the best coefficient precision
    /// 
    /// FLAC uses the bit depth and block size to determine the best coefficient
    /// precision. By default, the precision is 14 bits but can be one of the
    /// following depending on several parameters:
    /// 
    /// | Bit depth | Block size |     Best precision      |
    /// |-----------|------------|-------------------------|
    /// |   < 16    |     any    | max(1, 2 + bit_depth/2) |
    /// |     16    |     192    |           7             |
    /// |     16    |     384    |           8             |
    /// |     16    |     576    |           9             |
    /// |     16    |    1152    |          10             |
    /// |     16    |    2304    |          11             |
    /// |     16    |    4608    |          12             |
    /// |     16    |     any    |          13             |
    /// |   > 16    |     384    |          12             |
    /// |   > 16    |    1152    |          13             |
    /// |   > 16    |     any    |          14             |
    pub fn get_best_precision(bps: u32, block_size: u32) -> u32 {
        if bps < 16 {
            (2 + bps / 2).max(1)
        } else if bps == 16 {
            match block_size {
                192 => 7,
                384 => 8,
                576 => 9,
                1152 => 10,
                2304 => 11,
                4608 => 12,
                _ => 13,
            }
        } else {
            match block_size {
                384 => 12,
                1152 => 13,
                _ => 14,
            }
        }
    }
}