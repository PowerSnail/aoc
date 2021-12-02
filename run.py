import subprocess as sp
import sys
import pathlib

year = int(sys.argv[1])
day = int(sys.argv[2])
part = int(sys.argv[3])

sp.run("cargo build --release", shell=True, check=True, stdout=sp.PIPE, stderr=sp.PIPE)
pathlib.Path(f"outputs/{year}").mkdir(parents=True, exist_ok=True)
sp.run(f"target/release/aoc_{year} {day} {part} <inputs/{year}/day{day}.txt | tee outputs/{year}/day{day}-part{part}.txt", shell=True, check=True)
sp.run(f"cat outputs/day{day}-part{part}.txt | xsel -ib", shell=True, check=True)