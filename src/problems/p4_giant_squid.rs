use std::collections::HashSet;
use crate::input::p4_giant_squid::*;

#[derive(Debug)]
pub struct Board {
    board: Vec<i32>,
    called_indices: HashSet<usize>
}

impl Board {
    fn winning_ranges() -> Vec<HashSet<usize>> {
        vec![
            (0..5).collect(),
            (5..10).collect(),
            (10..15).collect(),
            (15..20).collect(),
            (20..25).collect(),

            HashSet::from([1, 6, 11, 16, 21]),
            HashSet::from([2, 7, 12, 17, 22]),
            HashSet::from([3, 8, 13, 18, 23]),
            HashSet::from([4, 9, 14, 19, 24]),

            HashSet::from([0, 6, 12, 18, 24]),
            HashSet::from([5, 9, 13, 17, 21]),
        ]
    }

    pub fn new(board: Vec<i32>) -> Board {
        Board {
            board,
            called_indices: HashSet::new()
        }
    }

    pub fn call(&mut self, num: i32) {
        let has_num = self.board.iter().position(|i| i.eq(&&num));
        if has_num.is_some() {
            self.called_indices.insert(has_num.unwrap());
        }
    }

    pub fn wins(&self) -> bool {
        Board::winning_ranges().iter()
            .any(|range| {
                let intersect: HashSet<_> = self.called_indices.intersection(&range).collect();
                intersect.len() == 5
            })
    }

    pub fn score(&self) -> i32 {
        (0..self.board.len()).fold(0, |tot, i| {
            if !self.called_indices.contains(&i) {
                tot + self.board.get(i).unwrap()
            } else {
                tot
            }
        })
    }
}

fn winning_board_score() -> i32 {
    let numbers = numbers();
    let mut boards = boards();

    for number in numbers {
        for board in boards.iter_mut() {
            board.call(number)
        }

        let winning_board = boards.iter().find(|b| b.wins());
        if winning_board.is_some() {
            return winning_board.unwrap().score() * number
        }
    }

    0
}

fn last_winning_board_score() -> i32 {
    let numbers = numbers();
    let mut boards = boards();

    let mut boards_won: HashSet<usize> = HashSet::new();
    let mut board_scores: Vec<i32> = Vec::new();

    for number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if !boards_won.contains(&i) {
                board.call(number);
                if board.wins() {
                    board_scores.push(board.score() * number);
                    boards_won.insert(i);
                }
            }
        }
    }

    board_scores.last().unwrap().to_owned()
}

pub fn main() {
    let winning_score = winning_board_score();
    let last_winning_score = last_winning_board_score();

    println!("Problem 4:");
    println!("Winning board score: {}", winning_score);
    println!("Last winning board score: {}", last_winning_score);
}
