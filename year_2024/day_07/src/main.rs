use aoc_lib::{PuzzleDate, PuzzleSolution, SolutionResult};

fn concat_nums(left: u64, right: u64) -> u64 {
    let mut str = left.to_string();
    str.push_str(&right.to_string());
    str.parse().unwrap()
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: u64,
    params: Vec<u64>,
}

impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let (test_value, params) = s.split_once(": ").unwrap();

        Self {
            test_value: test_value.parse().unwrap(),
            params: params
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}

impl Equation {
    fn is_solvable(&self) -> bool {
        if let [n] = &self.params[..] {
            self.test_value == *n
        } else {
            let sum = self.params[0] + self.params[1];
            let mut params_sum = self.params.clone();
            params_sum.remove(0);
            params_sum[0] = sum;

            let mul = self.params[0] * self.params[1];
            let mut params_mul = params_sum.clone();
            params_mul[0] = mul;

            let sum_eq = Self {
                test_value: self.test_value,
                params: params_sum,
            };

            let mul_eq = Self {
                test_value: self.test_value,
                params: params_mul,
            };

            sum_eq.is_solvable() || mul_eq.is_solvable()
        }
    }

    fn is_solvable_2(&self) -> bool {
        if let [n] = &self.params[..] {
            self.test_value == *n
        } else {
            let mut params_sum = self.params.clone();
            params_sum.remove(0);
            params_sum[0] = self.params[0] + self.params[1];

            let mut params_mul = params_sum.clone();
            params_mul[0] = self.params[0] * self.params[1];

            let mut params_concat = params_sum.clone();
            params_concat[0] = concat_nums(self.params[0], self.params[1]);

            let sum_eq = Self {
                test_value: self.test_value,
                params: params_sum,
            };

            let mul_eq = Self {
                test_value: self.test_value,
                params: params_mul,
            };

            let concat_eq = Self {
                test_value: self.test_value,
                params: params_concat,
            };

            sum_eq.is_solvable_2() || mul_eq.is_solvable_2() || concat_eq.is_solvable_2()
        }
    }
}

struct Day7;

impl PuzzleSolution for Day7 {
    type Input = Vec<Equation>;
    type Output = u64;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input.lines().map(Equation::from).collect()
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let count = input
            .iter()
            .filter(|eq| eq.is_solvable())
            .map(|eq| eq.test_value)
            .sum();
        Ok(count)
    }
    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        let count = input
            .iter()
            .filter(|eq| eq.is_solvable_2())
            .map(|eq| eq.test_value)
            .sum();
        Ok(count)
    }
}

fn main() {
    aoc_lib::cli::<Day7>(PuzzleDate::new(2024, 7));
}
