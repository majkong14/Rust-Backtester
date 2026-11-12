# Rust Backtester

A hands-on project for learning Rust by implementing the same small backtester
in several programming styles. The starting point is a familiar problem from
Python: a single-instrument SMA crossover strategy.

The goal is to learn the language through working code: variables, functions,
ownership and borrowing, iterators, structs, traits, generics, file I/O, and
event processing.

## Roadmap

Each stage and substage belongs on a separate branch. The branch names below
describe the planned implementations.

| Stage | Approach | Expected result | Branch |
| --- | --- | --- | --- |
| 1 | Functional | Indicator and signal functions, explicit state, and a simple procedural backtest loop | `feature/functional` |
| 1.1 | Functional with vectorization | Explore array-based calculations while preserving the execution rules | `feature/functional-vectorized` |
| 2 | Structs and methods | Model indicators, signals, and the backtester with `struct` and `impl`; introduce traits and generics where useful | `feature/structs` |
| 2.1 | Structs with vectorization | Apply vectorized calculations within the same design | `feature/structs-vectorized` |
| 3 | Event-driven | Explicit events, a queue, and a synchronous processing loop | `feature/event-driven` |
| 3.1 | Asynchronous events | Replace the synchronous processing loop with an asynchronous implementation | `feature/event-driven-async` |

Stage 2 explores OOP through Rust's composition, methods, and traits. Stateful
position sizing may still require an ordered pass in the vectorized versions.
The asynchronous stage explores scheduling while preserving deterministic
historical execution.

Further stages can be added as the project develops.

## Current state

Stage 1 is in progress. The project currently includes:

- An SMA function returning `Result<Vec<Option<f64>>, &'static str>`.
- A crossover signal function returning `Result<Vec<bool>, &'static str>`.
- A functional backtest that applies each signal at the next price observation
  and returns an equity series.
- Interactive terminal input for prices, initial cash, leverage, reverse
  logic, and the two SMA periods.
- Unit tests for indicator windows, signal changes, warm-up, equality,
  backtest equity, leverage, reverse logic, no-entry behavior, and a signal on
  the final observation.

The backtest currently uses one supplied price series for both signal timing
and valuation. It applies the previous observation's signal at the current
observation and calculates equity at that same price. Separate open and close
prices, whole-share accounting, portfolio weights, CSV loading, and comparison
with the frozen reference fixtures remain to be implemented.

## Source layout

```text
src/
  main.rs        Terminal input, signal calculation, and backtest output
  backtest.rs    Functional backtest loop and its tests
  indicators.rs  SMA calculation and indicator tests
  signals.rs     SMA crossover decisions and signal tests
docs/fixtures/   Frozen reference data, trading rules, and a CSV verifier
```

## One strategy, one reference result

Every implementation uses the same frozen inputs and trading rules:

- One instrument: SPY, with SMA(50) and SMA(200) calculated from closing prices.
- Hold a long position when SMA(50) is above SMA(200); otherwise hold cash.
- Make decisions after the close and execute at the next trading session's open.
- Start with USD 10,000, trade whole shares, and use no leverage or trading costs.

The reference data lives in [`docs/fixtures/`](docs/fixtures/README.md). Equity,
actual portfolio weights, positions, and fills must match across all stages.
Dates, signals, and share quantities must match exactly; floating-point values
use the documented numerical tolerance.

## Project conventions

- Build each milestone in its own branch and keep completed versions available
  for comparison.
- Preserve the strategy and accounting rules when changing the architecture.
- Keep the learning steps small and explain the Rust concepts they introduce.
- Keep this repository self-contained: exclude private project names, source
  code, internal paths, and dependencies on private repositories.
- Preserve the existing fixture CSVs as fixed reference inputs and outputs.
  New datasets should have explicit source and redistribution terms.
- Leave changes in the working tree for review; the repository owner creates
  commits manually.

## Getting started

With Rust and Cargo available in the shell, start the interactive program:

```sh
cargo run --offline
```

Enter the requested values, one per line:

1. Prices separated by spaces, using a decimal point for fractional values.
2. Initial cash.
3. Leverage, greater than zero.
4. Reverse logic: `Y` or `N` (case-insensitive).
5. The fast SMA period.
6. The slow SMA period.

For example:

```text
1 2 3 2 1 2 3
10000
1
n
2
3
```

The program prints the calculated signals and the backtest result using Rust's
debug formatting (for example, `Equity: Ok([...])`). Input parsing and I/O
errors are propagated from `main`; invalid reverse-logic input returns an
error. Signal-construction errors currently cause a panic through `expect`.

Each period must be positive and no greater than the number of input prices.

Each signal describes the desired position after that session's close:
`true` means hold shares, and `false` means hold cash. A missing SMA during
warm-up produces `false`; equal averages also produce `false`. The current
prototype applies the signal at the next price observation; until separate
Open prices are loaded, that observation is the execution and valuation price.
Reverse logic multiplies the position return by `-1.0` while preserving the
same signal-entry and exit timing.

Run the Rust unit tests with:

```sh
cargo test --offline
```

To verify the saved fixtures with Python 3 and its standard library:

```sh
python3 docs/fixtures/verify_fixtures.py
```

## Local commit checks

Enable the repository's pre-commit hook after cloning:

```sh
git config --local core.hooksPath .githooks
```

Before each commit, the hook checks formatting, runs offline tests, treats
Clippy warnings as errors, and checks whitespace in both staged and unstaged
changes. A failed check blocks the commit.

Run the same checks manually with:

```sh
.githooks/pre-commit
```
