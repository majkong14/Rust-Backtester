mod indicators;

use indicators::sma;

fn main() {
    let prices = [1.0, 2.0, 3.0, 4.0, 5.0];

    assert_eq!(sma(&[1.0, 2.0], 3), Ok(vec![None; 2]));
    assert_eq!(sma(&prices, 0), Ok(vec![None; prices.len()]));

    println!("{:?}", sma(&prices, 3));
    println!("{:?}", sma(&prices, 10));
    println!("{:?}", sma(&prices, 0));
}
