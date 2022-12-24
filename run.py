import pathlib
import sqlite3
import time
import typer
import subprocess as sp
import functools
import dotenv
import requests
import os


shell = functools.partial(sp.run, shell=True, check=True, text=True)


def init_db(path: str):
    db = sqlite3.connect(path)
    db.execute(
        f"""
        CREATE TABLE IF NOT EXISTS QA (
            year INTEGER,
            day INTEGER,
            part INTEGER,
            question TEXT,
            answer TEXT
        )"""
    )
    return db


def question_answers(db: sqlite3.Connection, year: int, day: int, part: int):
    for row in db.execute(
        f"""
        SELECT
            question,
            answer
        FROM
            QA
        WHERE
            year = ?
            AND day = ?
            AND part = ?
        ORDER BY
            question""",
        (year, day, part),
    ).fetchall():
        yield (row[0], row[1])


def run(year: int, day: int, part: int, input: str):
    tik = time.time()
    result = shell(
        f"target/release/aoc {year} {day} {part}",
        input=input,
        encoding="ascii",
        stdout=sp.PIPE,
        stderr=None,
    )
    tok = time.time()
    return (tok - tik, result.stdout.rstrip("\n"))


app = typer.Typer()

@app.command()
def add_test(year: int, day: int, part: int, question: str, answer: str):
    db = init_db("tests/qa.sqlite")
    db.execute(
        f"""
        INSERT INTO QA (year, day, part, question, answer)
        VALUES (?, ?, ?, ?, ?)
        """,
        (year, day, part, question, answer),
    )
    db.commit()


@app.command()
def download_input(year: int, day: int):
    db = init_db("inputs/qa.sqlite")
    if db.execute("SELECT COUNT(*) FROM QA WHERE year=? AND day=?;", (year, day)).fetchone()[0] != 0:
        print("Already downloaded")
        return

    dotenv.load_dotenv()
    aoc_session = os.environ.get("AOC_SESSION")
    response = requests.get(f"https://adventofcode.com/{year}/day/{day}/input", cookies={"session": aoc_session})
    response.raise_for_status()

    db.execute(f"""
        INSERT INTO QA (year, day, part, question, answer)
        VALUES (?, ?, ?, ?, NULL)
        """, (year, day, 1, response.text))
    db.execute(f"""
        INSERT INTO QA (year, day, part, question, answer)
        VALUES (?, ?, ?, ?, NULL)
        """, (year, day, 2, response.text))
    db.commit()


@app.command()
def test(year: int, day: int, part: int):
    print("Compiling")
    shell("cargo build --release", stdout=sp.DEVNULL, stderr=sp.DEVNULL)

    db = init_db("tests/qa.sqlite")    
    for i, (question, answer) in enumerate(question_answers(db, year, day, part)):
        print(f"Running test {i}")
        try:
            result = run(year, day, part, question)
        except sp.CalledProcessError as e:
            ...
            print(f"Test failed. Process exit with nonzero return code")
            break
    
        if result[1] != answer:
            print(f"Test failed. Expecting {answer!r} but got {result[1]!r}")
            break
    else:
        print("Test passed")


@app.command()
def real(year: int, day: int, part: int, save: bool = False):
    print("Compiling")
    shell("cargo build --release", stdout=sp.DEVNULL, stderr=sp.DEVNULL)
    db = init_db("inputs/qa.sqlite")    
    for i, (question, answer) in enumerate(question_answers(db, year, day, part)):
        print(f"Running program {i}")
        try:
            result = run(year, day, part, question)
        except sp.CalledProcessError as e:
            ...
            print(f"Failed. Process exit with nonzero return code")
            break
        
        if answer != None and result[1] != answer:
            print(f"Answer changed. Expecting {answer!r} but got {result[1]!r}")
            break
    
        print(result[1])
        print("Took {:.2f}ms".format(result[0] * 1000))
        shell("xsel -ib", input=result[1])

        if save:
            db.execute(
                f"""
                UPDATE QA 
                SET 
                    answer = ? 
                WHERE 
                    year = ? 
                    AND day = ? 
                    AND part = ? 
                    AND question = ?
                """,
                (result[1], year, day, part, question),
            )
            db.commit()


if __name__ == "__main__":
    app()