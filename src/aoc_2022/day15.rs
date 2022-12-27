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

pub fn part1() {
    let reports = parse_input(stdio_string().as_str()).unwrap().1;
    let mut dead_zone : HashSet<i64> = HashSet::new();
    let mut existing: HashSet<i64> = HashSet::new();
    for report in reports {
        let distance = report.distance();
        let horizontal_max_delta = distance - (report.sensor.1 - TARGET_ROW).abs();
        if horizontal_max_delta >= 0 {
            dead_zone.extend(report.sensor.0 - horizontal_max_delta..=report.sensor.0 + horizontal_max_delta);
        }

        if report.beacon.1 == TARGET_ROW {
            existing.insert(report.beacon.0);
        }
    }
    let dead_zone = dead_zone.difference(&existing);
    println!("{}", dead_zone.count());
}

pub fn part2() {
    todo!();
}