// https://adventofcode.com/2025/day/4
// Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

use std::thread::sleep;
use std::time::Duration;
use aoc2025::{clrscr, read_input};
const ROLLS: char = '@';
const OFFSET: usize = 4;
fn main() {
    clrscr(Some(4));

    let input = read_input(4);

    // ██████╗  █████╗ ██████╗ ████████╗     ██╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ███║
    // ██████╔╝███████║██████╔╝   ██║       ╚██║
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║        ██║
    // ██║     ██║  ██║██║  ██║   ██║        ██║
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝        ╚═╝
    let mut accessible_rolls: usize = 0;
    let mut map_matrix: Vec<Vec<char>> = Vec::new();
    let mut cloned_map_matrix: Vec<Vec<char>> = Vec::new();

    for rolls_line in input.lines() {
        let rolls = rolls_line.chars().collect::<Vec<char>>();
        map_matrix.push(rolls);
    }
    cloned_map_matrix = map_matrix.clone();

    for (row_index, rolls_row) in map_matrix.clone().iter().enumerate() {
        for (col_index, roll_col) in rolls_row.iter().enumerate() {
            if *roll_col == ROLLS {
                if !has_more_then_four_rolls_adjacents(&mut map_matrix, row_index, col_index) {
                    cloned_map_matrix[row_index][col_index] = 'x';
                    accessible_rolls+=1;
                }
            }
        }
    }

    println!("#Part 1) How many rolls of paper can be accessed by a forklift? → {}", accessible_rolls);

    // ██████╗  █████╗ ██████╗ ████████╗    ██████╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ╚════██╗
    // ██████╔╝███████║██████╔╝   ██║        █████╔╝
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║       ██╔═══╝
    // ██║     ██║  ██║██║  ██║   ██║       ███████╗
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝       ╚══════╝
    accessible_rolls = 0;
    let mut some_roll_was_removed: bool = true;
    let mut is_accessible: bool = true;

    while some_roll_was_removed {
        some_roll_was_removed = false;

        for (row_index, rolls_row) in map_matrix.clone().iter().enumerate() {
            for (col_index, roll_col) in rolls_row.iter().enumerate() {
                if *roll_col == ROLLS {
                    is_accessible = !has_more_then_four_rolls_adjacents(&mut map_matrix, row_index, col_index);
                    some_roll_was_removed = some_roll_was_removed || is_accessible;
                    if is_accessible {
                        map_matrix[row_index][col_index] = 'x';
                        accessible_rolls+=1;
                    }
                }
            }
        }
    }

    println!("#Part 2) How many rolls of paper in total can be removed by the Elves and their forklifts? {}", accessible_rolls);
}

fn render_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        println!("{}", row.iter().collect::<String>());
    }
}

fn has_more_then_four_rolls_adjacents(matrix: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut adjacents: Vec<char> = Vec::new();

    // Top-Left | Center | Right
    if row > 0 {
        if col > 0 {
            adjacents.push(*matrix.get(row-1).unwrap_or(&Vec::new()).get(col-1).unwrap_or(&'-'));
        }
        adjacents.push(*matrix.get(row-1).unwrap_or(&Vec::new()).get(col).unwrap_or(&'-'));
        adjacents.push(*matrix.get(row-1).unwrap_or(&Vec::new()).get(col+1).unwrap_or(&'-'));
    }

    // Left-Right
    if col > 0 {
        adjacents.push(*matrix.get(row).unwrap_or(&Vec::new()).get(col - 1).unwrap_or(&'-'));
    }
    adjacents.push(*matrix.get(row).unwrap_or(&Vec::new()).get(col+1).unwrap_or(&'-'));

    // Bottom-Left | Center | Right
    if col > 0 {
        adjacents.push(*matrix.get(row + 1).unwrap_or(&Vec::new()).get(col - 1).unwrap_or(&'-'));
    }
    adjacents.push(*matrix.get(row+1).unwrap_or(&Vec::new()).get(col).unwrap_or(&'-'));
    adjacents.push(*matrix.get(row+1).unwrap_or(&Vec::new()).get(col+1).unwrap_or(&'-'));


    adjacents.iter().filter(|c| **c == ROLLS).count() >= 4
}