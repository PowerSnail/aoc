import math
import subprocess as sp
import sys
import pathlib
import functools
import time
import docopt
import re
from rich.console import Console
from rich.table import Table

shell = functools.partial(sp.run, shell=True, check=True, text=True)
console = Console()

CLI = """run.py

Usage:
    run.py [options] <year> <day> <part> 
    run.py list [<year>]               
    run.py tests [<year>]     
    run.py -h | --help

Options:
    -t          Run Test.
    -s          Save result.
    -h --help   Show this screen.
"""


def main():
    opts = docopt.docopt(CLI)

    if opts["list"]:
        if opts["<year>"]:
            list_result(opts["<year>"])
        else:
            for y in pathlib.Path(f"outputs").iterdir():
                list_result(y.name)

    elif opts["tests"]:
        if opts["<year>"]:
            tests(opts["<year>"])
        else:
            for y in pathlib.Path(f"outputs").iterdir():
                tests(y.name)
    else:
        try:
            run(opts["<year>"], opts["<day>"], opts["<part>"], opts["-s"], opts["-t"])
        except sp.CalledProcessError as err:
            if err.stdout:
                sys.stderr.write("STDOUT\n------\n")
                sys.stderr.write(err.stdout)
            if err.stderr:
                sys.stderr.write("STDERR\n------\n")
                sys.stderr.write(err.stderr)
            sys.stderr.write("------\n")


def list_result(year):
    out_dir = pathlib.Path(f"outputs/{year}")
    files = [
        re.match("day(\d+)-part(\d+).txt", file.name)
        for file in out_dir.glob("day*-part*.txt")
    ]
    files = {(int(match.group(1)), int(match.group(2))) for match in files}

    table = Table(title=f"Year {year}")
    table.add_column("Day", justify="right")
    table.add_column("Part 1")
    table.add_column("Part 2")

    for day in range(1, 26):
        table.add_row(
            str(day),
            ("✓" if (day, 1) in files else "❌"),
            ("✓" if (day, 2) in files else "❌"),
        )

    console.print(table)


def tests(year):
    out_dir = pathlib.Path(f"outputs/{year}")
    files = [
        re.match("day(\d+)-part(\d+).txt", file.name)
        for file in out_dir.glob("day*-part*.txt")
    ]
    files = {(int(match.group(1)), int(match.group(2))) for match in files}
    results = {}
    for (day, part) in files:
        try:
            console.print(f"Day {day} Part {part}")
            results[(day, part)] = run(year, day, part, False, False)
        except sp.CalledProcessError as err:
            if err.stdout:
                sys.stderr.write("STDOUT\n------\n")
                sys.stderr.write(err.stdout)
            if err.stderr:
                sys.stderr.write("STDERR\n------\n")
                sys.stderr.write(err.stderr)
            sys.stderr.write("------\n")

    table = Table(title=f"Year {year}")
    table.add_column("Day", justify="right")
    table.add_column("Part 1")
    table.add_column("Part 1 Time")
    table.add_column("Part 2")
    table.add_column("Part 2 Time")

    chars = {
        True: "✓",
        False: "❌",
        None: "○",
    }

    for day in range(1, 26):
        part1_result, part1_time = results.get((day, 1), (None, math.nan))
        part2_result, part2_time = results.get((day, 2), (None, math.nan))
        table.add_row(str(day), chars[part1_result], f"{part1_time:.4f}", chars[part2_result], f"{part2_time:.4f}")

    console.print(table)


def run(year, day, part, to_save, to_test):
    out_dir = pathlib.Path(f"outputs/{year}")
    out_dir.mkdir(parents=True, exist_ok=True)
    out_file = out_dir / f"day{day}-part{part}.txt"

    in_dir = pathlib.Path(f"inputs/{year}")
    in_dir.mkdir(parents=True, exist_ok=True)
    in_file = "inputs/test.txt" if to_test else in_dir / f"day{day}.txt"

    print("Compiling...")
    shell("cargo build --release", stdout=sp.DEVNULL, stderr=sp.DEVNULL)

    print("Running solution...")
    tic = time.time()
    result = shell(
        f"target/release/aoc {year} {day} {part} <{in_file}",
        stdout=sp.PIPE,
        stderr=None,
    )
    toc = time.time()
    print(f"Finished in {toc - tic:.4f}s")

    answer = result.stdout
    if result.stderr:
        print(result.stderr)
    print(answer)
    shell("xsel -ib", input=answer)

    if to_save:
        with out_file.open("w") as file:
            file.write(answer)
    elif not to_test:
        if out_file.exists():
            result = shell(f"diff {out_file} - || true", input=answer)
            return result.returncode == 0, toc - tic
    return True, toc - tic


main()
