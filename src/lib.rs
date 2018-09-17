#![feature(exclusive_range_pattern)]
extern crate wasm_bindgen;
use std::ops::Range;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve_string(puzzle_string: &str) -> String {
    let puzzle = &mut build_puzzle(puzzle_string);
    let solved = solve_sudoku(puzzle);
    return solved;
}

#[derive(Clone, Debug)]
struct Square {
    value: Option<u32>,
    possibles: Vec<u32>,
    index: usize,
    row: Range<usize>,
    column: [usize; 9],
    box_group: [usize; 9],
}

impl Square {
    fn new(value: Option<u32>, index: usize) -> Square {
        Square {
            value: value,
            possibles: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            index,
            row: get_row_by_index(index),
            column: get_column_by_index(index),
            box_group: get_box_by_index(index),
        }
    }

    fn update_value(&mut self) -> bool {
        if let Some(_) = self.value {
            self.possibles = vec![];
            return false;
        } else if self.possibles.len() == 1 {
            self.value = self.possibles.pop();
            println!(
                "Updating value at index {} with value {:#?}",
                self.index, self.value
            );
            return true;
        } else {
            return false;
        }
    }

    fn trim_possibles(&mut self, value: u32) {
        self.possibles = self
            .possibles
            .iter()
            .filter(|&&x| x != value)
            .cloned()
            .collect::<Vec<u32>>();
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

fn destruct_puzzle(grid: &Vec<Square>) -> String {
    let mut solved_puzzle: String = String::new();
    for square in grid {
        if let Some(value) = square.value {
            solved_puzzle.push_str(&value.to_string());
        } else {
            solved_puzzle.push_str(".");
        }
    }
    return solved_puzzle;
}

fn solve_sudoku(grid: &mut Vec<Square>) -> String {
    let guesses: &mut Vec<Vec<Square>> = &mut vec![];
    let mut current_grid = grid.clone();
    loop {
        solve_iteration(&mut current_grid);
        let solve_status = check_solved_state(&current_grid);
        match solve_status {
            SudokuState::Solved => {
                println!("SOLVED");
                break;
            }
            SudokuState::NeedsGuess => {
                println!("Needs guess");
                let applied_guesses = &mut apply_guess(&current_grid);
                println!("applied guesses {}", &applied_guesses.len());
                guesses.append(applied_guesses);
                if let Some(new_guess) = guesses.pop() {
                    current_grid = new_guess.clone();
                    continue;
                }
                break;
            }
            SudokuState::InProgress => {
                println!("In progress!");
                continue;
            }
            SudokuState::Invalid => {
                println!("INVALID");
                if let Some(new_guess) = guesses.pop() {
                    current_grid = new_guess.clone();
                    continue;
                }
                break;
            }
        }
    }
    return destruct_puzzle(&current_grid);
}

fn apply_guess(grid: &Vec<Square>) -> Vec<Vec<Square>> {
    let mut min_possibles = 9;
    let mut guesses_applied = vec![];
    for square in grid {
        let possibles = &square.possibles.len();
        if possibles > &0 && possibles < &min_possibles {
            min_possibles = possibles.clone();
        }
    }

    println!("MIN POSSIBLES {}", min_possibles);

    for square in grid {
        let possibles_size = square.possibles.len();
        if possibles_size == min_possibles {
            for possible in &square.possibles {
                let grid_with_guess = &mut grid.clone();
                let guessed = Square {
                    value: Some(possible.clone()),
                    possibles: vec![],
                    index: square.index,
                    row: get_row_by_index(square.index),
                    column: get_column_by_index(square.index),
                    box_group: get_box_by_index(square.index),
                };
                grid_with_guess[square.index] = guessed;
                guesses_applied.push(grid_with_guess.clone());
            }
            break;
        }
    }
    for guess in &guesses_applied {
        let destructed = destruct_puzzle(&guess);
        print_puzzle(destructed);
    }
    return guesses_applied;
}

fn solve_iteration(grid: &mut Vec<Square>) {
    let grid_clone = grid.clone();
    let mut value_updated = false;
    for (i, _square) in grid_clone.iter().enumerate() {
        let grid_to_pass = grid.clone();
        if i == 17 {
            println!("INDEX 17 {:#?}", &grid[i].possibles)
        }
        check_column(&mut grid[i], &grid_to_pass);
        if i == 17 {
            println!("INDEX 17 {:#?}", &grid[i].possibles)
        }
        check_row(&mut grid[i], &grid_to_pass);
        if i == 17 {
            println!("INDEX 17 {:#?}", &grid[i].possibles)
        }
        check_box(&mut grid[i], &grid_to_pass);
        if i == 17 {
            println!("INDEX 17 {:#?}", &grid[i].possibles)
        }
        let update = grid[i].update_value();
        if update {
            value_updated = true;
        }
    }
    if value_updated {
        solve_iteration(grid);
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

//  0,  1,  2,   3,  4,  5,   6,  7,  8,
//  9, 10, 11,  12, 13, 14,  15, 16, 17,
// 18, 19, 20,  21, 22, 23,  24, 25, 26,
//
// 27, 28, 29,  30, 31, 32,  33, 34, 35,
// 36, 37, 38,  39, 40, 41,  42, 43, 44,
// 45, 46, 47,  48, 49, 50,  51, 52, 53,
//
// 54, 55, 56,  57, 58, 59,  60, 61, 62,
// 63, 64, 65,  66, 67, 68,  69, 70, 71,
// 72, 73, 74,  75, 76, 77,  78, 79, 80

fn get_box_by_index(index: usize) -> [usize; 9] {
    let int = index as u32;
    let row = int / 9;
    let column = index % 9;
    match (row, column) {
        (0..3, 0..3) => [0, 1, 2, 9, 10, 11, 18, 19, 20],
        (0..3, 3..6) => [3, 4, 5, 12, 13, 14, 21, 22, 23],
        (0..3, 6..9) => [6, 7, 8, 15, 16, 17, 24, 25, 26],
        (3..6, 0..3) => [27, 28, 29, 36, 37, 38, 45, 46, 47],
        (3..6, 3..6) => [30, 31, 32, 39, 40, 41, 48, 49, 50],
        (3..6, 6..9) => [33, 34, 35, 42, 43, 44, 51, 52, 53],
        (6..9, 0..3) => [54, 55, 56, 63, 64, 65, 72, 73, 74],
        (6..9, 3..6) => [57, 58, 59, 66, 67, 68, 75, 76, 77],
        (6..9, 6..9) => [60, 61, 62, 69, 70, 71, 78, 79, 80],
        _ => panic!("could not find box for row {} column {}", row, column),
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

fn check_box(square: &mut Square, grid: &Vec<Square>) {
    for &i in square.box_group.clone().iter() {
        let square_to_check: &Square = &grid[i];
        if let Some(value) = square_to_check.value {
            square.trim_possibles(value)
        }
    }
}

fn check_solved_state(grid: &Vec<Square>) -> SudokuState {
    let mut guess_required = true;
    let mut solved = true;
    for (i, square) in grid.iter().enumerate() {
        if square.value == None {
            solved = false;
            if square.possibles.len() == 0 {
                println!("invalid AT Index {}, {:#?}", i, &grid[i]);
                return SudokuState::Invalid;
            } else if square.possibles.len() == 1 {
                guess_required = false;
            }
        }
    }
    if solved {
        print_puzzle(destruct_puzzle(grid));
        return SudokuState::Solved;
    } else if guess_required {
        return SudokuState::NeedsGuess;
    } else {
        return SudokuState::InProgress;
    }
}

fn print_puzzle(puzzle_string: String) {
    let divider = " --- --- --- --- --- --- --- --- --- \n";
    let mut output: String = String::new();
    output.push_str(divider);
    for (i, value) in puzzle_string.chars().enumerate() {
        output.push_str("| ");
        output.push_str(&value.to_string());
        output.push_str(" ");
        if i % 9 == 8 {
            output.push_str(" |");
            output.push_str("\n");
            output.push_str(divider);
        }
    }
    println!("{}", output);
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
    fn test_check_box() {
        let test_string =
            "643859721581762493927413856254937168196.48.37378..1.49469865312735126984812394675";
        let grid = &mut build_puzzle(test_string);
        let test_square: &mut Square = &mut grid[49].clone();
        check_box(test_square, grid);
        assert_eq!(test_square.possibles, [2, 5, 6]);
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
    fn test_solve_iteration() {
        let test_string =
            ".389...5775..84.1...65.748..6..9574.519.43..8.7.6..3.5941.3.5..3.54..1.9...159.34";
        let after_one =
            ".389...57752.84.1...65.748..6..9574.519.43..8.7.6..3.5941.3.5..3.54..1.9...159.34";
        let after_two =
            ".389...57752384.1.1965.748..63.9574.519.43..8.746..3.5941.3.5..3.54..1.9..7159.34";
        let grid = &mut build_puzzle(test_string);
        solve_iteration(grid);
        let output = destruct_puzzle(&grid);
        assert_eq!(output, after_one);
        solve_iteration(grid);
        let output_2 = destruct_puzzle(&grid);
        assert_eq!(output_2, after_two);
    }

    #[test]
    fn test_solve_simple_sudoku() {
        let test_string =
            ".389...5775..84.1...65.748..6..9574.519.43..8.7.6..3.5941.3.5..3.54..1.9...159.34";
        let solved_string =
            "438961257752384916196527483863295741519743628274618395941836572385472169627159834";
        let solved = solve_string(test_string);
        assert_eq!(solved, solved_string)
    }

    #[test]
    fn test_solve_medium_sudoku() {
        let test_string =
            "6.3..972.5..7....3.2.4..8.6.5.9..1681.......7378..1.4.469..5.1.735..6..48123..6.5";
        let solved = solve_string(test_string);
        let solved_string =
            "643589721581762493927413856254937168196248537378651249469875312735126984812394675";
        assert_eq!(solved, solved_string);
    }

    #[test]
    fn test_hard_sudoku() {
        let test_string =
            "54..6..318.741..2.....3.....542..1.............2..185.....9.....9..452.767..2..95";
        let solved = solve_string(test_string);
        let valid_solved =
            "549862731837419526216537948354286179781954362962371854425798613193645287678123495";
        assert_eq!(solved, valid_solved);
    }
    #[test]
    fn test_really_hard_sudoku() {
        let test_string =
            "1...9...2735..........4..5..5.....63.....61...8.1.....8..5.72....4.....1...2..73.";
        let solved = solve_string(test_string);
        let valid_solved =
            "148395672735682914692741358957828463423876189486139527819517246274963891519214736";
        assert_eq!(solved, valid_solved);
    }

    #[test]
    fn test_really_hard_sudoku_2() {
        let test_string =
            "7...1...........4.29.....57..98.61..6.........28.4.....3.7....8.82.5.......12.3..";
        let solved = solve_string(test_string);
        print_puzzle(solved.clone());
        let valid_solved =
            "746915832853672941291483657579836124614297583328541769935764218182359476467128395";
        assert_eq!(solved, valid_solved);
    }
}
