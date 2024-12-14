use aoc_lib::cli::{PuzzleSolution, SolutionResult};

fn determinant(mat: [[i128; 2]; 2]) -> i128 {
    mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
}

#[derive(Debug, Clone)]
struct Button {
    x_step: i128,
    y_step: i128,
}

impl From<&str> for Button {
    fn from(s: &str) -> Self {
        let (_, real_part) = s.split_once(": ").unwrap();
        let e = real_part
            .split(", ")
            .map(|part| part.split_once("+").unwrap().1.parse::<i128>().unwrap())
            .collect::<Vec<_>>();

        Self {
            x_step: e[0],
            y_step: e[1],
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_x: i128,
    prize_y: i128,
}

impl Machine {
    fn moves_required(&self) -> Option<(i128, i128)> {
        let d = determinant([
            [self.button_a.x_step, self.button_b.x_step],
            [self.button_a.y_step, self.button_b.y_step],
        ]);

        let dx = determinant([
            [self.prize_x, self.button_b.x_step],
            [self.prize_y, self.button_b.y_step],
        ]);

        let dy = determinant([
            [self.button_a.x_step, self.prize_x],
            [self.button_a.y_step, self.prize_y],
        ]);

        if dx % d != 0 || dy % d != 0 {
            None
        } else {
            Some((dx / d, dy / d))
        }
    }
}

struct Day12;

impl PuzzleSolution for Day12 {
    type Input = Vec<Machine>;
    type Output = i128;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines = raw_input.trim().lines().collect::<Vec<_>>();

        let mut machines = vec![];

        for chunk in lines.split(|s| s.is_empty()) {
            let buttons = chunk[..2]
                .iter()
                .map(|&s| Button::from(s))
                .collect::<Vec<_>>();
            let prizes = chunk[2]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|part| part.split_once("=").unwrap().1.parse::<i128>().unwrap())
                .collect::<Vec<_>>();

            machines.push(Machine {
                button_a: buttons[0].clone(),
                button_b: buttons[1].clone(),
                prize_x: prizes[0],
                prize_y: prizes[1],
            })
        }

        machines
    }

    fn part_1(machines: &Self::Input) -> SolutionResult<Self::Output> {
        Ok(machines
            .iter()
            .map(|m| m.moves_required())
            .map(|opt| opt.map(|(a, b)| 3 * a + b).unwrap_or(0))
            .sum())
    }

    fn part_2(machines: &Self::Input) -> SolutionResult<Self::Output> {
        let new_machines = machines
            .iter()
            .map(|machine| {
                let mut machine = machine.clone();
                machine.prize_x += 10000000000000;
                machine.prize_y += 10000000000000;
                machine
            })
            .collect::<Vec<_>>();

        Self::part_1(&new_machines)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day12>(2024, 13);
}
