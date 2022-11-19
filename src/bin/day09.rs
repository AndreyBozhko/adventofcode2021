use adventofcode::{self as aoc, aoc_problem, Solution};
use itertools::Itertools;
use std::iter;
use std::str::Lines;

struct Solver;

fn all_neighbors(
    (i, j): (usize, usize),
    (max_i, max_j): (usize, usize),
) -> Vec<(usize, usize)> {
    let neighbors = [
        if i == 0 { None } else { Some((i - 1, j)) },
        if i + 1 == max_i {
            None
        } else {
            Some((i + 1, j))
        },
        if j == 0 { None } else { Some((i, j - 1)) },
        if j + 1 == max_j {
            None
        } else {
            Some((i, j + 1))
        },
    ];
    neighbors.iter().flatten().copied().collect()
}

fn dfs(
    heights: &mut Vec<Vec<(char, i32)>>,
    idx: i32,
    (x, y): (usize, usize),
    bounds: (usize, usize),
) -> usize {
    heights[x][y] = (heights[x][y].0, idx);

    let mut sz = 1;
    for (nx, ny) in all_neighbors((x, y), bounds) {
        match heights[nx][ny] {
            (d, 0) if d != '9' => {
                sz += dfs(heights, idx, (nx, ny), bounds);
            }
            _ => {}
        }
    }

    sz
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let heights: Vec<Vec<_>> = input.map(|el| el.chars().collect()).collect();
        let mut risk = 0;

        for i in 0..heights.len() {
            for j in 0..heights[i].len() {
                let neighbors =
                    all_neighbors((i, j), (heights.len(), heights[i].len()));
                if neighbors
                    .iter()
                    .all(|&(x, y)| heights[x][y] > heights[i][j])
                {
                    risk += 1 + heights[i][j] as usize - '0' as usize;
                }
            }
        }
        risk
    }

    fn solve_b(&self, input: Lines) -> usize {
        let mut heights: Vec<Vec<_>> = input
            .map(|el| el.chars().zip(iter::repeat(0)).collect())
            .collect();

        let mut idx = 0;
        let mut sizes = vec![];

        for i in 0..heights.len() {
            for j in 0..heights[i].len() {
                match heights[i][j] {
                    (c, 0) if c != '9' => {
                        idx += 1;
                        let bounds = (heights.len(), heights[i].len());
                        sizes.push(dfs(&mut heights, idx, (i, j), bounds));
                    }
                    _ => {}
                }
            }
        }

        sizes
            .iter()
            .sorted_by_key(|&&sz| -(sz as i32))
            .take(3)
            .product()
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 9);
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
            |2199943210
            |3987894921
            |9856789892
            |8767896789
            |9899965678
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 15);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 1134);
    }
}
