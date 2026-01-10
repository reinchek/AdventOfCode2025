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

    let input = read_input(1, None);

    println!("#Part 1) Zero count: {}", part_1(&input).0);
    println!("#Part 2) How many times passed through 0 (zero) number: {}", part_2(&input));
}

// ██████╗  █████╗ ██████╗ ████████╗     ██╗
// ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ███║
// ██████╔╝███████║██████╔╝   ██║       ╚██║
// ██╔═══╝ ██╔══██║██╔══██╗   ██║        ██║
// ██║     ██║  ██║██║  ██║   ██║        ██║
// ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝        ╚═╝
fn part_1(input: &String) -> (usize, u32) {
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

    (zero_count, hits_zero_times)
}

// ██████╗  █████╗ ██████╗ ████████╗    ██████╗
// ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ╚════██╗
// ██████╔╝███████║██████╔╝   ██║        █████╔╝
// ██╔═══╝ ██╔══██║██╔══██╗   ██║       ██╔═══╝
// ██║     ██║  ██║██║  ██║   ██║       ███████╗
// ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝       ╚══════╝
fn part_2(input: &String) -> u32 {
    part_1(input).1
}


#[test]
fn part1_test_zero_count() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
    assert_eq!(part_1(&input).0, 3);
}

#[test]
fn part2_how_many_times_passed_through_0() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
    assert_eq!(part_2(&input), 6);
}