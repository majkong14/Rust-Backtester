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

With Rust and Cargo available in the shell:

```sh
cargo run
```

The current Rust program is the initial `Hello, world!` scaffold. The roadmap
describes the work ahead.

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
