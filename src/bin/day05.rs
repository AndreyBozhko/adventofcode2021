use adventofcode::{self as aoc, aoc_problem, make_err, map, Solution};
use std::cmp::Ordering;
use std::str::{FromStr, Lines};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
struct Segment(Point, Point);

struct PointIterator {
    cur: Point,
    end: Point,
    dir: (i32, i32),
    started: bool,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.cur);
        }
        if self.cur == self.end {
            return None;
        }
        self.cur = Point::new(self.cur.x + self.dir.0, self.cur.y + self.dir.1);
        Some(self.cur)
    }
}

impl Segment {
    fn iterate_points(&self) -> PointIterator {
        let dir = (
            match self.0.x.cmp(&self.1.x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
            match self.0.y.cmp(&self.1.y) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
        );

        PointIterator {
            cur: self.0,
            end: self.1,
            dir,
            started: false,
        }
    }
}

impl FromStr for Segment {
    type Err = aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(" -> ", ",");
        let nums: Vec<i32> = s.split(',').map(|n| n.parse().unwrap()).collect();
        if let [x1, y1, x2, y2] = nums[..] {
            Ok(Self(Point::new(x1, y1), Point::new(x2, y2)))
        } else {
            make_err!("Failed to parse")
        }
    }
}

fn count_intersections(segments: &[Segment]) -> usize {
    let mut pts = map![];
    for s in segments {
        for pt in s.iterate_points() {
            *pts.entry(pt).or_insert(0_usize) += 1;
        }
    }

    pts.values().filter(|&&cnt| cnt > 1).count()
}

struct Solver;

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let segments: Vec<Segment> = input
            .map(|line| line.parse().unwrap())
            .filter(|s: &Segment| s.0.x == s.1.x || s.0.y == s.1.y)
            .collect();

        count_intersections(&segments)
    }

    fn solve_b(&self, input: Lines) -> usize {
        let segments: Vec<Segment> = input.map(|line| line.parse().unwrap()).collect();

        count_intersections(&segments)
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 5);
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
            |0,9 -> 5,9
            |8,0 -> 0,8
            |9,4 -> 3,4
            |2,2 -> 2,1
            |7,0 -> 7,4
            |6,4 -> 2,0
            |0,9 -> 2,9
            |3,4 -> 1,4
            |0,0 -> 8,8
            |5,5 -> 8,2
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 5);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 12);
    }
}
