// https://adventofcode.com/2025/day/6
// Solve the problems on the math worksheet. What is the grand total found by adding together all of the answers to the individual problems?

use std::arch::x86_64::_mulx_u32;
use std::fs::{read_to_string, DirBuilder};
use aoc2025::read_input;

fn main() {
    let input = read_input(6, Some(false));

    // ██████╗  █████╗ ██████╗ ████████╗     ██╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ███║
    // ██████╔╝███████║██████╔╝   ██║       ╚██║
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║        ██║
    // ██║     ██║  ██║██║  ██║   ██║        ██║
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝        ╚═╝
    let operations: Vec<char> = input
        .trim()
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|c| c
            .to_string()
            .parse::<char>()
            .unwrap()
        ).collect();

    let original_matrix = input
        .lines()
        .collect::<Vec<&str>>().as_slice()[..input.lines().count() - 1]
        .to_vec()
        .iter()
        .map(|line| line
            .split_whitespace()
            .map(|c| c.to_string().parse::<i128>().unwrap())
            .collect::<Vec<i128>>()
        ).collect::<Vec<Vec<i128>>>();

    let max_col_len = original_matrix
        .iter()
        .map(|c| c.len())
        .max()
        .unwrap();

    let mut matrix_cols_rows_swapped: Vec<Vec<i128>> = Vec::new();

    for c in 0..max_col_len {
        let mut swapped_row: Vec<i128>= Vec::new();
        for row in &original_matrix {
            if let Some(value) = row.get(c) {
                swapped_row.push(*value);
            }
        }
        matrix_cols_rows_swapped.push(swapped_row);
    }

    let mut grand_total: i128 = 0;
    for (row_index, row) in matrix_cols_rows_swapped.iter().enumerate() {
        grand_total += match operations[row_index] {
            '+' => row.iter().sum(),
            '*' => row.iter().product(),
            _  => 0,
        };
    }
    println!("#Part 1) What is the grand total found by adding together all of the answers to the individual problems? {grand_total}");

    // ██████╗  █████╗ ██████╗ ████████╗    ██████╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ╚════██╗
    // ██████╔╝███████║██████╔╝   ██║        █████╔╝
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║       ██╔═══╝
    // ██║     ██║  ██║██║  ██║   ██║       ███████╗
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝       ╚══════╝

    enum Operations {
        Add,
        Mul
    }
    impl From<&str> for Operations {
        fn from(value: &str) -> Self {
            match value {
                "*" => { Operations::Mul },
                "+" => { Operations::Add },
                _ => { Operations::Add }
            }
        }
    }


    let mut last_signs_row = input.lines().last().unwrap();
    let signs_splitted_whitespaces = last_signs_row.split(['*', '+']);
    let signs_splitted = last_signs_row.split_inclusive(['*', '+']);
    let mut signs_splitted_array_whitespaces: Vec<&str> = signs_splitted_whitespaces.clone().collect();
    signs_splitted_array_whitespaces.remove(0);


    let input_lines = input
        .trim()
        .lines()
        .collect::<Vec<&str>>().as_slice()[..input.lines().count() - 1]
        .to_vec();

    let mut grand_total = 0;

    let mut greatest_number_len: usize = 0;
    let mut col_offset = 0;
    let mut numbers: Vec<i128> = Vec::new();
    let input_lines: Vec<&str> = input_lines[0..input_lines.len()-1].to_vec();

    for (sign_index, sign_str) in signs_splitted.map(|sign| sign.trim()).enumerate() {
        if sign_index == signs_splitted_array_whitespaces.len()-1 {
            greatest_number_len = &input_lines.last().unwrap().len() - last_signs_row.len() + 1;
        } else {
            greatest_number_len = signs_splitted_array_whitespaces[sign_index].len();
        }


        let mut temp_number: String = "".into();
        let sign = sign_str.trim();

        for col in col_offset..col_offset+greatest_number_len {
            for line in &input_lines {
                if col == line.len() {
                    continue;
                }
                let char = &line[col..=col];
                if !char.is_empty() && char != " " {
                    temp_number.push(line[col..=col].chars().collect::<Vec<char>>()[0]);
                }
            }
            if temp_number.is_empty() {
                temp_number = match Operations::from(sign) {
                    Operations::Mul => "1".to_string(),
                    Operations::Add => "0".to_string()
                };
            }

            numbers.push(temp_number.parse::<i128>().unwrap());

            temp_number.clear();

            if numbers.len() == greatest_number_len {
                grand_total += match Operations::from(sign) {
                    Operations::Mul => numbers.iter().product::<i128>(),
                    Operations::Add => numbers.iter().sum::<i128>()
                };

                numbers.clear();

                col_offset += greatest_number_len+1;
            }
        }
    }

    println!("#Part 2) What is the grand total found by adding together all of the answers to the individual problems? {grand_total}");
}

