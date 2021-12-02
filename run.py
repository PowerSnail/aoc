import subprocess as sp
import sys
import pathlib

year = int(sys.argv[1])
day = int(sys.argv[2])
part = int(sys.argv[3])

out_dir = pathlib.Path(f"outputs/{year}")
out_dir.mkdir(parents=True, exist_ok=True)
out_file = out_dir / f"day{day}-part{part}.txt"

in_dir = pathlib.Path(f"inputs/{year}")
in_dir.mkdir(parents=True, exist_ok=True)
in_file = in_dir / f"day{day}.txt"

sp.run("cargo build --release", shell=True, check=True, stdout=sp.PIPE, stderr=sp.PIPE)
sp.run(
    f"target/release/aoc {year} {day} {part} <{in_file} | tee {out_file}",
    shell=True,
    check=True,
)
sp.run(f"cat {out_file} | xsel -ib", shell=True, check=True)
