// https://adventofcode.com/2025/day/6
// Solve the problems on the math worksheet. What is the grand total found by adding together all of the answers to the individual problems?

use aoc2025::read_input;

fn main() {
    let input = read_input(6);
    let input = "123 328  51 64
45 64  387 23
6 98  215 314
*   +   *   +";

    // ██████╗  █████╗ ██████╗ ████████╗     ██╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ███║
    // ██████╔╝███████║██████╔╝   ██║       ╚██║
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║        ██║
    // ██║     ██║  ██║██║  ██║   ██║        ██║
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝        ╚═╝
    let operations: Vec<char> = input
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
    println!("original_matrix: {:?}", original_matrix);
    println!("matrix_cols_rows_swapped: {matrix_cols_rows_swapped:?}");

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
    let mut rtl_columns: Vec<Vec<isize>> = Vec::new();

    // for row in matrix_cols_rows_swapped {
    //     for col in row {
    //         let num2str = col.to_string().chars().rev().collect::<String>().parse::<i128>().unwrap();
    //     }
    // }
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

    struct Numbers {
        numbers: Vec<Vec<i128>>,
        operation: Operations,
    }


    // |1|2|3|3|2|8|5|1| |6|4| |
    // |4|5| |6|4| |3|8|7|2|3| |
    // |6| | |9|8| |2|1|5|3|1|4|
    // |*| | |+| | |*| | |+| | |
    let mut last_signs_row = input.lines().last().unwrap();
    let signs_splitted = last_signs_row.split_inclusive(['*', '+']);
    let signs_splitted_array: Vec<&str> = signs_splitted.clone().collect();
    let input_lines = input
        .lines()
        .collect::<Vec<&str>>().as_slice()[..input.lines().count() - 1]
        .to_vec();

    let mut grand_total = 0;

    for (sign_index, sign_str) in signs_splitted.enumerate() {
        let greatest_number_len: usize;

        if sign_index == 0 {
            greatest_number_len = signs_splitted_array[sign_index+1].len()-1;
        } else {
            greatest_number_len = sign_str.len()-1;
        }


        for row in 0..input_lines.iter().count() {
            for col in &input_lines[0..greatest_number_len] {
                dbg!(col);
            }
        }

    }
    //
    //
    // // 123 328  51   64
    // // 45  64   387  23
    // // 6   98   215  314
    // // *   +   *   +
    //
    // // 123 45  6
    // // 328 64  98
    // // 51  387 215
    // // 64  23  314    [6][4][]|[2][3][]|[3][1][4]
    //
    // let mut matrix_of_cells: Vec<Vec<Vec<i128>>> = vec!();
    //
    // for row in matrix_cols_rows_swapped {
    //     let mut row_with_cells_digits: Vec<Vec<i128>> = vec!();
    //
    //     for col in row {
    //         let mut num_to_cell: Vec<i128> = vec!();
    //         // ex: 64  => vec![6, 4]
    //         //     23  => vec![2, 3]
    //         //     314 => vec![3, 1, 4]
    //         for digit in col.to_string().chars() {
    //             let converted_digit = digit.clone().to_digit(10).unwrap() as i128;
    //             &num_to_cell.push(converted_digit);
    //         }
    //         row_with_cells_digits.push(num_to_cell);
    //     }
    //     println!("row => {:?}", row_with_cells_digits);
    //     matrix_of_cells.push(row_with_cells_digits);
    // }
    // println!("full => {:?}", matrix_of_cells);
    //
    // // println!("rtl_columns -> {rtl_columns:?}");

}

