#!/usr/bin/env python3

import argparse
import pathlib
import shutil
import statistics
import subprocess
import time


def benchmark(command: list[str], path: pathlib.Path, runs: int) -> tuple[float, float, float]:
    subprocess.run(
        [*command, str(path)],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    timings = []
    for _ in range(runs):
        started = time.perf_counter()
        subprocess.run(
            [*command, str(path)],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        timings.append(time.perf_counter() - started)

    return statistics.mean(timings), min(timings), max(timings)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("hfile_bin", type=pathlib.Path)
    parser.add_argument("bench_file", type=pathlib.Path)
    args = parser.parse_args()

    b3sum = shutil.which("b3sum")
    if b3sum is None:
        raise SystemExit("b3sum was not found in PATH")

    cases = (
        ("small", pathlib.Path("tests/test-file"), 30),
        ("large", args.bench_file, 7),
    )
    commands = {
        "hfile": [str(args.hfile_bin)],
        "b3sum": [b3sum],
    }

    for label, path, runs in cases:
        print(f"{label} {path}")
        results = {}
        for name, command in commands.items():
            average, minimum, maximum = benchmark(command, path, runs)
            results[name] = average
            print(f"  {name}: avg={average:.6f}s min={minimum:.6f}s max={maximum:.6f}s")
        print(f"  ratio: {results['hfile'] / results['b3sum']:.2f}x\n")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
