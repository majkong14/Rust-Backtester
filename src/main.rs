mod indicators;
mod signals;

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
    println!("Enter fast MA period:");
    io::stdin().read_line(&mut input)?;
    let fast_period = input.trim().parse::<usize>()?;

    input.clear();
    println!("Enter slow MA period:");
    io::stdin().read_line(&mut input)?;
    let slow_period = input.trim().parse::<usize>()?;

    let result = sma_crossover(&prices, fast_period, slow_period);
    println!("Signals: {result:?}");

    Ok(())
}
