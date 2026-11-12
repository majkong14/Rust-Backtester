mod backtest;
mod indicators;
mod signals;

use backtest::backtest;
use signals::sma_crossover;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    println!("Enter prices, space-denominated:");
    io::stdin().read_line(&mut input)?;

    let mut prices = Vec::new();

    for value in input.split_whitespace() {
        prices.push(value.parse::<f64>()?);
    }

    input.clear();
    println!("Enter initial cash:");
    io::stdin().read_line(&mut input)?;
    let initial_cash = input.trim().parse::<f64>()?;

    input.clear();
    println!("Enter leverage:");
    io::stdin().read_line(&mut input)?;
    let leverage = input.trim().parse::<f64>()?;

    input.clear();
    println!("Reverse the logic? (Y/N):");
    io::stdin().read_line(&mut input)?;

    let reversed_logic = match input.trim().to_ascii_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => return Err("Enter Y or N".into()),
    };

    input.clear();
    println!("Enter fast MA period:");
    io::stdin().read_line(&mut input)?;
    let fast_period = input.trim().parse::<usize>()?;

    input.clear();
    println!("Enter slow MA period:");
    io::stdin().read_line(&mut input)?;
    let slow_period = input.trim().parse::<usize>()?;

    let signals =
        sma_crossover(&prices, fast_period, slow_period).expect("Error constructing signals");
    println!("Signals: {signals:?}");

    let equity = backtest(&prices, &signals, initial_cash, leverage, reversed_logic);
    println!("Equity: {equity:?}");

    Ok(())
}
