use adventofcode::{self as aoc, aoc_problem, map, Solution};
use std::str::Lines;

struct DeterministicDie(usize);

impl DeterministicDie {
    fn new() -> Self {
        Self(100)
    }
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0 % 100 + 1;
        Some(self.0)
    }
}

struct Solver;

impl Solver {
    fn parse_input(&self, input: Lines) -> (usize, usize) {
        let pos: Vec<usize> = input
            .map(|line| line.split_once(": ").expect("Expect one delimiter").1)
            .map(|n| n.parse().unwrap())
            .collect();

        assert_eq!(pos.len(), 2);

        (pos[0], pos[1])
    }
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let (pos1, pos2) = self.parse_input(input);

        const TARGET: usize = 1000;
        let mut die = DeterministicDie::new();

        let mut scores = [0, 0];
        let mut positions = [pos1 - 1, pos2 - 1];
        for turn in 0.. {
            let roll = (0..3).map(|_| die.next().unwrap()).sum::<usize>();
            positions[turn % 2] = (positions[turn % 2] + roll) % 10;
            scores[turn % 2] += positions[turn % 2] + 1;
            if scores[turn % 2] >= TARGET {
                let rounds = 3 * (turn + 1);
                return rounds * scores[0].min(scores[1]);
            }
        }
        unreachable!()
    }

    fn solve_b(&self, input: Lines) -> usize {
        let (pos1, pos2) = self.parse_input(input);
        const TARGET: usize = 21;

        let mut scores = map![0 => map![(pos1-1, pos2-1, 0, 0) => 1_usize]];
        let mut final_scores = map![];

        const POSSIBILITIES: [(usize, usize); 7] =
            [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

        for turn in 0.. {
            match scores.get(&turn) {
                Some(m) if !m.is_empty() => {}
                _ => {
                    break;
                }
            }
            let mut m = map![];
            for (&(pos1, pos2, sc1, sc2), &cnt) in scores.get(&turn).unwrap() {
                if sc1 == TARGET || sc2 == TARGET {
                    *final_scores.entry((sc1, sc2)).or_insert(0) += cnt;
                    continue;
                }
                for (d1, freq1) in POSSIBILITIES {
                    let new_pos1 = (pos1 + d1) % 10;
                    let new_sc1 = (sc1 + new_pos1 + 1).min(TARGET);
                    if new_sc1 == TARGET {
                        *m.entry((new_pos1, pos2, new_sc1, sc2)).or_insert(0) +=
                            cnt * freq1;
                    } else {
                        for (d2, freq2) in POSSIBILITIES {
                            let new_pos2 = (pos2 + d2) % 10;
                            let new_sc2 = (sc2 + new_pos2 + 1).min(TARGET);
                            *m.entry((new_pos1, new_pos2, new_sc1, new_sc2))
                                .or_insert(0) += cnt * (freq1 * freq2);
                        }
                    }
                }
            }
            scores.insert(turn + 1, m);
        }

        let wins1: usize = final_scores
            .iter()
            .filter(|(&(sc1, _), _)| sc1 == TARGET)
            .map(|el| el.1)
            .copied()
            .sum();
        let wins2: usize = final_scores
            .iter()
            .filter(|(&(sc1, sc2), _)| sc1 < TARGET && sc2 == TARGET)
            .map(|el| el.1)
            .copied()
            .sum();

        wins1.max(wins2)
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 21);
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
            |Player 1 starting position: 4
            |Player 2 starting position: 8
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 739_785);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 444_356_092_776_315);
    }
}
