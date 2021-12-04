use std::collections::HashSet;

pub(crate) fn solve(input: &[String]) {
    let draws: Vec<u32> = input
        .iter()
        .take(1)
        .flat_map(|s| s.split(','))
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<i32>>> = vec![vec![]];

    for line in input.iter().skip(2) {
        if line.is_empty() {
            boards.last_mut().unwrap().push(vec![0; 6]);
            boards.push(vec![]);
            continue;
        }
        boards.last_mut().unwrap().push({
            let mut row: Vec<_> = line
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            row.push(0);
            row
        });
    }
    boards.last_mut().unwrap().push(vec![0; 6]);

    let (first_board, first_draw, last_board, last_draw) = get_winning_board(&draws, &mut boards);

    let calculate_score = |board: &Vec<Vec<i32>>, draw| {
        board
            .iter()
            .take(5)
            .flat_map(|row| row.iter().take(5))
            .filter(|&&x| x != -1)
            .sum::<i32>() as u32
            * draw
    };

    let score: u32 = calculate_score(&first_board, first_draw);
    println!("Part 1: {}", score);
    let score: u32 = calculate_score(&last_board, last_draw);
    println!("Part 2: {}", score);
}

fn get_winning_board(
    draws: &[u32],
    boards: &mut Vec<Vec<Vec<i32>>>,
) -> (Vec<Vec<i32>>, u32, Vec<Vec<i32>>, u32) {
    let mut first_winning_draw = 0;
    let mut first_winning_board = vec![];
    let last_winning_draw;
    let last_winning_board;
    let num_boards = boards.len();
    let mut won_boards = HashSet::with_capacity(boards.len());

    for &draw in draws {
        for board_idx in 0..boards.len() {
            for row_idx in 0..boards[0].len() - 1 {
                for col_idx in 0..boards[0][0].len() - 1 {
                    if boards[board_idx][row_idx][col_idx] == draw as i32 {
                        boards[board_idx][row_idx][col_idx] = -1;
                        boards[board_idx][row_idx][5] += 1;
                        boards[board_idx][5][col_idx] += 1;
                        if boards[board_idx][row_idx][5] == 5 || boards[board_idx][5][col_idx] == 5
                        {
                            won_boards.insert(board_idx);
                            if won_boards.len() == 1 {
                                first_winning_board = boards[board_idx].clone();
                                first_winning_draw = draw;
                            }
                            if won_boards.len() == num_boards {
                                last_winning_board = boards[board_idx].clone();
                                last_winning_draw = draw;
                                return (
                                    first_winning_board,
                                    first_winning_draw,
                                    last_winning_board,
                                    last_winning_draw,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    unreachable!("All boards must eventually win")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day4() {
        let input: Vec<_> = include_str!("../inputs/day4_sample.txt")
            .lines()
            .map(|s| s.to_string())
            .collect();
        solve(&input);
    }
}
