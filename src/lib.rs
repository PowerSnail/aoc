#![feature(stdin_forwarders)]
pub mod aoc_2015;
pub mod aoc_2021;
pub mod parsers;

#[macro_use]
pub mod prelude;

pub const FUNCTIONS_2015: [[fn() -> (); 2]; 25] = [
    [aoc_2015::day1_part1, aoc_2015::day1_part2],
    [aoc_2015::day2_part1, aoc_2015::day2_part2],
    [aoc_2015::day3_part1, aoc_2015::day3_part2],
    [aoc_2015::day4_part1, aoc_2015::day4_part2],
    [aoc_2015::day5_part1, aoc_2015::day5_part2],
    [aoc_2015::day6_part1, aoc_2015::day6_part2],
    [aoc_2015::day7_part1, aoc_2015::day7_part2],
    [aoc_2015::day8_part1, aoc_2015::day8_part2],
    [aoc_2015::day9_part1, aoc_2015::day9_part2],
    [aoc_2015::day10_part1, aoc_2015::day10_part2],
    [aoc_2015::day11_part1, aoc_2015::day11_part2],
    [aoc_2015::day12_part1, aoc_2015::day12_part2],
    [aoc_2015::day13_part1, aoc_2015::day13_part2],
    [aoc_2015::day14_part1, aoc_2015::day14_part2],
    [aoc_2015::day15_part1, aoc_2015::day15_part2],
    [aoc_2015::day16_part1, aoc_2015::day16_part2],
    [aoc_2015::day17_part1, aoc_2015::day17_part2],
    [aoc_2015::day18_part1, aoc_2015::day18_part2],
    [aoc_2015::day19_part1, aoc_2015::day19_part2],
    [aoc_2015::day20_part1, aoc_2015::day20_part2],
    [aoc_2015::day21_part1, aoc_2015::day21_part2],
    [aoc_2015::day22_part1, aoc_2015::day22_part2],
    [aoc_2015::day23_part1, aoc_2015::day23_part2],
    [aoc_2015::day24_part1, aoc_2015::day24_part2],
    [aoc_2015::day25_part1, aoc_2015::day25_part2],
];

pub const FUNCTIONS_2021: [[fn() -> (); 2]; 25] = [
    [aoc_2021::day1_part1, aoc_2021::day1_part2],
    [aoc_2021::day2_part1, aoc_2021::day2_part2],
    [aoc_2021::day3_part1, aoc_2021::day3_part2],
    [aoc_2021::day4_part1, aoc_2021::day4_part2],
    [aoc_2021::day5_part1, aoc_2021::day5_part2],
    [aoc_2021::day6_part1, aoc_2021::day6_part2],
    [aoc_2021::day7_part1, aoc_2021::day7_part2],
    [aoc_2021::day8_part1, aoc_2021::day8_part2],
    [aoc_2021::day9_part1, aoc_2021::day9_part2],
    [aoc_2021::day10_part1, aoc_2021::day10_part2],
    [aoc_2021::day11_part1, aoc_2021::day11_part2],
    [aoc_2021::day12_part1, aoc_2021::day12_part2],
    [aoc_2021::day13_part1, aoc_2021::day13_part2],
    [aoc_2021::day14_part1, aoc_2021::day14_part2],
    [aoc_2021::day15_part1, aoc_2021::day15_part2],
    [aoc_2021::day16_part1, aoc_2021::day16_part2],
    [aoc_2021::day17_part1, aoc_2021::day17_part2],
    [aoc_2021::day18_part1, aoc_2021::day18_part2],
    [aoc_2021::day19_part1, aoc_2021::day19_part2],
    [aoc_2021::day20_part1, aoc_2021::day20_part2],
    [aoc_2021::day21_part1, aoc_2021::day21_part2],
    [aoc_2021::day22_part1, aoc_2021::day22_part2],
    [aoc_2021::day23_part1, aoc_2021::day23_part2],
    [aoc_2021::day24_part1, aoc_2021::day24_part2],
    [aoc_2021::day25_part1, aoc_2021::day25_part2],
];
