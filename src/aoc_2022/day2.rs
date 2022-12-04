use crate::std_iter;

pub fn part1() {
    let score: u64 = std_iter!(Lines).map(|line| {
        let opponent = line.as_bytes()[0] - b'A';
        let mine = line.as_bytes()[2] - b'X';
        let outcome = (mine + 4 - opponent) % 3;        
        (mine + 1 + outcome * 3) as u64
    }).sum();
    println!("{}", score);
}

pub fn part2() {
    let score: u64 = std_iter!(Lines).map(|line| {
        let opponent = line.as_bytes()[0] - b'A';
        let outcome = line.as_bytes()[2] - b'X';
        let mine = (opponent + outcome + 2) % 3;
        (mine + 1 + outcome * 3) as u64
    }).sum();
    println!("{}", score);
}