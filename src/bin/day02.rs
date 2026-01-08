/// https://adventofcode.com/2025/day/2
///
/// Since the young Elf was just doing silly patterns, you can find the invalid IDs
/// by looking for any ID which is made only of some sequence of digits repeated twice.
/// So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
///
/// None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

use aoc2025::{clrscr, read_input};

fn main() {
    clrscr(Some(2));

    let input = read_input(2, None).trim().to_string();
    // let input = String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");

    let mut part1_collected_numbers: Vec<usize> = Vec::new();
    let mut part2_collected_numbers: Vec<usize> = Vec::new();

    for ranges in input.split(",") {
        let (start, end) = ranges.split_once("-").unwrap();

        // Check that start/end doesn't start with "0" number, cause invalid.
        if start.starts_with("0") || end.starts_with("0") {
            continue;
        }

        let start = start.parse::<usize>().unwrap();
        let end   = end  .parse::<usize>().unwrap();

        for number in start..=end {
            let str_number = number.to_string();
            let str_len = str_number.len();

            // ██████╗  █████╗ ██████╗ ████████╗     ██╗
            // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ███║
            // ██████╔╝███████║██████╔╝   ██║       ╚██║
            // ██╔═══╝ ██╔══██║██╔══██╗   ██║        ██║
            // ██║     ██║  ██║██║  ██║   ██║        ██║
            // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝        ╚═╝
            let (lstr, rstr) = (str_number[..str_len/2].to_string(), str_number[str_len/2..].to_string());
            if str_len % 2 == 0 {
                if lstr == rstr {
                    part1_collected_numbers.push(number);
                }
            }

            // ██████╗  █████╗ ██████╗ ████████╗    ██████╗
            // ██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝    ╚════██╗
            // ██████╔╝███████║██████╔╝   ██║        █████╔╝
            // ██╔═══╝ ██╔══██║██╔══██╗   ██║       ██╔═══╝
            // ██║     ██║  ██║██║  ██║   ██║       ███████╗
            // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝       ╚══════╝
            let mut divisors: Vec<usize> = Vec::new();

            // Check divisors for str_len
            for i in 1..=str_len {
                if str_len % i == 0 {
                    divisors.push(i);
                }
            }

            // If none of divisors match to the given rule,
            // check if the number is made of equal digits.
            let str_number_bytes = str_number.as_bytes();
            for divisor in divisors {
                let str_chunks: Vec<&str> = str_number_bytes
                    .chunks(divisor)
                    .map(|chunk| std::str::from_utf8(chunk).unwrap())
                    .collect();

                if str_chunks.len() > 1 {
                    if str_chunks.iter().all(|chunk| chunk.eq(&str_chunks[0])) {
                        part2_collected_numbers.push(number);
                        break;
                    }
                }
            }
        }
        println!("✅ Range: {start}-{end}");
    }

    let part1_sum = part1_collected_numbers.iter().sum::<usize>();
    let part2_sum = part2_collected_numbers.iter().sum::<usize>();

    println!("#Part 1) Collected numbers sum: {}", part1_sum);
    println!("#Part 2) Collected numbers sum: {}", part2_sum);
}
