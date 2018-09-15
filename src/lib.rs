#![feature(exclusive_range_pattern)]
extern crate wasm_bindgen;
use std::ops::Range;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// pub fn solve_string(puzzle_string: &str) -> Vec<u32> {
//     println!("Starting to solve: {}", puzzle_string);
//     let mut puzzle = build_puzzle(puzzle_string);
//     solve_sudoku(&mut puzzle)
// }
#[derive(Clone)]
struct Square {
    value: Option<u32>,
    possibles: Vec<u32>,
    index: usize,
    row: Range<usize>,
    column: [usize; 9],
}

impl Square {
    fn new(value: Option<u32>, index: usize) -> Square {
        Square {
            value: value,
            possibles: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            index,
            row: get_row_by_index(index),
            column: get_column_by_index(index),
        }
    }

    fn update_value(&mut self, value: u32) {
        self.value = Some(value)
    }

    fn trim_possibles(&mut self, value: u32) {
        self.possibles = self
            .possibles
            .iter()
            .filter(|&&x| x != value)
            .cloned()
            .collect::<Vec<u32>>()
    }
}

#[derive(Debug, PartialEq)]
enum SudokuState {
    Solved,
    InProgress,
    Invalid,
    NeedsGuess,
}

fn build_puzzle(puzzle_string: &str) -> Vec<Square> {
    let mut grid = Vec::new();
    for (i, character) in puzzle_string.chars().enumerate() {
        let digit = character.to_digit(10);
        let grid_cell = Square::new(digit, i);
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

fn solve_sudoku(grid: &mut Vec<Square>) -> Vec<u32> {
    loop {
        solve_iteration(grid);
        let solve_status = check_solved_state(&grid);
        match solve_status {
            SudokuState::Solved => {
                println!("SOLVED");
                break;
            }
            SudokuState::NeedsGuess => {
                println!("Needs guess");
                break;
            }
            SudokuState::InProgress => {
                println!("In progress!");
                break;
            }
            SudokuState::Invalid => {
                println!("INVALID");
                break;
            }
        }
    }
    return destruct_puzzle(&grid);
}

fn solve_iteration(grid: &mut Vec<Square>) {
    let grid_clone = grid.clone();
    for (i, _square) in grid_clone.iter().enumerate() {
        check_column(&mut grid[i], &grid_clone);
        check_row(&mut grid[i], &grid_clone);
    }
}

fn get_row_by_index(index: usize) -> Range<usize> {
    match index {
        0..9 => 0..9,
        9..18 => 9..18,
        18..27 => 18..27,
        27..36 => 27..36,
        36..45 => 36..45,
        45..54 => 45..54,
        54..63 => 54..63,
        63..72 => 63..72,
        72..81 => 72..81,
        _ => panic!("Could not match index {} to row", index),
    }
}

fn get_column_by_index(index: usize) -> [usize; 9] {
    match index % 9 {
        0 => [0, 9, 18, 27, 36, 45, 54, 63, 72],
        1 => [1, 10, 19, 28, 37, 46, 55, 64, 73],
        2 => [2, 11, 20, 29, 38, 47, 56, 65, 74],
        3 => [3, 12, 21, 30, 39, 48, 57, 66, 75],
        4 => [4, 13, 22, 31, 40, 49, 58, 67, 76],
        5 => [5, 14, 23, 32, 41, 50, 59, 68, 77],
        6 => [6, 15, 24, 33, 42, 51, 60, 69, 78],
        7 => [7, 16, 25, 34, 43, 52, 61, 70, 79],
        8 => [8, 17, 26, 35, 44, 53, 62, 71, 80],
        _ => panic!("Could not find column for index {}", index),
    }
}

fn check_row(square: &mut Square, grid: &Vec<Square>) {
    for i in square.row.clone() {
        let square_to_check: &Square = &grid[i];
        if let Some(value) = square_to_check.value {
            square.trim_possibles(value);
        }
    }
}

fn check_column(square: &mut Square, grid: &Vec<Square>) {
    for &i in square.column.clone().iter() {
        let square_to_check: &Square = &grid[i];
        if let Some(value) = square_to_check.value {
            square.trim_possibles(value)
        }
    }
}

fn check_solved_state(grid: &Vec<Square>) -> SudokuState {
    let mut guess_required = true;
    let mut solved = true;
    for square in grid {
        if square.value == None {
            solved = false;
            if square.possibles.len() == 0 {
                return SudokuState::Invalid;
            } else if square.possibles.len() == 1 {
                guess_required = false;
            }
        }
    }
    if solved {
        return SudokuState::Solved;
    } else if guess_required {
        return SudokuState::NeedsGuess;
    } else {
        return SudokuState::InProgress;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_possibles() {
        let square = &mut Square::new(None, 3);
        square.trim_possibles(7);
        let expected_possibles = [1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(square.possibles, expected_possibles)
    }

    #[test]
    fn test_get_row_by_index() {
        let row1 = get_row_by_index(3);
        let row5 = get_row_by_index(45);
        assert_eq!((row1, row5), (0..9, 45..54))
    }

    #[test]
    fn test_build_grid() {
        let test_string =
            "...28.94.1.4...7......156.....8..57.4.......8.68..9.....196......5...8.3.43.28...";
        let grid = build_puzzle(test_string);
        assert_eq!(grid[0].value, None);
        assert_eq!(grid[0].index, 0);
        assert_eq!(grid[0].possibles, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(grid[3].value, Some(2));
        assert_eq!(grid[3].index, 3);
        assert_eq!(grid[3].possibles, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_check_row() {
        let test_string =
            "...28.94.1.4...7......156.....8..57.4.......8.68..9.....196......5...8.3.43.28...";
        let grid = &mut build_puzzle(test_string);
        let test_square: &mut Square = &mut grid[2].clone();
        check_row(test_square, grid);
        assert_eq!(test_square.possibles, [1, 3, 5, 6, 7])
    }

    #[test]
    fn test_check_column() {
        let test_string =
            "...28.94.1.4...7......156.....8..57.4.......8.68..9.....196......5...8.3.43.28...";
        let grid = &mut build_puzzle(test_string);
        let test_square: &mut Square = &mut grid[2].clone();
        check_column(test_square, grid);
        assert_eq!(test_square.possibles, [2, 6, 7, 9])
    }

    #[test]
    fn test_check_solved_state() {
        let test_string =
            "...28.94.1.4...7......156.....8..57.4.......8.68..9.....196......5...8.3.43.28...";
        let grid = build_puzzle(test_string);
        let solved_state = check_solved_state(&grid);
        // Without ever checking for possibles, this should lead to every None value having multiple options
        assert_eq!(solved_state, SudokuState::NeedsGuess)
    }

    #[test]
    fn test_solve_sudoku() {
        let test_string =
            "...28.94.1.4...7......156.....8..57.4.......8.68..9.....196......5...8.3.43.28...";
        let grid = build_puzzle(test_string);
        let solved = solve_sudoku(&mut grid.clone());
        assert_eq!(solved, [1, 2, 3]);
    }
}
