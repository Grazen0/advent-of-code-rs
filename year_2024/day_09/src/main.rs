use aoc_lib::cli::{PuzzleSolution, SolutionResult};

fn checksum(filesystem: Vec<usize>) -> usize {
    filesystem
        .iter()
        .enumerate()
        .map(|(i, &n)| i * (n as usize))
        .sum()
}

#[derive(Debug, Clone)]
enum Segment {
    File { id: usize, size: usize },
    Space(usize),
}

struct Day9;

impl PuzzleSolution for Day9 {
    type Input = Vec<u8>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .trim()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as u8)
            .collect()
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut input = input.clone();
        let mut mem = Vec::new();

        let mut left_id = 0;
        let mut left_ptr = 0;
        let mut right_id = (input.len() - 1) / 2;
        let mut right_ptr = input.len() - 1;

        'main: while left_ptr < input.len() - 1 {
            while input[left_ptr] > 0 {
                mem.push(left_id);
                input[left_ptr] -= 1;
            }

            left_id += 1;

            while input[left_ptr + 1] > 0 {
                if input[right_ptr] == 0 {
                    right_id -= 1;
                    right_ptr -= 2;
                }

                if right_ptr == 0 || input[right_ptr] == 0 {
                    break 'main;
                }

                mem.push(right_id);

                input[right_ptr] -= 1;
                input[left_ptr + 1] -= 1;
            }

            left_ptr += 2;
        }

        Ok(checksum(mem))
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut mem = Vec::new();

        for (i, &n) in input.iter().enumerate() {
            if i % 2 == 0 {
                mem.push(Segment::File {
                    id: i / 2,
                    size: n as usize,
                })
            } else {
                mem.push(Segment::Space(n as usize));
            }
        }

        for i in (0..mem.len()).rev() {
            if let Segment::File { id, size } = mem[i].clone() {
                for j in 0..i {
                    if let Segment::Space(space) = mem[j] {
                        if space >= size {
                            let _ = std::mem::replace(&mut mem[i], Segment::Space(size));
                            mem[j] = Segment::Space(space - size);
                            mem.insert(j, Segment::File { id, size });
                            break;
                        }
                    }
                }
            }
        }

        let mut i = 0;
        let mut checksum = 0;

        for segment in mem {
            match segment {
                Segment::File { id, size } => {
                    for _ in 0..size {
                        checksum += i * id;
                        i += 1;
                    }
                }
                Segment::Space(size) => i += size,
            }
        }

        Ok(checksum)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day9>(2024, 9);
}
