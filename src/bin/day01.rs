/// https://adventofcode.com/2025/day/1
///
/// Analyze the rotations in your attached document. What's the actual password to open the door?

use aoc2025::{clrscr, read_input};
use aoc2025::Direction;
use aoc2025::CircularLinkedList;

const LIST_LENGTH: u8 = 100;
const START_FROM: u8 = 50;

fn main () {
    clrscr(Some(1));

    let input = read_input(1);
    let mut list = CircularLinkedList::new(LIST_LENGTH);
    let mut number = 0;
    let mut zero_count = 0;
    let mut hits_zero_times = 0;

    // Set dial to START_FROM
    list.move_forward(START_FROM as u32, &mut hits_zero_times);

    for instruction in input.lines() {
        let direction = Direction::from(instruction);
        number = instruction[1..].parse::<u32>().unwrap();

        match direction {
            Direction::Left => list.move_backward(number, &mut hits_zero_times),
            Direction::Right => list.move_forward(number, &mut hits_zero_times),
        };

        if list.get_head() == 0 {
            zero_count += 1;
        }
    }

    println!("#Part 1) Zero count: {zero_count}");
    println!("#Part 2) How many times passed through 0 (zero) number: {hits_zero_times}");
}
