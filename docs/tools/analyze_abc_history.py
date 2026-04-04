#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from datetime import datetime, timedelta
from pathlib import Path

ABC_RE = re.compile(r"abc(\d{3})$")
ABC_FILE_RE = re.compile(r"abc(\d{3})_([a-z])\.rs$")
LETTERS = "abcdefg"


@dataclass
class ContestRecord:
    name: str
    number: int
    solved: dict[str, bool]
    commented: dict[str, bool]


def has_comment(path: Path) -> bool:
    return any(line.lstrip().startswith("//") for line in path.read_text().splitlines())


def load_records(contest_root: Path) -> list[ContestRecord]:
    records: list[ContestRecord] = []
    for entry in sorted(contest_root.iterdir(), key=lambda p: p.name):
        if not entry.is_dir():
            continue
        match = ABC_RE.fullmatch(entry.name)
        if not match:
            continue

        solved = {letter: False for letter in LETTERS}
        commented = {letter: False for letter in LETTERS}
        for letter in LETTERS:
            path = entry / "src" / "bin" / f"{letter}.rs"
            if not path.exists():
                continue
            solved[letter] = True
            commented[letter] = has_comment(path)

        records.append(
            ContestRecord(
                name=entry.name,
                number=int(match.group(1)),
                solved=solved,
                commented=commented,
            )
        )

    records.sort(key=lambda record: record.number)
    return records


def load_kenchon_abc_records(kenchon_bin_root: Path, max_age_days: int) -> list[ContestRecord]:
    now = datetime.now()
    records: list[ContestRecord] = []
    for path in sorted(kenchon_bin_root.glob("*.rs"), key=lambda p: p.name):
        match = ABC_FILE_RE.fullmatch(path.name)
        if not match:
            continue
        if now - datetime.fromtimestamp(path.stat().st_mtime) > timedelta(days=max_age_days):
            continue

        letter = match.group(2)
        solved = {current: False for current in LETTERS}
        commented = {current: False for current in LETTERS}
        solved[letter] = True
        commented[letter] = has_comment(path)
        records.append(
            ContestRecord(
                name=path.stem,
                number=int(match.group(1)),
                solved=solved,
                commented=commented,
            )
        )

    records.sort(key=lambda record: (record.number, record.name))
    return records


def count_stats(records: list[ContestRecord]) -> dict[str, dict[str, int]]:
    stats: dict[str, dict[str, int]] = {
        letter: {"solved": 0, "commented": 0, "self_like": 0} for letter in LETTERS
    }
    for record in records:
        for letter in LETTERS:
            if not record.solved[letter]:
                continue
            stats[letter]["solved"] += 1
            if record.commented[letter]:
                stats[letter]["commented"] += 1
            else:
                stats[letter]["self_like"] += 1
    return stats


def print_stats(title: str, records: list[ContestRecord], unit_label: str = "records") -> None:
    if not records:
        print(f"{title}: no records")
        return

    print(title)
    print(f"  {unit_label}: {len(records)} ({records[0].name} .. {records[-1].name})")
    print("  letter  solved  self_like  commented  comment_ratio")
    stats = count_stats(records)
    for letter in LETTERS:
        solved = stats[letter]["solved"]
        commented = stats[letter]["commented"]
        self_like = stats[letter]["self_like"]
        ratio = commented / solved if solved else 0.0
        print(
            f"  {letter:>6}  {solved:>6}  {self_like:>9}  {commented:>9}  {ratio:>13.1%}"
        )
    print()


def print_weighted_blend(
    title: str,
    primary_records: list[ContestRecord],
    supplemental_records: list[ContestRecord],
    supplemental_weight: float,
) -> None:
    primary = count_stats(primary_records)
    supplemental = count_stats(supplemental_records)

    print(title)
    print(
        "  letter  weighted_solved  weighted_self_like  weighted_commented  weighted_self_ratio"
    )
    for letter in LETTERS:
        weighted_solved = (
            primary[letter]["solved"] + supplemental_weight * supplemental[letter]["solved"]
        )
        weighted_self = (
            primary[letter]["self_like"] + supplemental_weight * supplemental[letter]["self_like"]
        )
        weighted_commented = (
            primary[letter]["commented"]
            + supplemental_weight * supplemental[letter]["commented"]
        )
        ratio = weighted_self / weighted_solved if weighted_solved else 0.0
        print(
            f"  {letter:>6}  {weighted_solved:>14.1f}  {weighted_self:>18.1f}  "
            f"{weighted_commented:>18.1f}  {ratio:>18.1%}"
        )
    print()


def main() -> None:
    parser = argparse.ArgumentParser(description="Analyze recorded ABC history under src/contest.")
    parser.add_argument(
        "--contest-root",
        type=Path,
        default=Path(__file__).resolve().parents[2],
        help="Path to src/contest",
    )
    parser.add_argument(
        "--recent-count",
        type=int,
        default=25,
        help="How many recent recorded ABC contests to treat as the primary window.",
    )
    parser.add_argument(
        "--kenchon-days",
        type=int,
        default=0,
        help="If > 0, include ABC practice files under kenchon modified within this many days.",
    )
    parser.add_argument(
        "--kenchon-weight",
        type=float,
        default=0.6,
        help="Weight multiplier for supplemental kenchon ABC practice.",
    )
    parser.add_argument(
        "--kenchon-root",
        type=Path,
        default=None,
        help="Path to kenchon/src/bin. Defaults to <contest-root>/kenchon/src/bin.",
    )
    args = parser.parse_args()

    records = load_records(args.contest_root)
    if not records:
        raise SystemExit("No ABC records found.")

    print(f"Recorded ABC contests: {len(records)}")
    print(f"Range: {records[0].name} .. {records[-1].name}")
    print()

    print_stats("Overall", records, unit_label="contests")

    recent = records[-args.recent_count :]
    middle = records[-2 * args.recent_count : -args.recent_count]
    older = records[-3 * args.recent_count : -2 * args.recent_count]

    print_stats(f"Recent {len(recent)} recorded contests", recent, unit_label="contests")
    if middle:
        print_stats(f"Previous {len(middle)} recorded contests", middle, unit_label="contests")
    if older:
        print_stats(f"Older {len(older)} recorded contests", older, unit_label="contests")

    if args.kenchon_days > 0:
        kenchon_root = args.kenchon_root or args.contest_root / "kenchon" / "src" / "bin"
        supplemental = load_kenchon_abc_records(kenchon_root, args.kenchon_days)
        print_stats(
            f"Recent kenchon ABC practice (last {args.kenchon_days} days)",
            supplemental,
            unit_label="files",
        )
        if supplemental:
            print_weighted_blend(
                f"Weighted blend: recent contests + {args.kenchon_weight:.2f} * kenchon supplement",
                recent,
                supplemental,
                args.kenchon_weight,
            )


if __name__ == "__main__":
    main()
