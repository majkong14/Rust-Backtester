use crate::indicators::sma;
use std::cmp::max;

pub fn sma_crossover(
    prices: &[f64],
    fast_period: usize,
    slow_period: usize,
) -> Result<Vec<bool>, &'static str> {
    let mut result = vec![false; prices.len()];

    if prices.len() < max(fast_period, slow_period) {
        return Err(
            "Length of prices supplied must be equal or bigger than the MAX(fast_period, slow_period))",
        );
    }

    let fast_ma = sma(prices, fast_period)?;
    let slow_ma = sma(prices, slow_period)?;

    for (i, (fast, slow)) in fast_ma.iter().zip(&slow_ma).enumerate() {
        result[i] = match (fast, slow) {
            (Some(fast), Some(slow)) => *fast > *slow,
            _ => false,
        };
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::sma_crossover;

    fn prices_fixture() -> Vec<f64> {
        vec![1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0]
    }

    #[test]
    fn dir_change() {
        assert_eq!(
            sma_crossover(&prices_fixture(), 2, 3),
            Ok(vec![false, false, true, true, false, false, true])
        );
    }

    #[test]
    fn rising() {
        let prices = &[1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(
            sma_crossover(prices, 2, 3),
            Ok(vec![false, false, true, true, true])
        );
    }

    #[test]
    fn falling() {
        let prices = &[5.0, 4.0, 3.0, 2.0, 1.0];
        assert_eq!(sma_crossover(prices, 2, 3), Ok(vec![false; 5]));
    }

    #[test]
    fn equal_means() {
        let prices = &[3.0; 5];
        assert_eq!(sma_crossover(prices, 2, 3), Ok(vec![false; 5]));
    }

    #[test]
    fn equal_hist() {
        let prices = &[1.0, 2.0, 3.0];
        assert_eq!(sma_crossover(prices, 2, 3), Ok(vec![false, false, true]));
    }

    #[test]
    fn zero_fast() {
        assert!(sma_crossover(&prices_fixture(), 0, 3).is_err());
    }

    #[test]
    fn zero_slow() {
        assert!(sma_crossover(&prices_fixture(), 2, 0).is_err());
    }

    #[test]
    fn insuff_hist() {
        let prices = &[1.0, 2.0];
        assert!(sma_crossover(prices, 2, 3).is_err());
    }

    #[test]
    fn empty_input() {
        let prices = &[];
        assert!(sma_crossover(prices, 2, 3).is_err());
    }
}
