use aoc_lib::{
    cli::{PuzzleSolution, SolutionResult},
    helper::structs::Index2D,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
struct AntennaMap {
    size: Index2D<isize>,
    antennas: HashMap<char, Vec<Index2D<isize>>>,
}

struct Day8;

impl PuzzleSolution for Day8 {
    type Input = AntennaMap;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        let mut antennas = HashMap::<char, Vec<Index2D<isize>>>::new();
        let char_matrix = raw_input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for (i, row) in char_matrix.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch != '.' {
                    antennas
                        .entry(ch)
                        .or_default()
                        .push(Index2D::new(i as isize, j as isize));
                }
            }
        }

        AntennaMap {
            size: Index2D::new(char_matrix.len() as isize, char_matrix[0].len() as isize),
            antennas,
        }
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let AntennaMap { size, antennas } = input;
        let mut antinodes = HashSet::new();

        for (_, antennas) in antennas {
            for i in 0..antennas.len() {
                let a = antennas[i];

                for j in (i + 1)..antennas.len() {
                    let b = antennas[j];
                    let diff = b - a;

                    let antinode_a = a - diff;
                    let antinode_b = b + diff;

                    if (0..size.i).contains(&antinode_a.i) && (0..size.j).contains(&antinode_a.j) {
                        antinodes.insert(antinode_a);
                    }

                    if (0..size.i).contains(&antinode_b.i) && (0..size.j).contains(&antinode_b.j) {
                        antinodes.insert(antinode_b);
                    }
                }
            }
        }

        Ok(antinodes.len())
    }

    fn part_2(map: &Self::Input) -> SolutionResult<Self::Output> {
        let mut antinodes = HashSet::new();

        for (_, antennas) in &map.antennas {
            for i in 0..antennas.len() {
                let a = antennas[i];

                for j in (i + 1)..antennas.len() {
                    let b = antennas[j];
                    let step = b - a;

                    let mut pos = b;

                    while (0..map.size.i).contains(&pos.i) && (0..map.size.j).contains(&pos.j) {
                        antinodes.insert(pos);
                        pos -= step;
                    }

                    pos = a;

                    while (0..map.size.i).contains(&pos.i) && (0..map.size.j).contains(&pos.j) {
                        antinodes.insert(pos);
                        pos += step;
                    }
                }
            }
        }

        Ok(antinodes.len())
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day8>(2024, 8);
}
