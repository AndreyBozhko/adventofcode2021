use adventofcode::{self as aoc, aoc_problem, set, Solution};
use itertools::Itertools;
use std::collections::HashMap;
use std::iter;
use std::str::Lines;

const N: usize = 10;

struct Octopus {
    energy: u32,
    flashed: bool,
}

impl From<char> for Octopus {
    fn from(c: char) -> Self {
        Self {
            energy: c.to_digit(10).expect("Expect valid number"),
            flashed: false,
        }
    }
}

struct Solver;

fn all_neighbors((i, j): (usize, usize)) -> Vec<(usize, usize)> {
    let xs = (i.max(1) - 1)..=(i + 1).min(N - 1);
    let ys = (j.max(1) - 1)..=(j + 1).min(N - 1);

    xs.cartesian_product(ys)
        .filter(|&pair| pair != (i, j))
        .collect()
}

fn run_step(octopuses: &mut HashMap<(usize, usize), Octopus>) -> usize {
    let mut queue1 = set![];
    let mut queue2 = set![];

    for (&pos, oct) in octopuses.iter_mut() {
        oct.energy = (oct.energy + 1) % 10;
        if oct.energy == 0 {
            oct.flashed = true;
            queue1.insert(pos);
        } else {
            oct.flashed = false;
        }
    }

    while !queue1.is_empty() {
        for (i, j) in queue1.iter().copied().flat_map(all_neighbors) {
            let oct = octopuses.get_mut(&(i, j)).unwrap();
            if !oct.flashed {
                oct.energy = (oct.energy + 1) % 10;
                if oct.energy == 0 {
                    oct.flashed = true;
                    queue2.insert((i, j));
                }
            }
        }
        queue1 = queue2;
        queue2 = set![];
    }

    octopuses.values().filter(|oct| oct.flashed).count()
}

impl Solver {
    fn parse_input(&self, input: Lines) -> HashMap<(usize, usize), Octopus> {
        input
            .flat_map(|line| line.chars())
            .enumerate()
            .map(|(k, c)| ((k / 10, k % 10), Octopus::from(c)))
            .collect()
    }
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let mut octopuses = self.parse_input(input);

        iter::repeat_with(|| run_step(&mut octopuses))
            .take(100)
            .sum()
    }

    fn solve_b(&self, input: Lines) -> usize {
        let mut octopuses = self.parse_input(input);

        1 + iter::repeat_with(|| run_step(&mut octopuses))
            .find_position(|&flashes| flashes == N * N)
            .unwrap()
            .0
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 11);
    let solver = Solver {};

    match aoc::earn_star(problem.part_b(), solver) {
        Ok(resp) => println!("Response: {}", resp),
        Err(err) => panic!("Something went wrong: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::StripMargin;

    #[test]
    fn test_solution() {
        let solver = Solver {};
        let input = r"
            |5483143223
            |2745854711
            |5264556173
            |6141336146
            |6357385478
            |4167524645
            |2176841721
            |6882881134
            |4846848554
            |5283751526
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 1656);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 195);
    }
}
