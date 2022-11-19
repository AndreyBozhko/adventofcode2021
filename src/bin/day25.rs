use adventofcode::{self as aoc, aoc_problem, set, Solution};
use std::str::Lines;

struct Solver;

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let mut grid: Vec<Vec<_>> = input
            .map(|line| {
                line.chars()
                    .map(|c| if c == '.' { None } else { Some(c) })
                    .collect()
            })
            .collect();

        let n = grid.len();
        let m = grid[0].len();

        let mut east_queue = set![];
        let mut south_queue = set![];
        for i in 0..n {
            for j in 0..m {
                if let Some(c) = &grid[i][j] {
                    if *c == '>' && grid[i][(j + 1) % m] == None {
                        east_queue.insert((i, j));
                    }
                    if *c == 'v' && grid[(i + 1) % n][j] == None {
                        south_queue.insert((i, j));
                    }
                }
            }
        }

        for step in 1.. {
            if south_queue.is_empty() && east_queue.is_empty() {
                return step;
            }

            let mut east_new = set![];
            let mut south_new = set![];

            for &(i, j) in &east_queue {
                if grid[i][(j + 1) % m] == None {
                    grid[i][(j + 1) % m] = Some('>');
                    grid[i][j] = None;
                }
            }
            for &(i, j) in &east_queue {
                if grid[i][j] == None {
                    south_queue.remove(&((i + n - 1) % n, (j + 1) % m));
                    if grid[i][(j + 2) % m] == None {
                        east_new.insert((i, (j + 1) % m));
                    }
                    if grid[i][(j + m - 1) % m] == Some('>') {
                        east_new.insert((i, (j + m - 1) % m));
                    }
                    if grid[(i + n - 1) % n][j] == Some('v') {
                        south_queue.insert(((i + n - 1) % n, j));
                    }
                }
            }
            east_queue = east_new;

            for &(i, j) in &south_queue {
                if grid[(i + 1) % n][j] == None {
                    grid[(i + 1) % n][j] = Some('v');
                    grid[i][j] = None;
                }
            }
            for &(i, j) in &south_queue {
                if grid[i][j] == None {
                    east_queue.remove(&((i + 1) % n, (j + m - 1) % m));
                    if grid[(i + 2) % n][j] == None {
                        south_new.insert(((i + 1) % n, j));
                    }
                    if grid[(i + n - 1) % n][j] == Some('v') {
                        south_new.insert(((i + n - 1) % n, j));
                    }
                    if grid[i][(j + m - 1) % m] == Some('>') {
                        east_queue.insert((i, (j + m - 1) % m));
                    }
                }
            }
            south_queue = south_new;
        }

        unreachable!()
    }

    fn solve_b(&self, _input: Lines) -> usize {
        unreachable!("Merry X-mas!")
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 25);
    let solver = Solver {};

    match aoc::earn_star(problem.part_a(), solver) {
        Ok(resp) => println!("Response: {}", resp),
        Err(err) => panic!("Something went wrong: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::StripMargin;

    #[test]
    fn test_solution() {
        let solver = Solver {};
        let input = r"
            | v...>>.vv>
            | .vv>>.vv..
            | >>.>v>...v
            | >>v>>.>.v.
            | v>v.vv.v..
            | >.>>..v...
            | .vv..>.>v.
            | v.v..>>v.v
            | ....v..v.>
        "
        .trim()
        .strip_margin_of("| ");

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 58);
    }
}
