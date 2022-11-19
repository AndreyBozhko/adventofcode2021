use adventofcode::{self as aoc, aoc_problem, Solution};
use std::collections::BinaryHeap;
use std::str::Lines;

struct Solver;

impl Solver {
    fn parse_input(&self, input: Lines) -> Vec<Vec<i32>> {
        input
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect()
    }
}

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

fn dijkstra(grid: Vec<Vec<i32>>) -> Option<i32> {
    let n = grid.len();
    assert!(grid.iter().all(|el| el.len() == n), "Expect square grid");

    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![None; n]; n];

    visited[0][0] = Some(0);
    heap.push((-0, 0, 0));

    while let Some((negative_cost, i, j)) = heap.pop() {
        let cost = -negative_cost;

        if (i, j) == (n - 1, n - 1) {
            return Some(cost);
        }

        if let Some(c) = visited[i][j] {
            if cost > c {
                continue;
            }
        }

        for (x, y) in all_neighbors((i, j), (n, n)) {
            let new_cost = cost + grid[x][y];
            match visited[x][y] {
                Some(c) if new_cost >= c => {}
                _ => {
                    heap.push((-new_cost, x, y));
                    visited[x][y] = Some(new_cost);
                }
            }
        }
    }
    None
}

impl Solution for Solver {
    type Output = i32;

    fn solve_a(&self, input: Lines) -> i32 {
        let grid = self.parse_input(input);

        dijkstra(grid).unwrap()
    }

    fn solve_b(&self, input: Lines) -> i32 {
        let gr = self.parse_input(input);
        let n = gr.len();

        let mut grid = vec![vec![0; 5 * n]; 5 * n];
        for i in 0..n {
            for j in 0..n {
                for ii in 0..5 {
                    for jj in 0..5 {
                        let val = (gr[i][j] + (ii + jj) as i32) % 9;
                        grid[i + ii * n][j + jj * n] = if val == 0 { 9 } else { val };
                    }
                }
            }
        }

        dijkstra(grid).unwrap()
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 15);
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
            |1163751742
            |1381373672
            |2136511328
            |3694931569
            |7463417111
            |1319128137
            |1359912421
            |3125421639
            |1293138521
            |2311944581
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 40);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 315);
    }
}
