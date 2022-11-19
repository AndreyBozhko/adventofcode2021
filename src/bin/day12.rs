use adventofcode::{self as aoc, aoc_problem, map, Solution};
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::Lines;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            nm => {
                if nm == nm.to_lowercase() {
                    Self::Small(nm.to_string())
                } else {
                    Self::Big(nm.to_string())
                }
            }
        }
    }
}

type CaveSystem = HashMap<Cave, Vec<Cave>>;

enum CanVisit {
    OnlyOnce,
    OneSmallCaveTwice,
}

struct CaveExplorer<'a> {
    caves: &'a CaveSystem,
    seen: RefCell<Vec<&'a Cave>>,
    visit_small_caves: CanVisit,
}

impl<'a> CaveExplorer<'a> {
    fn new(caves: &'a CaveSystem, visit_small_caves: CanVisit) -> Self {
        Self {
            caves,
            seen: RefCell::new(vec![]),
            visit_small_caves,
        }
    }

    fn validate_path_for(&self, cv: &Cave) -> bool {
        let seen = self.seen.borrow();
        match self.visit_small_caves {
            CanVisit::OnlyOnce => !seen.contains(&cv),
            CanVisit::OneSmallCaveTwice => {
                !seen.contains(&cv)
                    || (matches!(cv, Cave::Small(_)) && seen.iter().all_unique())
            }
        }
    }

    fn count_paths_to_exit_from(&self, cur: &'a Cave) -> usize {
        match cur {
            Cave::End => 1,
            cave if self.validate_path_for(cave) => {
                if !matches!(cave, Cave::Big(_)) {
                    self.seen.borrow_mut().push(cave);
                }
                let count = self
                    .caves
                    .get(cave)
                    .unwrap()
                    .iter()
                    .map(|next| self.count_paths_to_exit_from(next))
                    .sum();
                if !matches!(cave, Cave::Big(_)) {
                    self.seen.borrow_mut().pop();
                }
                count
            }
            _ => 0,
        }
    }
}

struct Solver;

impl Solver {
    fn parse_input(&self, input: Lines) -> CaveSystem {
        let mut caves: CaveSystem = map![];

        input
            .map(|line| line.split_once('-').expect("Expect one delimiter"))
            .flat_map(|(c1, c2)| {
                [
                    (Cave::from(c1), Cave::from(c2)),
                    (Cave::from(c2), Cave::from(c1)),
                ]
            })
            .for_each(|(from, to)| caves.entry(from).or_default().push(to));

        caves
    }
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let caves = self.parse_input(input);
        let explorer = CaveExplorer::new(&caves, CanVisit::OnlyOnce);
        explorer.count_paths_to_exit_from(&Cave::Start)
    }

    fn solve_b(&self, input: Lines) -> usize {
        let caves = self.parse_input(input);
        let explorer = CaveExplorer::new(&caves, CanVisit::OneSmallCaveTwice);
        explorer.count_paths_to_exit_from(&Cave::Start)
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 12);
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
    fn test_example_1() {
        let solver = Solver {};
        let input = r"
            |start-A
            |start-b
            |A-c
            |A-b
            |b-d
            |A-end
            |b-end
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 10);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 36);
    }

    #[test]
    fn test_example_2() {
        let solver = Solver {};
        let input = r"
            |dc-end
            |HN-start
            |start-kj
            |dc-start
            |dc-HN
            |LN-dc
            |HN-end
            |kj-sa
            |kj-HN
            |kj-dc
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 19);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 103);
    }

    #[test]
    fn test_example_3() {
        let solver = Solver {};
        let input = r"
            |fs-end
            |he-DX
            |fs-he
            |start-DX
            |pj-DX
            |end-zg
            |zg-sl
            |zg-pj
            |pj-he
            |RW-he
            |fs-DX
            |pj-RW
            |zg-RW
            |start-pj
            |he-WI
            |zg-he
            |pj-fs
            |start-RW
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 226);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 3509);
    }
}
