"""Verify frozen reference CSVs using only the Python standard library."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
from pathlib import Path

FILES = (
    "prices.csv",
    "equity.csv",
    "weights.csv",
    "positions.csv",
    "signals.csv",
    "fills.csv",
)
DEFAULT_FIXTURE = Path(__file__).resolve().parent / "sma_crossover_50_200"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def verify(fixture: Path) -> None:
    metadata = json.loads((fixture / "metadata.json").read_text())
    hashes = metadata["file_sha256"]
    require(set(hashes) == set(FILES), "Unexpected fixture file manifest")
    tables = {}
    for filename in FILES:
        path = fixture / filename
        actual_hash = hashlib.sha256(path.read_bytes()).hexdigest()
        require(actual_hash == hashes[filename], f"Checksum mismatch: {filename}")
        with path.open(newline="") as source:
            tables[filename] = list(csv.DictReader(source))

    prices = tables["prices.csv"]
    dates = [row["Date"] for row in prices]
    summary = metadata["summary"]
    require(len(dates) == summary["rows"], "Unexpected row count")
    require(dates == sorted(set(dates)), "Dates must be sorted and unique")
    require(
        dates[0] == summary["start"] and dates[-1] == summary["end"],
        "Unexpected date range",
    )
    for filename in FILES[1:-1]:
        require(
            [row["Date"] for row in tables[filename]] == dates,
            f"Misaligned dates: {filename}",
        )

    rtol = metadata["validation"]["comparison_rtol"]
    atol = metadata["validation"]["comparison_atol"]

    def close(actual: float, expected: float, label: str) -> None:
        require(
            math.isfinite(actual)
            and math.isfinite(expected)
            and abs(actual - expected) <= atol + rtol * abs(expected),
            f"Value mismatch: {label}",
        )

    for price, position, equity_row, weight in zip(
        prices,
        tables["positions.csv"],
        tables["equity.csv"],
        tables["weights.csv"],
        strict=True,
    ):
        date = price["Date"]
        quantity = float(position["quantity"])
        cash = float(position["cash"])
        market_value = float(position["market_value"])
        equity = float(equity_row["equity"])
        require(
            quantity >= 0 and quantity.is_integer(), f"Invalid share quantity: {date}"
        )
        require(cash >= -atol and equity > 0, f"Invalid account balance: {date}")
        close(market_value, quantity * float(price["Close"]), f"market value on {date}")
        close(equity, cash + market_value, f"equity on {date}")
        close(float(weight["SPY"]), market_value / equity, f"SPY weight on {date}")
        close(float(weight["CASH"]), cash / equity, f"cash weight on {date}")
        close(
            float(weight["SPY"]) + float(weight["CASH"]), 1.0, f"weight sum on {date}"
        )

    close(
        float(tables["equity.csv"][0]["equity"]),
        summary["initial_equity"],
        "initial equity",
    )
    close(
        float(tables["equity.csv"][-1]["equity"]),
        summary["final_equity"],
        "final equity",
    )
    close(
        float(tables["positions.csv"][-1]["cash"]), summary["final_cash"], "final cash"
    )
    close(
        float(tables["positions.csv"][-1]["quantity"]),
        summary["final_quantity"],
        "final quantity",
    )
    fills = tables["fills.csv"]
    require(len(fills) == summary["fills"], "Unexpected fill count")
    require(
        sum(row["side"] == "BUY" for row in fills) == summary["buys"],
        "Unexpected buy count",
    )
    require(
        sum(row["side"] == "SELL" for row in fills) == summary["sells"],
        "Unexpected sell count",
    )
    require(fills[0]["Date"] == summary["first_fill"], "Unexpected first fill")
    print(
        f"Verified {len(FILES)} CSV files, {len(dates):,} bars and {len(fills)} fills."
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixture-dir", type=Path, default=DEFAULT_FIXTURE)
    args = parser.parse_args()
    verify(args.fixture_dir)


if __name__ == "__main__":
    main()
