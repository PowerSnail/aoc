use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0")]
#[derive(Debug)]
struct Opts {
    year: u32,
    day: usize,
    part: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    match (opts.year, opts.day, opts.part) {
        (2015, day, part) => aoc::aoc_2015::SOLUTIONS[day - 1][part - 1](),
        (2021, day, part) => aoc::aoc_2021::SOLUTIONS[day - 1][part - 1](),
        (2022, day, part) => aoc::aoc_2022::SOLUTIONS[day - 1][part - 1](),
        _ => eprintln!("Error: Unknown options {:?}", opts),
    }
}
