// https://adventofcode.com/2025/day/5
// Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?

use std::collections::HashSet;

use std::ops::{RangeInclusive};
use range_ext::intersect::Intersect;
use aoc2025::{clrscr, read_input};

fn main() {
    clrscr(Some(5));
    let input = read_input(5);

    // ██████╗  █████╗ ██████╗ ████████╗     ██╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ███║
    // ██████╔╝███████║██████╔╝   ██║       ╚██║
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║        ██║
    // ██║     ██║  ██║██║  ██║   ██║        ██║
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝        ╚═╝
    let input_lines_vec = input.lines().collect::<Vec<&str>>();
    let separator_position: usize = input.lines().position(|line| line.is_empty()).unwrap();
    let fresh_ids_map= input_lines_vec.clone().as_slice()[..separator_position].to_vec();
    let ids_list: Vec<&str> =  input_lines_vec.as_slice()[separator_position+1..].to_vec();
    let mut fresh_available_ids: usize = 0;

    // Flatten fresh IDs list.
    let mut fresh_ids_flatten: Vec<(RangeInclusive<usize>)> = Vec::new();

    for range in &fresh_ids_map {
        let left_right_values = range.split("-").collect::<Vec<&str>>();
        let left  = left_right_values[0].parse::<usize>().unwrap();
        let right = left_right_values[1].parse::<usize>().unwrap();


        fresh_ids_flatten.push((left..=right));
    }

    for id in ids_list.iter().map(|id| id.parse::<usize>().unwrap()) {
        for fresh_ids_range in &fresh_ids_flatten {
            if fresh_ids_range.contains(&id) {
                fresh_available_ids += 1;
                break;
            }
        }
    }
    println!("#Part 1) How many of the available ingredient IDs are fresh? {}", fresh_available_ids);

    // ██████╗  █████╗ ██████╗ ████████╗    ██████╗
    // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ╚════██╗
    // ██████╔╝███████║██████╔╝   ██║        █████╔╝
    // ██╔═══╝ ██╔══██║██╔══██╗   ██║       ██╔═══╝
    // ██║     ██║  ██║██║  ██║   ██║       ███████╗
    // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝       ╚══════╝
    // To resolve part 2 use the following algorithm:
    // 1. Loop through all ranges (sorted by start values)
    // 2. Check for current range if the start of the other ones is contained in it: curr_range.contains(inner_loop_range_item.start)
    // 3. If it's true, and if the inner_loop_range_item.end is biggest then curr_range.end, set curr_range.end = inner_loop_range_item.end
    // 4. Now count all end-start and get the total sum.
    fresh_ids_flatten.sort_by(|r_curr, r_next| r_curr.start().cmp(r_next.start()));
    let mut merged_ranges = recursive_merging(&mut fresh_ids_flatten);
    let mut merged_ranges = merged_ranges.iter().collect::<HashSet<_>>();

    println!("Merged IDs: {:#?}", merged_ranges);
    let total = merged_ranges.iter().map(|range| range.end()+1-range.start()).sum::<usize>();

    println!("#Part 2) How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges? {}", total);
}
 
/// Recursively merges overlapping or adjacent ranges in the input vector.
/// 
/// This function takes a mutable reference to a vector of ranges and returns a new vector
/// where all overlapping or adjacent ranges have been merged into single, continuous ranges.
/// The merging is done in-place where possible, and the function uses recursion to handle
/// multiple levels of merging that might be needed after each merge operation.
/// 
/// # Arguments
/// * `fresh_ids_ranges` - A mutable reference to a vector of ranges to be merged
/// 
/// # Returns
/// A new vector containing the merged ranges
fn recursive_merging(fresh_ids_ranges: &mut Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let clone = fresh_ids_ranges.clone();

    for (item_idx, range_item) in clone.iter().enumerate() {
        for (inner_idx, range_inner) in clone.iter().enumerate() {
            if can_be_merged(range_item, range_inner) {
                fresh_ids_ranges[item_idx] = merge_ranges(range_item, range_inner);
                fresh_ids_ranges.remove(inner_idx);
                return recursive_merging(fresh_ids_ranges);
            } else {
                // If true, simply remove range_inner, because it's full contained by range_item
                if range_item.contains(range_inner.start()) && range_inner.end() < range_item.end() {
                    fresh_ids_ranges.remove(inner_idx);
                    return recursive_merging(fresh_ids_ranges);
                }
            }
        }

    }

    fresh_ids_ranges.clone()
}

fn can_be_merged(range_x: &RangeInclusive<usize>, range_y: &RangeInclusive<usize>) -> bool {
    if range_x.eq(range_y) {
        false
    } else if range_x.contains(range_y.start()) && range_x.end() <= range_y.end() {
        true
    } else {
        false
    }
}

fn merge_ranges(range_x: &RangeInclusive<usize>, range_y: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    *range_x.start()..=*range_y.end()
}