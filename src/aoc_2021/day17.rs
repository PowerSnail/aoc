use crate::{prelude::*, std_iter};

fn parse_target_area(s: &str) -> IResult<&str, (i64, i64, i64, i64)> {
    let (s, (x1, x2, y1, y2)) = tuple((
        preceded(tag("target area: x="), parse_i64),
        preceded(tag(".."), parse_i64),
        preceded(tag(", y="), parse_i64),
        preceded(tag(".."), parse_i64),
    ))(s)?;
    Ok((s, (x1, x2, y1, y2)))
}

pub fn part1() {
    let (_, _, y1, y2) = std_iter!(Lines)
        .map(|l| parse_target_area(&l).unwrap().1)
        .next()
        .unwrap();
    let vy_max = if y1 > 0 {
        y1.max(y2) + 1
    } else {
        -y1.min(y2) - 1
    };
    let y_max = (vy_max + 1) * vy_max / 2;
    println!("{}", y_max);
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<Vec<f64>> {
    let determinant = b * b - 4.0 * a * c;
    if determinant >= 0.0 {
        let d_root = determinant.sqrt();
        Some(vec![(-b + d_root) / 2. / a, (-b - d_root) / 2. / a])
    } else {
        eprintln!("No solution for: a={}, b={}, c={}", a, b, c);
        None
    }
}

pub fn part2() {
    let (x1, x2, y1, y2) = std_iter!(Lines)
        .map(|l| parse_target_area(&l).unwrap().1)
        .next()
        .unwrap();

    let vy_min = if y2 > 0 {
        solve_quadratic(1., 1., -2.0 * y1 as f64).unwrap()[0].ceil() as i64
    } else {
        y1
    };
    let vy_max = if y1 > 0 { y2 + 1 } else { -y1 - 1 };
    let vx_min = solve_quadratic(1., 1., -2. * x1 as f64).unwrap()[0].ceil() as i64;
    let vx_max = x2;

    let count = iproduct!(vx_min..=vx_max, vy_min..=vy_max)
        .filter(|&(vx, vy)| {
            let mut probe = (0, 0, vx, vy);
            loop {
                if (probe.3 <= 0 && probe.1 < y1) || probe.0 > x2 {
                    return false;
                }
                if x1 <= probe.0 && probe.0 <= x2 && y1 <= probe.1 && probe.1 <= y2 {
                    return true;
                }
                let dvx = if probe.2 == 0 {
                    0
                } else {
                    probe.2 / probe.2.abs()
                };
                probe = (
                    probe.0 + probe.2,
                    probe.1 + probe.3,
                    probe.2 - dvx,
                    probe.3 - 1,
                );
            }
        })
        .count();

    println!("{}", count);
}
