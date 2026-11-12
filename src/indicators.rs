pub fn sma(prices: &[f64], period: usize) -> Result<Vec<Option<f64>>, &'static str> {
    let mut result: Vec<Option<f64>> = vec![None; prices.len()];

    if period == 0 || prices.len() < period {
        return Err("Period must be bigger or equal to the prices length");
    }

    for end in (period - 1)..prices.len() {
        let start = end + 1 - period;
        let window = &prices[start..=end];
        let sum: f64 = window.iter().sum();
        let mean = sum / period as f64;
        result[end] = Some(mean);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::sma;

    fn prices_fixture() -> Vec<f64> {
        vec![1.0, 2.0, 3.0, 4.0, 5.0]
    }

    #[test]
    fn sma_calculates_series() {
        assert_eq!(
            sma(&prices_fixture(), 3),
            Ok(vec![None, None, Some(2.0), Some(3.0), Some(4.0)])
        )
    }

    #[test]
    fn sma_empty_input() {
        assert_eq!(
            sma(&[], 3),
            Err("Period must be bigger or equal to the prices length")
        );
    }

    #[test]
    fn sma_zero_period() {
        let prices = prices_fixture();
        assert_eq!(
            sma(&prices, 0),
            Err("Period must be bigger or equal to the prices length")
        );
    }

    #[test]
    fn sma_unit_period() {
        let prices = prices_fixture();
        assert_eq!(
            sma(&prices, 1),
            Ok(vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)])
        );
    }

    #[test]
    fn sma_equal_len_period() {
        let prices = prices_fixture();
        assert_eq!(
            sma(&prices, prices.len()),
            Ok(vec![None, None, None, None, Some(3.0)])
        )
    }

    #[test]
    fn sma_insuff_price_hist() {
        let prices = prices_fixture();
        assert_eq!(
            sma(&prices, prices.len() + 5),
            Err("Period must be bigger or equal to the prices length")
        )
    }
}
