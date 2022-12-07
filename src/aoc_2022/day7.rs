use std::collections::HashMap;

use crate::prelude::*;
use crate::std_iter;

enum Line<'a> {
    Cd(&'a str),
    Ls,
    Dir(&'a str),
    File(u64, &'a str),
}

fn parse_cd(i: &str) -> IResult<&str, Line> {
    let (i, r) = preceded(tag("$ cd "), take_while(|_| true))(i)?;
    Ok((i, Line::Cd(r)))
}

fn parse_ls(i: &str) -> IResult<&str, Line> {
    let (i, _) = tag("$ ls")(i)?;
    Ok((i, Line::Ls))
}

fn parse_dir(i: &str) -> IResult<&str, Line> {
    let (i, name) = preceded(tag("dir "), take_while(|_| true))(i)?;
    Ok((i, Line::Dir(name)))
}

fn parse_file(i: &str) -> IResult<&str, Line> {
    let (i, (size, name)) = tuple((parse_u64, take_while(|_| true)))(i)?;
    Ok((i, Line::File(size, name)))
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((parse_cd, parse_ls, parse_dir, parse_file))(i)
}

fn count(
    name: &str,
    sizes: &mut HashMap<String, u64>,
    children: &HashMap<String, Vec<String>>,
) -> u64 {
    if !sizes.contains_key(name) {
        let size: u64 = children[name]
            .iter()
            .map(|name| count(name, sizes, children))
            .sum();
        sizes.insert(name.to_string(), size);
    }
    return sizes[name];
}

fn make_fs() -> HashMap<String, u64> {
    let mut sizes: HashMap<String, u64> = HashMap::new();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_path = vec!["/"];

    let lines = std_iter!(Lines).collect_vec();

    for line in lines.iter() {
        let cwd = current_path.join("");
        match parse_line(&line).unwrap().1 {
            Line::Cd("..") => {
                current_path.pop();
                current_path.pop();
            }
            Line::Cd("/") => {
                current_path.truncate(1);
            }
            Line::Cd(name) => {
                current_path.push(name);
                current_path.push("/");
            }
            Line::Ls => {
                children.insert(cwd, vec![]);
            }
            Line::Dir(name) => {
                let child_name = cwd.clone() + &name + "/";
                children.get_mut(&cwd).unwrap().push(child_name);
            }
            Line::File(size, name) => {
                let child_name = cwd.clone() + &name;
                children.get_mut(&cwd).unwrap().push(child_name.clone());
                sizes.insert(child_name, size);
            }
        }
    }

    count("/", &mut sizes, &children);
    sizes
}

pub fn part1() {
    let total: u64 = make_fs()
        .iter()
        .filter(|&(name, count)| name.ends_with("/") && *count <= 100_000)
        .map(|v| v.1)
        .sum();
    println!("{}", total);
}

pub fn part2() {
    let sizes = make_fs();
    let spare = 70_000_000 - sizes["/"];
    let needed = 30_000_000 - spare;
    eprintln!("Need {}", needed);

    let to_delete = sizes
        .iter()
        .filter(|&(key, size)| key.ends_with("/") && *size >= needed)
        .map(|v| v.1)
        .min()
        .unwrap();
    println!("{}", to_delete);
}
