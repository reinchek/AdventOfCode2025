use std::fmt;
use aoc2025::{clrscr, read_input};

#[derive(Debug, Clone)]
enum PointType {
    Empty,
    Ingress,
    Splitter,
    BeamPiece
}

impl From<char> for PointType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '^' => Self::Splitter,
            'S' => Self::Ingress,
            '|' => Self::BeamPiece,
            _ => Self::Empty
        }
    }
}

impl Into<char> for PointType {
    fn into(self) -> char {
        match self {
            PointType::Empty => '.',
            PointType::Splitter => '^',
            PointType::Ingress => 'S',
            PointType::BeamPiece => '|'
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
            grid: techyon_beam_vec
        }
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

    let techyon_beam_path = TachyonBeamPath::new(&input);
    let techyon_beam_path = part_1(&input);

    println!("grid: {}", techyon_beam_path);
}

fn part_1(input: &String) -> TachyonBeamPath {
    let mut techyon_beam_path = TachyonBeamPath::new(&input);
    let mut current_position: (usize, usize) = (0, 0);
    for (row_idx, row) in techyon_beam_path.clone().grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if match col {
                PointType::Empty => false,
                PointType::Ingress => true,
                PointType::BeamPiece => true,
                PointType::Splitter => true
            } {
              if matches!(col, PointType::Splitter) {
                  current_position = (row_idx, col_idx);

                  // Showing the beam starting down from 'S' (Ingress) point.
                  techyon_beam_path.grid[row_idx+1][col_idx] = '|'.into();
              } else if matches!(col, PointType::Splitter) {
                  current_position = (row_idx, col_idx);

                  // Showing the beams divided on the left and on the right of next row.
                  techyon_beam_path.grid[row_idx+1][col_idx-1] = '|'.into();
                  techyon_beam_path.grid[row_idx+1][col_idx+1] = '|'.into();
              }
            }
        }
    }

    techyon_beam_path
}