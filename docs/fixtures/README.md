# SMA crossover reference fixtures

`sma_crossover_50_200/` contains the frozen reference inputs and outputs for the
functional, struct-based, and event-driven implementations of the backtester.
The CSV snapshots remain unchanged across implementation stages.

## Trading rules

- Trade SPY using SMA(50) and SMA(200) of its own `Close` prices.
- Use the complete saved history: **8,318 bars, 1993-01-29 through 2026-02-13**.
- Start with **USD 10,000** in cash and no shares.
- Each SMA includes the current close. An incomplete window has no value,
  represented by an empty CSV field.
- The first 199 bars produce no trading signal. Once both averages are
  available, the target is long when `SMA50 > SMA200` and cash otherwise.
  Equality closes an existing position.
- If the condition is already true on the first bar with a valid SMA200,
  enter without waiting for another crossover.
- Observe the signal after the close and execute at the **next row's Open**.
  The next row is the next trading session, rather than the next calendar day.
- Buy `floor(available capital / Open)` whole shares on entry. Keep the cash
  remainder and sell all shares on exit.
- Resize only on entry. Hold the share quantity between signal transitions;
  reinvest the available capital on the next entry.
- Use no leverage, short positions, commissions, slippage, or cash interest.
  Cash must remain nonnegative.
- Use the saved OHLC values as supplied, without additional price adjustments
  or separate dividend cashflows. `Unadjusted Close` is retained in the input
  snapshot and is unused by the strategy.
- After execution at the open, value the portfolio at that session's close.
- Keep any open position at the end of the history. A signal on the final
  row remains unfilled when there is no following row.

## Files

Dates use `YYYY-MM-DD`. Numerical outputs use 17 significant digits so the
stored values can be restored as `f64`.

| File | Contents |
| --- | --- |
| `prices.csv` | Frozen daily SPY input prices and volume |
| `equity.csv` | Total account value in USD at each close |
| `weights.csv` | Actual SPY and cash weights at each close |
| `positions.csv` | Share quantity, cash, and position market value at each close |
| `signals.csv` | Both SMAs, the target SPY weight after the close, and a decision-event flag |
| `fills.csv` | Execution date, signal date, side, signed share quantity, price, and fee |
| `metadata.json` | Trading parameters, summary, baseline validation results, and CSV checksums |

```text
market_value[t] = quantity[t] * Close[t]
equity[t]       = cash[t] + market_value[t]
weight_SPY[t]   = market_value[t] / equity[t]
weight_CASH[t]  = cash[t] / equity[t]
```

`target_SPY` is the decision made after that row's close. Actual weights reflect
the shares and cash already held at the close. `decision_event=1` marks a
decision eligible for execution on the following row. Fill quantities are
positive for buys and negative for sells.

## Comparison contract

Dates, signals, share quantities, and trade sides must match exactly.
SMA warm-up fields must be empty in the same rows. Compare floating-point
values using:

```text
abs(actual - expected) <= 1e-9 + 1e-12 * abs(expected)
```

Differences in floating-point operation order must preserve trading decisions
and share quantities.

## Reference result

- First buy: **1993-11-12**, following the **1993-11-11** signal.
- Executions: **31**, comprising 16 buys and 15 sells.
- Final equity: **USD 192,479.03462000002**.
- Final position: **282 SPY shares** and **USD 225.53462000002037** in cash.

At creation, the readable reference and optimized execution paths agreed on
account and position snapshots, fills, and decision masks. The largest
numerical snapshot difference was zero. Independent checks covered every SMA
window, prefix signal causality, execution timing, sizing, and cash accounting.
These are recorded baseline checks; the original engine is outside this repository.

## Verify the saved files

From the repository root, run:

```sh
python3 docs/fixtures/verify_fixtures.py
```

The verifier uses Python's standard library. It checks CSV hashes, aligned dates,
summary values, and the equity and weight identities above. It reads the saved
files without regenerating the reference backtest.

The existing CSV snapshots are retained as reference data. New datasets should
have explicit source and redistribution terms. A change to the strategy or
inputs calls for a separately named fixture rather than overwriting this one
to accommodate a changed implementation.
