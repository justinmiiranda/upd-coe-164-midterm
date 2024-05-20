pub struct VarPredictor;

impl VarPredictor {
    /// Get the correlation of a vector of data
    pub fn get_autocorrelation(data: &Vec<i32>, lag: u32) -> Vec<f64> {
        // https://www.scicoding.com/4-ways-of-calculating-autocorrelation-in-python/
        let n = data.len();
        let mean: f64 = data.iter().sum::<i32>() as f64 / n as f64;
    
        let variance: f64 = data.iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / n as f64;
    
        let ndata: Vec<f64> = data.iter().map(|&x| x as f64 - mean).collect();
    
        let l = lag as usize;
        let mut c: Vec<f64> = vec![0.0; 1];
        if l > 0 {
            let tmp: Vec<f64> = (0..n - l)
                .map(|i| ndata[l + i] * ndata[i])
                .collect();
    
            c[0] = tmp.iter().sum::<f64>() / n as f64 / variance;
        } else {
            c[0] = 1.0;
        }
    
        c
    }

    /// Get the predictor coefficients
    /// 
    /// The coefficients are computed using the Levinson-Durbin algorithm.
    pub fn get_predictor_coeffs(autoc: &Vec<f64>, predictor_order: u32) -> Vec<f64> {
        // https://www.musicdsp.org/en/latest/Analysis/137-lpc-analysis-autocorrelation-levinson-durbin-recursion.html#comments
        let mut a = vec![0.0; (predictor_order + 1) as usize];
        let mut am1 = vec![0.0; (predictor_order + 1) as usize];
        let mut k = vec![0.0; predictor_order as usize];
    
        if autoc[0] == 0.0 {
            for i in 1..=predictor_order as usize {
                k[i - 1] = 0.0;
                a[i] = 0.0;
            }
        } else {
            let mut km;
            let mut em1;
            let mut em;
            let mut err;
    
            a[0] = 1.0;
            am1[0] = 1.0;
            km = 0.0;
            em1 = autoc[0];
    
            for m in 1..=predictor_order as usize {
                err = 0.0;
                for k in 1..m {
                    err += am1[k] * autoc[m - k];
                }
    
                km = (autoc[m] - err) / em1;
                k[m - 1] = -km;
                a[m] = km;
    
                for k in 1..m {
                    a[k] = am1[k] - km * am1[m - k];
                }
    
                em = (1.0 - km * km) * em1;
    
                for s in 0..=predictor_order as usize {
                    am1[s] = a[s];
                }
    
                em1 = em;
            }
        }
    
        a
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
        // (Vec <u32>: quantized_coeffs, u32: shift_factor)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocorrelation() {
        // https://www.scicoding.com/4-ways-of-calculating-autocorrelation-in-python/
        let data = vec![
            3, 16, 156, 47, 246, 176, 233, 140, 130, 101, 166, 201, 200, 116, 118, 247, 
            209, 52, 153, 232, 128, 27, 192, 168, 208, 187, 228, 86, 30, 151, 18, 254, 
            76, 112, 67, 244, 179, 150, 89, 49, 83, 147, 90, 33, 6, 158, 80, 35, 186, 127
        ];

        let lags: Vec<u32> = (0..10).collect();
        let expected_results = vec![
            1.0, 0.07326561, 0.01341434, -0.03866088, 0.13064865,
            -0.05907283, -0.00449197, 0.08829021, -0.05690311, 0.03172606
        ];

        for (i, &lag) in lags.iter().enumerate() {
            let autocorrelation = VarPredictor::get_autocorrelation(&data, lag);
            assert!((autocorrelation[0] - expected_results[i]).abs() < 1e-6, "Failed at lag {}: expected {}, got {}", lag, expected_results[i], autocorrelation[0]);
        }
    }
}