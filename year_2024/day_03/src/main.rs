use aoc_lib::cli::{PuzzleSolution, SolutionResult};

struct Day3;

fn read_number(s: &str, index: &mut usize) -> Option<i32> {
    let num_len = s[*index..]
        .find(|ch: char| !ch.is_ascii_digit())
        .unwrap_or(s.len());

    if num_len == 0 {
        None
    } else {
        let num_str = &s[*index..][..num_len];
        *index += num_len;
        Some(num_str.parse().unwrap())
    }
}

fn read_mul(s: &str, index: &mut usize) -> Option<i32> {
    if !s[*index..].starts_with("mul(") {
        return None;
    }
    *index += "mul(".len();

    let left = read_number(s, index)?;

    if s.as_bytes()[*index] != b',' {
        return None;
    }

    *index += 1;
    let right = read_number(s, index)?;

    if s.as_bytes()[*index] != b')' {
        return None;
    }

    *index += 1;
    Some(left * right)
}

impl PuzzleSolution for Day3 {
    type Input = String;
    type Output = i32;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut sum = 0;
        let mut index = 0;

        while index < input.len() {
            match read_mul(input, &mut index) {
                Some(value) => sum += value,
                None => index += 1,
            }
        }

        Ok(sum)
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut sum = 0;
        let mut index = 0;
        let mut do_enabled = true;

        while index < input.len() {
            if do_enabled {
                if input[index..].starts_with("don't()") {
                    index += "don't()".len();
                    do_enabled = false;
                } else {
                    match read_mul(input, &mut index) {
                        Some(value) => sum += value,
                        None => index += 1,
                    }
                }
            } else if input[index..].starts_with("do()") {
                index += "do()".len();
                do_enabled = true;
            } else {
                index += 1;
            }
        }

        Ok(sum)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day3>(2024, 3);
}
