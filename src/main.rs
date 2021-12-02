use clap::Parser;
use aoc::{aoc_2015, aoc_2021};

#[derive(Parser)]
#[clap(version = "1.0")]
#[derive(Debug)]
struct Opts {
    year: u32,
    day: u32,
    part: u32,
}

fn main() {
    let opts: Opts = Opts::parse();
    let input = std::io::stdin();

    match (opts.year, opts.day, opts.part) {
        (2015, 1, 1) =>aoc_2015::day1_part1(input),
        (2015, 1, 2) =>aoc_2015::day1_part2(input),
        (2015, 2, 1) =>aoc_2015::day2_part1(input),
        (2015, 2, 2) =>aoc_2015::day2_part2(input),
        (2015, 3, 1) =>aoc_2015::day3_part1(input),
        (2015, 3, 2) =>aoc_2015::day3_part2(input),
        (2015, 4, 1) =>aoc_2015::day4_part1(input),
        (2015, 4, 2) =>aoc_2015::day4_part2(input),
        (2015, 5, 1) =>aoc_2015::day5_part1(input),
        (2015, 5, 2) =>aoc_2015::day5_part2(input),
        (2015, 6, 1) =>aoc_2015::day6_part1(input),
        (2015, 6, 2) =>aoc_2015::day6_part2(input),
        (2015, 7, 1) =>aoc_2015::day7_part1(input),
        (2015, 7, 2) =>aoc_2015::day7_part2(input),
        (2015, 8, 1) =>aoc_2015::day8_part1(input),
        (2015, 8, 2) =>aoc_2015::day8_part2(input),
        (2015, 9, 1) =>aoc_2015::day9_part1(input),
        (2015, 9, 2) =>aoc_2015::day9_part2(input),
        (2015, 10, 1) =>aoc_2015::day10_part1(input),
        (2015, 10, 2) =>aoc_2015::day10_part2(input),
        (2015, 11, 1) =>aoc_2015::day11_part1(input),
        (2015, 11, 2) =>aoc_2015::day11_part2(input),
        (2015, 12, 1) =>aoc_2015::day12_part1(input),
        (2015, 12, 2) =>aoc_2015::day12_part2(input),
        (2015, 13, 1) =>aoc_2015::day13_part1(input),
        (2015, 13, 2) =>aoc_2015::day13_part2(input),
        (2015, 14, 1) =>aoc_2015::day14_part1(input),
        (2015, 14, 2) =>aoc_2015::day14_part2(input),
        (2015, 15, 1) =>aoc_2015::day15_part1(input),
        (2015, 15, 2) =>aoc_2015::day15_part2(input),
        (2015, 16, 1) =>aoc_2015::day16_part1(input),
        (2015, 16, 2) =>aoc_2015::day16_part2(input),
        (2015, 17, 1) =>aoc_2015::day17_part1(input),
        (2015, 17, 2) =>aoc_2015::day17_part2(input),
        (2015, 18, 1) =>aoc_2015::day18_part1(input),
        (2015, 18, 2) =>aoc_2015::day18_part2(input),
        (2015, 19, 1) =>aoc_2015::day19_part1(input),
        (2015, 19, 2) =>aoc_2015::day19_part2(input),
        (2015, 20, 1) =>aoc_2015::day20_part1(input),
        (2015, 20, 2) =>aoc_2015::day20_part2(input),
        (2015, 21, 1) =>aoc_2015::day21_part1(input),
        (2015, 21, 2) =>aoc_2015::day21_part2(input),
        (2015, 22, 1) =>aoc_2015::day22_part1(input),
        (2015, 22, 2) =>aoc_2015::day22_part2(input),
        (2015, 23, 1) =>aoc_2015::day23_part1(input),
        (2015, 23, 2) =>aoc_2015::day23_part2(input),
        (2015, 24, 1) =>aoc_2015::day24_part1(input),
        (2015, 24, 2) =>aoc_2015::day24_part2(input),
        (2015, 25, 1) =>aoc_2015::day25_part1(input),
        (2015, 25, 2) =>aoc_2015::day25_part2(input),

        (2021, 1, 1) =>aoc_2021::day1_part1(input),
        (2021, 1, 2) =>aoc_2021::day1_part1(input),
        (2021, 2, 1) =>aoc_2021::day2_part1(input),
        (2021, 2, 2) =>aoc_2021::day2_part2(input),
        _ => eprintln!("Error: Unknown options {:?}", opts),
    }
}
