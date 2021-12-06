import subprocess as sp
import sys
import pathlib
import functools

shell = functools.partial(sp.run, shell=True, check=True, text=True)

year = int(sys.argv[1])
day = int(sys.argv[2])
part = int(sys.argv[3])
to_save = "-s" in sys.argv

out_dir = pathlib.Path(f"outputs/{year}")
out_dir.mkdir(parents=True, exist_ok=True)
out_file = out_dir / f"day{day}-part{part}.txt"

in_dir = pathlib.Path(f"inputs/{year}")
in_dir.mkdir(parents=True, exist_ok=True)
in_file = in_dir / f"day{day}.txt"

shell("cargo build --release", stdout=sp.DEVNULL, stderr=sp.DEVNULL)
answer = shell(
    f"target/release/aoc {year} {day} {part} <{in_file}",
    stdout=sp.PIPE,
    stderr=None,
).stdout

print(answer)
shell("xsel -ib", input=answer)

if to_save:
    with out_file.open("w") as file:
        file.write(answer)
else:
    if out_file.exists():
        process = shell(f"diff {out_file} - || true", input=answer)
