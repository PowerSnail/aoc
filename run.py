import subprocess as sp
import sys

day = int(sys.argv[1])
part = int(sys.argv[2])

sp.run("cargo build --release", shell=True, check=True, stdout=sp.PIPE, stderr=sp.PIPE)
sp.run(f"target/release/aoc_2021 {day} {part} <inputs/day{day}.txt | tee outputs/day{day}-part{part}.txt", shell=True, check=True)
sp.run(f"cat outputs/day{day}-part{part}.txt | xsel -ib", shell=True, check=True)