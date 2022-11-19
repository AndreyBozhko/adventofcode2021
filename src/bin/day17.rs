use adventofcode::{self as aoc, aoc_problem, Solution};
use itertools::Itertools;
use std::str::Lines;

struct Solver;

impl Solver {
    fn parse_input(&self, input: Lines) -> ((i32, i32), (i32, i32)) {
        let coords = input
            .last()
            .unwrap()
            .strip_prefix("target area: x=")
            .unwrap()
            .split(", y=")
            .flat_map(|el| el.split(".."))
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        ((coords[0], coords[1]), (coords[2], coords[3]))
    }
}

fn simulate(
    (mut vx, mut vy): (i32, i32),
    (xmin, xmax): (i32, i32),
    (ymin, ymax): (i32, i32),
) -> bool {
    let (mut x, mut y) = (0, 0);
    loop {
        x += vx;
        y += vy;
        vx = (vx - 1).max(0);
        vy -= 1;
        if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
            return true;
        }
        if xmax < x || y < ymin {
            return false;
        }
    }
}

impl Solution for Solver {
    type Output = i32;

    fn solve_a(&self, input: Lines) -> i32 {
        let (_, (ymin, ymax)) = self.parse_input(input);

        assert!(ymin < 0);
        assert!(ymin < ymax);

        let vy = ymin.abs() - 1;
        vy * (1 + vy) / 2
    }

    fn solve_b(&self, input: Lines) -> i32 {
        let ((xmin, xmax), (ymin, ymax)) = self.parse_input(input);

        assert!(ymin < 0);
        assert!(ymin < ymax);
        assert!(xmin > 0);
        assert!(xmin < xmax);

        let vy_max = ymin.abs() - 1;
        let vy_min = ymin;
        let vx_max = xmax;

        (0..=vx_max)
            .cartesian_product(vy_min..=vy_max)
            .filter(|&(vx, vy)| simulate((vx, vy), (xmin, xmax), (ymin, ymax)))
            .count() as i32
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 17);
    let solver = Solver {};

    match aoc::earn_star(problem.part_b(), solver) {
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
            |target area: x=20..30, y=-10..-5
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 45);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 112);
    }
}
