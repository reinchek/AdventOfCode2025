use aoc2025::{clrscr, read_input};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
enum PointType {
    Empty,
    Ingress,
    Splitter(bool), // Holds the active state of the splitter, true = active, false = inactive.
    BeamPiece,
}

impl From<char> for PointType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '^' => Self::Splitter(false),
            '°' => Self::Splitter(true),
            'S' => Self::Ingress,
            '|' => Self::BeamPiece,
            _ => Self::Empty,
        }
    }
}

impl Into<char> for PointType {
    fn into(self) -> char {
        match self {
            PointType::Empty => '.',
            PointType::Splitter(false) => '^',
            PointType::Splitter(true) => '°',
            PointType::Ingress => 'S',
            PointType::BeamPiece => '|',
        }
    }
}

#[derive(Debug, Clone)]
struct TachyonBeamPath {
    grid: Vec<Vec<PointType>>,
}

impl TachyonBeamPath {
    fn new(input: &String) -> Self {
        let mut techyon_beam_vec: Vec<Vec<PointType>> = Vec::new();

        for row in input.lines() {
            let mut temp_row: Vec<PointType> = Vec::new();

            for char in row.chars() {
                temp_row.push(char.into());
            }
            techyon_beam_vec.push(temp_row);
        }

        TachyonBeamPath {
            grid: techyon_beam_vec,
        }
    }
}

impl fmt::Display for PointType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <PointType as Into<char>>::into(self.clone()))
    }
}

impl fmt::Display for TachyonBeamPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for col in row.clone() {
                write!(f, "{}", <PointType as Into<char>>::into(col));
            }
            writeln!(f, "{}", "\n");
        }
        Ok(())
    }
}

fn main() {
    clrscr(Some(7));

    let input = read_input(7, Some(true));
    // let input = read_input(7, Some(false));

    let techyon_beam_path = part_1(&input);

    println!("grid: \n{}", techyon_beam_path);
}

fn part_1(input: &String) -> TachyonBeamPath {
    let mut techyon_beam_path = TachyonBeamPath::new(&input);
    let mut active_splitters_count: usize = 1;

    for (row_idx, row) in techyon_beam_path.clone().grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if match col {
                PointType::Empty => true,
                PointType::Ingress => true,
                PointType::BeamPiece => true,
                PointType::Splitter(_) => true,
            } {
                if matches!(col, PointType::Ingress) {
                    // Showing the beam starting down from 'S' (Ingress) point.
                    techyon_beam_path.grid[row_idx + 1][col_idx] = '|'.into();
                } else if matches!(col, PointType::Splitter(_)) {
                    // Showing the beams divided at the left and right on the same splitter row.
                    techyon_beam_path.grid[row_idx][col_idx - 1] = '|'.into();
                    techyon_beam_path.grid[row_idx][col_idx + 1] = '|'.into();

                    // Since we encounter a splitter, start with a loop to propagate the beam
                    // vertically down all empty points.
                    for left_right in [col_idx - 1, col_idx + 1].to_vec().iter() {
                        let mut down_counter = 1;

                        while
                            (
                                matches!(techyon_beam_path.grid[row_idx + down_counter][*left_right], PointType::Empty) ||
                                matches!(techyon_beam_path.grid[row_idx + down_counter][*left_right], PointType::Splitter(_))
                            ) &&
                            row_idx + down_counter < techyon_beam_path.grid.len() - 1
                        {
                            if matches!(techyon_beam_path.grid[row_idx+down_counter][*left_right], PointType::Splitter(false)) {
                                active_splitters_count += 1;
                                techyon_beam_path.grid[row_idx+down_counter][*left_right] = '°'.into();
                            } else if matches!(techyon_beam_path.grid[row_idx+down_counter][*left_right], PointType::Empty){
                                // Showing the beams divided on the left and on the right of the next row.
                                techyon_beam_path.grid[row_idx + down_counter][*left_right] = '|'.into();
                                down_counter += 1;
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("active splitters_count: {}", active_splitters_count);

    techyon_beam_path
}


#[test]
fn part1_test_how_many_times_will_the_beam_be_split() {
    let input = read_input(7, Some(false));
    let techyon_beam_path = part_1(&input);
    assert_eq!(techyon_beam_path.grid.len(), 21);
}
