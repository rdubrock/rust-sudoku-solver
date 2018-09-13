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
}

impl Square {
    fn new(value: Option<u32>, index: usize) -> Square {
        Square {
            value: value,
            possibles: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            index,
        }
    }
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

// fn solve_sudoku(grid: &mut Vec<Square>) -> Vec<u32> {
//     // let mut puzzle_solved = false;
//     loop {
//         puzzle_loop(grid);
//         //  if puzzle_solved { break }
//     }
//     return destruct_puzzle(&grid);
// }

fn trim_possibles(possibles: &Vec<u32>, value: u32) -> Vec<u32> {
    possibles
        .iter()
        .filter(|&&x| x != value)
        .cloned()
        .collect::<Vec<u32>>()
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

fn check_row<'a>(square: &'a mut Square, grid: &mut Vec<Square>) -> &'a mut Square {
    let row = get_row_by_index(square.index);
    for i in row {
        let square_to_check: &Square = &grid[i];
        if let Some(value) = square_to_check.value {
            let new_possibles = trim_possibles(&square.possibles, value);
            square.possibles = new_possibles
        }
    }
    return square;
}

fn check_column<'a>(square: &'a mut Square, grid: &mut Vec<Square>) -> &'a mut Square {
    let column = get_column_by_index(square.index);
    for &i in column.iter() {
        let square_to_check: &Square = &grid[i];
        if let Some(value) = square_to_check.value {
            let new_possibles = trim_possibles(&square.possibles, value);
            square.possibles = new_possibles
        }
    }
    return square;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_possibles() {
        let possibles = (1..10).collect::<Vec<u32>>();
        let value = 7;
        let newPossibles = trim_possibles(&possibles, value);
        let expectedPossibles = [1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(newPossibles, expectedPossibles)
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
        let new_square = check_row(test_square, grid);
        assert_eq!(new_square.possibles, [1, 3, 5, 6, 7])
    }

    #[test]
    fn test_check_column() {
        let test_string =
            "...28.94.1.4...7......156.....8..57.4.......8.68..9.....196......5...8.3.43.28...";
        let grid = &mut build_puzzle(test_string);
        let test_square: &mut Square = &mut grid[2].clone();
        let new_square = check_column(test_square, grid);
        assert_eq!(new_square.possibles, [2, 6, 7, 9])
    }
}
