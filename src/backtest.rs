pub fn backtest(
    prices: &[f64],
    signals: &[bool],
    initial_cash: f64,
    leverage: f64,
    reversed_logic: bool,
) -> Result<Vec<f64>, &'static str> {
    //checks
    {
        if prices.len() != signals.len() {
            return Err("Input lengths must match");
        }
        if prices.is_empty() || signals.is_empty() {
            return Err("Input must not be empty");
        }
        if !leverage.is_finite() || leverage <= 0.0 {
            return Err("Leverage must be positive");
        }
        if !initial_cash.is_finite() || initial_cash <= 0.0 {
            return Err("Initial cash must be positive");
        }
        if prices.iter().any(|p| !p.is_finite() || *p <= 0.0) {
            return Err("Prices must be finite and positive");
        }
    }

    // declarations
    let mut open_price = 0.0;
    let mut in_position = false;
    let mut cash = initial_cash;
    let mut base_cash = initial_cash;
    let mut result = vec![base_cash; signals.len()];

    let rev_logic_mask = (-1.0_f64).powi(reversed_logic as i32);

    // main loop
    for t in 1..(signals.len()) {
        let (curr_price, prev_signal) = (prices[t], signals[t - 1]);

        if !in_position {
            result[t] = cash;

            if prev_signal {
                in_position = true;
                base_cash = cash;
                open_price = curr_price;
            }
        } else {
            result[t] =
                base_cash * (leverage * rev_logic_mask * (curr_price / open_price - 1.0) + 1.0);

            if !prev_signal {
                in_position = false;
                cash = result[t];
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::backtest;

    //constants
    const TOL: f64 = 1e-5;

    //helper
    fn cmp_with_tol(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    //test template functions
    fn test_backtest_equity(
        expected_eq: &[f64],
        prices: &[f64],
        signals: &[bool],
        initial_cash: f64,
        leverage: f64,
        reversed_logic: bool,
    ) {
        let actual_eq = backtest(prices, signals, initial_cash, leverage, reversed_logic);

        let actual_eq = match actual_eq {
            Ok(values) => values,
            Err(error) => panic!("backtest returned an error: {error}"),
        };

        assert_eq!(actual_eq.len(), expected_eq.len());

        assert!(
            actual_eq
                .iter()
                .zip(expected_eq.iter())
                .all(|(a, b)| cmp_with_tol(*a, *b, TOL))
        )
    }

    fn test_backtest_err(
        expected_err: &str,
        prices: &[f64],
        signals: &[bool],
        initial_cash: f64,
        leverage: f64,
        reversed_logic: bool,
    ) {
        let actual_eq = backtest(prices, signals, initial_cash, leverage, reversed_logic);

        match actual_eq {
            Ok(_) => {}
            Err(error) => {
                assert_eq!(error, expected_err);
            }
        }
    }

    //tests
    #[test]
    fn test_expected_equity() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = false;

        let expected_eq = &[
            1000.0,
            1000.0,
            1000.0 * (120.0 / 110.0),
            1000.0 * (130.0 / 110.0),
        ];
        let prices = &[100.0, 110.0, 120.0, 130.0];
        let signals = &[true, true, false, false];

        test_backtest_equity(
            expected_eq,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_no_entries() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = false;

        let expected_eq = &[1000.0; 4];
        let prices = &[100.0, 110.0, 120.0, 130.0];
        let signals = &[false; 4];

        test_backtest_equity(
            expected_eq,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_last_signal_true() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = false;

        let expected_eq = &[1000.0; 4];
        let prices = &[100.0, 110.0, 120.0, 130.0];
        let signals = &[false, false, false, true];

        test_backtest_equity(
            expected_eq,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_leverage() {
        let leverage: f64 = 2.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = false;

        let expected_eq = &[
            1000.0,
            1000.0,
            1000.0 * (1.0 + 2.0 * (120.0 / 110.0 - 1.0)),
            1000.0 * (1.0 + 2.0 * (130.0 / 110.0 - 1.0)),
        ];
        let prices = &[100.0, 110.0, 120.0, 130.0];
        let signals = &[true, true, false, false];

        test_backtest_equity(
            expected_eq,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_reversed_logic() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = true;

        let expected_eq = &[
            1000.0,
            1000.0,
            1000.0 * (1.0 - (120.0 / 110.0 - 1.0)),
            1000.0 * (1.0 - (130.0 / 110.0 - 1.0)),
        ];
        let prices = &[100.0, 110.0, 120.0, 130.0];
        let signals = &[true, true, false, false];

        test_backtest_equity(
            expected_eq,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_input_len_mismatch() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = true;

        let prices = &[110.0, 120.0, 130.0];
        let signals = &[true, true, false, false];
        let expected_err = "Input lengths must match";

        test_backtest_err(
            expected_err,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_empty_inputs() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = true;

        let prices = &[];
        let signals = &[];
        let expected_err = "Input must not be empty";

        test_backtest_err(
            expected_err,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_negative_leverage() {
        let leverage: f64 = -1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = true;

        let prices = &[100.0, 110.0, 120.0, 130.0];
        let signals = &[true, true, false, false];
        let expected_err = "Leverage must be positive";

        test_backtest_err(
            expected_err,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_negative_initial_cash() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = -1000.0;
        let reversed_logic: bool = true;

        let prices = &[100.0, 110.0, 120.0, 130.0];
        let signals = &[true, true, false, false];
        let expected_err = "Initial cash must be positive";

        test_backtest_err(
            expected_err,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_nan_prices() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = true;

        let prices = &[100.0, f64::NAN, 120.0, 130.0];
        let signals = &[true, true, false, false];
        let expected_err = "Prices must be finite and positive";

        test_backtest_err(
            expected_err,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_inf_prices() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = true;

        let prices = &[100.0, f64::INFINITY, 120.0, 130.0];
        let signals = &[true, true, false, false];
        let expected_err = "Prices must be finite and positive";

        test_backtest_err(
            expected_err,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_zero_prices() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = true;

        let prices = &[100.0, 100.0, 120.0, 0.0];
        let signals = &[true, true, false, false];
        let expected_err = "Prices must be finite and positive";

        test_backtest_err(
            expected_err,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }

    #[test]
    fn test_neg_prices() {
        let leverage: f64 = 1.0;
        let initial_cash: f64 = 1000.0;
        let reversed_logic: bool = true;

        let prices = &[100.0, 100.0, 120.0, -130.0];
        let signals = &[true, true, false, false];
        let expected_err = "Prices must be finite and positive";

        test_backtest_err(
            expected_err,
            prices,
            signals,
            initial_cash,
            leverage,
            reversed_logic,
        )
    }
}
