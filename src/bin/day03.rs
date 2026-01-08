// https://adventofcode.com/2025/day/3
// There are many batteries in front of you. Find the maximum joltage possible from each bank; what is the total output joltage?

extern crate core;

use std::collections::HashSet;
use aoc2025::{clrscr, read_input};

fn main() {
    clrscr(Some(3));

    let input = read_input(3, None);

    let mut banks_max_joltages: Vec<usize> = Vec::new();


    // ██████╗  █████╗ ██████╗ ████████╗     ██╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ███║
    // ██████╔╝███████║██████╔╝   ██║       ╚██║
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║        ██║
    // ██║     ██║  ██║██║  ██║   ██║        ██║
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝        ╚═╝
    for battery_bank in input.lines() {
        let combinations = banks_cartesian_product(battery_bank.to_string());
        let max = combinations.iter().max().unwrap();
        banks_max_joltages.push(*max);
    }

    let sum: usize = banks_max_joltages.iter().sum();
    println!("#Part 1) Total output joltage: {sum}");

    // ██████╗  █████╗ ██████╗ ████████╗    ██████╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ╚════██╗
    // ██████╔╝███████║██████╔╝   ██║        █████╔╝
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║       ██╔═══╝
    // ██║     ██║  ██║██║  ██║   ██║       ███████╗
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝       ╚══════╝
    let mut total: u128 = 0;

    for line in input.lines() {
        let max_s = best_subsequence_of_k(line, 12);
        let value: u128 = max_s.parse().unwrap();
        total += value;

        println!("{} → {}", line, max_s);
    }

    println!("#Part 2) Total: {}", total);
}

fn banks_cartesian_product(battery_bank: String) -> HashSet<usize> {
    let to_numerical_digits: Vec<usize> = battery_bank.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect();
    let mut cartesian_product: Vec<usize> = Vec::new();

    for (i, outer_digit) in to_numerical_digits.iter().enumerate() {
        for (j, inner_digit) in to_numerical_digits.iter().enumerate() {
            if j > i {
                cartesian_product.push((*outer_digit*10)+(*inner_digit)); // es: [8, 9] = (8*10)+9 = 89
            }
        }
    }

    cartesian_product.iter().map(|item| *item).collect::<HashSet<usize>>()
}

// Algorithm gently offered by ChatGPT 😭 I'm not happy about that.
// I spent a lot of time trying to solve it... without any results.
// It uses greedy algorithm: https://it.wikipedia.org/wiki/Algoritmo_greedy
fn best_subsequence_of_k(line: &str, k: usize) -> String {
    let digits: Vec<u8> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut stack: Vec<u8> = Vec::with_capacity(k);
    let mut to_remove = digits.len() - k;

    for &d in &digits {
        while let Some(&last) = stack.last() {
            if to_remove > 0 && last < d {
                stack.pop();
                to_remove -= 1;
            } else {
                break;
            }
        }
        stack.push(d);
    }


    stack.truncate(k);

    stack.iter().map(|d| char::from(b'0' + *d)).collect()
}