pub struct FixedPredictor;

impl FixedPredictor {
    /// Get order that yields the least sum of residuals
    /// 
    /// The predictor orders are from 0 to 4 inclusive and is retrieved
    /// by finding the predictor that yields the *minimum* sum of residuals
    /// for the given `data` and derived predictor.
    pub fn best_predictor_order(data: Vec <i32>) -> Option <u32> {
        let mut res_0 = FixedPredictor::get_residuals(data, 0);
        let mut res_1 = FixedPredictor::get_residuals(data, 1);
        let mut res_2 = FixedPredictor::get_residuals(data, 2);
        let mut res_3 = FixedPredictor::get_residuals(data, 3);
        let mut res_4 = FixedPredictor::get_residuals(data, 4);

        let sum_0 = FixedPredictor::calculate_sum(res_0);
        let sum_1 = FixedPredictor::calculate_sum(res_1);
        let sum_2 = FixedPredictor::calculate_sum(res_2);
        let sum_3 = FixedPredictor::calculate_sum(res_3);
        let sum_4 = FixedPredictor::calculate_sum(res_4);
        
        let min_predictor_order = vec![sum_0, sum_1, sum_2, sum_3, sum_4]
            .into_iter()
            .enumerate()
            .filter_map(|(index, sum)| sum.map(|value| (index, value)))
            .min_by_key(|&(_, sum)| sum)
            .map(|(index, _)| index);

        Some(min_predictor_order)
    }

    /// Get residuals of a fixed predictor order 
    /// 
    /// The predictor orders are from 0 to 4 inclusive and corresponds
    /// to one of the five "fixed" predictor orders written in the FLAC
    /// specification. The predictor orders are defined as follows:
    /// 
    /// 0: r[i] = 0
    /// 1: r[i] = data[i - 1]
    /// 2: r[i] = 2 * data[i - 1] - data[i - 2]
    /// 3: r[i] = 3 * data[i - 1] - 3 * data[i - 2] + data[i - 3]
    /// 4: r[i] = 4 * data[i - 1] - 6 * data[i - 2] + 4 data[i - 3] - data[i - 4]
    /// 
    /// This function returns a vector with each element containing data[i] - r[i].
    /// 
    /// # Errors
    /// `None` is returned if an error occurs in the function. This includes whether
    /// the predictor order provided is not within 0 and 4 inclusive and whether the
    /// size of `data` is less than the predictor order.
    pub fn get_residuals(data: Vec <i32>, predictor_order: u32) -> Option <Vec <i32>> {
        if predictor_order > 4 {
            return  None;
        }

        let mut res = Vec::new();

        for i in 0..=data.len() {
            if i >= predictor_order as usize {
                res.push(FixedPredictor::calculate_residual(&data, predictor_order, i));
            } else {
                res.push(0)
            }
        }

        return  Some(res);
    }

    pub fn calculate_residual(data: &Vec<i32>, predictor_order: u32, i: usize) -> i32 {
        if predictor_order == 0 {
            data[i]
        } else if predictor_order == 1 {
            data[i] - (data[i - 1])
        }  else if predictor_order == 2 {
            data[i] - (2 * data[i - 1] - data[i - 2])
        } else if predictor_order == 3 {
            data[i] - (3 * data[i - 1] - 3 * data[i - 2] + data[i - 3])
        } else {
            data[i] - (4 * data[i - 1] - 6 * data[i - 2] + 4 * data[i - 3] - data[i - 4])
        }
    }

    fn calculate_sum(vector: Option<Vec<i32>>) -> Option<i32> {
        match vector {
            Some(vec) => {
                let sum: i32 = vec.iter().sum();
                Some(sum)
            }
            None => None,
        }
    }
    
}