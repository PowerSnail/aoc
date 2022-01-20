use crate::{prelude::*, std_iter};

fn has_bingo(board: &[u32]) -> bool {
    if board
        .iter()
        .chunks(5)
        .into_iter()
        .any(|mut row| row.all(|n| *n == 0))
    {
        return true;
    }
    if (0usize..5).any(|c| (0..5).all(|r| board[r * 5 + c] == 0)) {
        return true;
    }
    return false;
}

pub fn part1() {
    let mut lines = std_iter!(Lines);
    let num_sequence = lines
        .next()
        .unwrap()
        .split(",")
        .map(|string| string.parse::<u32>().unwrap())
        .collect_vec();
    let mut boards: Vec<u32> = lines
        .map(|l| {
            l.split(" ")
                .filter_map(|l| l.parse::<u32>().ok())
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let board_count = boards.len() / 25;
    for n in num_sequence.into_iter() {
        for x in boards.iter_mut() {
            if *x == n {
                *x = 0;
            }
        }

        for board in 0..board_count {
            if has_bingo(&boards[board * 25..(board + 1) * 25]) {
                println!(
                    "{}",
                    boards[board * 25..(board + 1) * 25].iter().sum::<u32>() * n
                );
                return;
            }
        }
    }
}

pub fn part2() {
    let mut lines = std_iter!(Lines);
    let num_sequence = lines
        .next()
        .unwrap()
        .split(",")
        .map(|string| string.parse::<u32>().unwrap())
        .collect_vec();
    let mut boards: Vec<u32> = lines
        .map(|l| {
            l.split(" ")
                .filter_map(|l| l.parse::<u32>().ok())
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let board_count = boards.len() / 25;
    let mut remaining_boards = (0..board_count).collect_vec();

    for n in num_sequence.into_iter() {
        for x in boards.iter_mut() {
            if *x == n {
                *x = 0;
            }
        }

        let boards_left = remaining_boards
            .iter()
            .map(|board| *board)
            .filter(|board| !has_bingo(&boards[board * 25..(board + 1) * 25]))
            .collect_vec();

        if boards_left.len() == 0 {
            let board = remaining_boards[0];
            println!(
                "{}",
                boards[board * 25..(board + 1) * 25].iter().sum::<u32>() * n
            );
            return;
        }

        remaining_boards = boards_left;
    }
}
