fn main() {
    println!("█████╗  ██████╗ ██████╗     ██████╗  ██████╗ ██████╗ ███████╗");
    println!("██╔══██╗██╔═══██╗██╔══██╗    ╚════██╗██╔═████╗╚════██╗██╔════╝");
    println!("███████║██║   ██║██║  ██║     █████╔╝██║██╔██║ █████╔╝███████╗");
    println!("██╔══██║██║   ██║██║  ██║    ██╔═══╝ ████╔╝██║██╔═══╝ ╚════██║");
    println!("██║  ██║╚██████╔╝██████╔╝    ███████╗╚██████╔╝███████╗███████║");
    println!("╚═╝  ╚═╝ ╚═════╝ ╚═════╝     ╚══════╝ ╚═════╝ ╚══════╝╚══════╝");
    println!("You have landed on my codebase where I solve the AdventOfCode 2025 challenges.\nYou are welcome!");
    println!("\nList of completed puzzles:");

    let green = "\x1b[32m";
    let gray = "\x1b[90m";
    let reset = "\x1b[0m";


    let progress = vec![
        (true, true),   // Day 1
        (true, true),  // Day 2
        (true, true), // Day 3
        (true, true), // Day 4
        (true, true), // Day 5
        (true, true), // Day 6
        (false, false), // Day 7
        (false, false), // Day 8
        (false, false), // Day 9
        (false, false), // Day 10
        (false, false), // Day 11
        (false, false), // Day 12
        (false, false), // Day 13
        (false, false), // Day 14
        (false, false), // Day 15
        (false, false), // Day 16
        (false, false), // Day 17
        (false, false), // Day 18
        (false, false), // Day 19
        (false, false), // Day 20
        (false, false), // Day 21
        (false, false), // Day 22
        (false, false), // Day 23
        (false, false), // Day 24
        (false, false), // Day 25
        // ... aggiungi gli altri giorni
    ];

    println!("+------+--------+--------+");
    println!("| Day  | Part 1 | Part 2 |");
    println!("+------+--------+--------+");

    let mut completed_parts = 0;

    for (i, (p1, p2)) in progress.iter().enumerate() {
        if *p1 { completed_parts += 1; }
        if *p2 { completed_parts += 1; }

        println!(
            "| {:>2}   |   {}{}{}    |   {}{}{}    |",
            i + 1,
            if *p1 { green } else { gray }, if *p1 { "✔" } else { "✘" }, reset,
            if *p2 { green } else { gray }, if *p2 { "✔" } else { "✘" }, reset
        );
    }

    println!("+------+--------+--------+");

    // ---- Progress Bar ----
    let total_parts = progress.len() * 2; // ogni giorno ha due parti
    let percentage = (completed_parts as f32 / total_parts as f32) * 100.0;

    let bar_width = 20;
    let filled = (percentage / 100.0 * bar_width as f32).round() as usize;
    let empty = bar_width - filled;

    let bar = format!(
        "[{}{}]",
        "█".repeat(filled),
        "-".repeat(empty)
    );

    println!("\nProgress: {} {:.0}%", bar, percentage);
}
