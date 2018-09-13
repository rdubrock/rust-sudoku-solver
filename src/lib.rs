#![feature(exclusive_range_pattern)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// pub fn solve_string(puzzle_string: &str) -> Vec<u32> {
//     println!("Starting to solve: {}", puzzle_string);
//     let mut puzzle = build_puzzle(puzzle_string);
//     solve_sudoku(&mut puzzle)
// }

struct Square {
    value: Option<u32>,
    possibles: Vec<u32>,
}

impl Square {
    fn new(value: Option<u32>) -> Square {
        Square {
            value: value,
            possibles: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }
}

fn build_puzzle(puzzle_string: &str) -> Vec<Square> {
    let mut grid = Vec::new();
    for character in puzzle_string.chars() {
        let digit = character.to_digit(10);
        let grid_cell = Square::new(digit);
        grid.push(grid_cell);
    }
    return grid;
}

fn build_possibles(value: Option<u32>) -> Vec<u32> {
    if let Some(_) = value {
        return Vec::new();
    } else {
        return (1..10).collect::<Vec<u32>>();
    }
}

fn destruct_puzzle(grid: &Vec<Square>) -> Vec<u32> {
    let mut solved_puzzle = Vec::new();
    for square in grid {
        if let Some(value) = square.value {
            solved_puzzle.push(value);
        }
    }
    return solved_puzzle;
}

// fn solve_sudoku(grid: &mut Vec<Square>) -> Vec<u32> {
//     // let mut puzzle_solved = false;
//     loop {
//         puzzle_loop(grid);
//         //  if puzzle_solved { break }
//     }
//     return destruct_puzzle(&grid);
// }

fn trim_possibles(possibles: Vec<u32>, value: u32) -> Vec<u32> {
    possibles
        .iter()
        .filter(|&&x| x != value)
        .cloned()
        .collect::<Vec<u32>>()
}

fn get_row_by_index(index: u32) -> u32 {
    match index {
        0..9 => 0,
        9..18 => 1,
        18..27 => 2,
        27..36 => 3,
        36..45 => 4,
        45..54 => 5,
        54..63 => 6,
        63..72 => 7,
        72..81 => 8,
        _ => panic!("Could not match index {} to row", index),
    }
}

// fn check_column(square: Square, grid: &Vec<Square>, location: usize) {
//    for check_square in grid.iter().step_by(9) {
// if let Some(value) = check_square.value {
// square.possibles = trim_possibles(square.possibles, value);
// }
// }
// }

// fn puzzle_loop(grid: &Vec<Square>) {
//     for (i, square) in grid.iter().enumerate() {
//         check_column(&grid, i)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_possibles() {
        let possibles = (1..10).collect::<Vec<u32>>();
        let value = 7;
        let newPossibles = trim_possibles(possibles, value);
        let expectedPossibles = [1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(newPossibles, expectedPossibles)
    }

    #[test]
    fn test_get_row_by_index() {
        let row1 = get_row_by_index(3);
        let row5 = get_row_by_index(45);
        assert_eq!((row1, row5), (0, 5))
    }
}
