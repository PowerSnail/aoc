use crate::prelude::*;

struct Report {
    sensor: (i64, i64), 
    beacon: (i64, i64)
}

impl Report {
    fn distance(&self) -> i64 {
        (self.beacon.0 - self.sensor.0).abs() + (self.beacon.1 - self.sensor.1).abs()
    }
}

fn parse_line(i: &str) -> ParserResult<Report> {
    let (i, x) = preceded(tag("Sensor at x="), parse_i64)(i)?;
    let (i, y) = preceded(tag(", y="), parse_i64)(i)?;
    let (i, x_) = preceded(tag(": closest beacon is at x="), parse_i64)(i)?;
    let (i, y_) = preceded(tag(", y="), parse_i64)(i)?;

    Ok((i, Report { sensor: (x, y), beacon: (x_, y_)}))
}

fn parse_input(i: &str) -> ParseResult<Vec<Report>> {
    separated_list1(tag("\n"), parse_line)(i)
}

// const TARGET_ROW: i64 = 10;
const TARGET_ROW: i64 = 2_000_000;

// const CONSTRAINT: i64 = 20;
const CONSTRAINT: i64 = 4_000_000;

fn merge_ranges(mut ranges: impl Iterator<Item=(i64, i64)>) -> Vec<(i64, i64)> {
    let mut merged_dead_zones = Vec::new();
    if let Some(first) = ranges.next() {
        merged_dead_zones.push(first);
    }

    for (lo, hi)  in ranges {
        let (lo_prev, hi_prev) = merged_dead_zones.pop().unwrap();
        if lo <= hi_prev {
            merged_dead_zones.push((lo_prev, hi.max(hi_prev)))
        } else {
            merged_dead_zones.push((lo_prev, hi_prev));
            merged_dead_zones.push((lo, hi));
        }
    }
    merged_dead_zones
}

fn dead_zone(report: &Report, row: i64) -> Option<(i64, i64)> {
    let distance = report.distance();
    let horizontal_max_delta = distance - (report.sensor.1 - row).abs();
    if horizontal_max_delta < 0 {
        None
    } else {
        let range = (report.sensor.0 - horizontal_max_delta, report.sensor.0 + horizontal_max_delta + 1);
        Some(range)
    }
}

pub fn part1() {
    let reports = parse_input(stdio_string().as_str()).unwrap().1;

    let dead_zones = reports.iter().filter_map(|r| dead_zone(r, TARGET_ROW)).sorted();
    let merged_dead_zones = merge_ranges(dead_zones);

    let existing = reports.iter().filter(|r| r.beacon.1 == TARGET_ROW).map(|r| r.beacon.0).collect::<HashSet::<_>>().len();
    let total: i64 = merged_dead_zones.into_iter().map(|(lo, hi)| hi - lo).sum();
    println!("{}", total as usize - existing);
}

pub fn part2() {
    let reports = parse_input(stdio_string().as_str()).unwrap().1;
    for y in 0..=CONSTRAINT {
        let dead_zones = reports.iter().filter_map(|r| dead_zone(r, y))
            .map(|(lo, hi)| (lo.clamp(0, CONSTRAINT), hi.clamp(0, CONSTRAINT)))
            .sorted();
        let merged_dead_zones = merge_ranges(dead_zones);

        if merged_dead_zones.len() == 1 {
            continue;
        }
        let x = merged_dead_zones[0].1;
        println!("{}", x * 4000000 + y);
        return;
    }
    unreachable!();
}