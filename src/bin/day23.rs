use adventofcode::{self as aoc, aoc_problem, Solution};
use std::collections::{BinaryHeap, VecDeque};
use std::str::Lines;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Apod {
    A,
    B,
    C,
    D,
}

macro_rules! impl_apod_attr {
    ($meth:ident() -> $rt:ty, [$a:expr, $b:expr, $c:expr, $d:expr]) => {
        impl Apod {
            fn $meth(&self) -> $rt {
                match self {
                    Apod::A => $a,
                    Apod::B => $b,
                    Apod::C => $c,
                    Apod::D => $d,
                }
            }
        }
    };
}

impl_apod_attr!(dest() -> usize, [2, 4, 6, 8]);
impl_apod_attr!(coeff() -> usize, [1, 10, 100, 1000]);

impl From<&str> for Apod {
    fn from(s: &str) -> Self {
        use Apod::*;
        match s {
            "A" => A,
            "B" => B,
            "C" => C,
            "D" => D,
            _ => panic!(),
        }
    }
}

struct Solver;

fn check(rooms: &[VecDeque<Apod>; 4], cap: usize) -> bool {
    rooms
        .iter()
        .zip([Apod::A, Apod::B, Apod::C, Apod::D])
        .all(|(r, apod)| r.len() == cap && r.iter().all(|&a| a == apod))
}

fn dfs(
    rooms: &mut [VecDeque<Apod>; 4],
    hall: &mut [Option<Apod>; 11],
    best: &mut BinaryHeap<usize>,
    score: usize,
    cap: usize,
) -> usize {
    if check(rooms, cap) {
        match best.peek() {
            Some(sc) if *sc <= score => {}
            _ => {
                // println!("New min energy: {}", score);
                best.push(score);
                while best.len() > 1 {
                    best.pop();
                }
            }
        }
        return score;
    }

    // move from hall into room
    for idx in [0, 1, 3, 5, 7, 9, 10] {
        let mut tot = usize::MAX;
        if hall[idx].is_none() {
            continue;
        }
        let apod = hall[idx].unwrap();
        let dst = apod.dest();
        let coeff = apod.coeff();

        let ri = dst / 2 - 1;
        hall[idx] = None;

        if rooms[ri].iter().all(|&a| a == apod)
            && rooms[ri].len() < cap
            && hall[dst.min(idx)..=dst.max(idx)]
                .iter()
                .all(|a| a.is_none())
        {
            rooms[ri].push_front(apod);
            tot = tot.min(dfs(
                rooms,
                hall,
                best,
                score
                    + coeff * (dst.max(idx) - dst.min(idx) + 1 + cap - rooms[ri].len()),
                cap,
            ));
            rooms[ri].pop_front();
        }
        hall[idx] = Some(apod);
        if tot != usize::MAX {
            return tot;
        }
    }

    // move from room into hall
    let mut totals = vec![];
    for i in 0..4 {
        let src = 2 * (i + 1);
        if rooms[i].is_empty() {
            continue;
        }
        let apod = rooms[i].pop_front().unwrap();
        let dst = apod.dest();
        let coeff = apod.coeff();

        for idx in [0, 1, 3, 5, 7, 9, 10] {
            if (src != dst || !rooms[i].iter().all(|&a| a == apod))
                && hall[src.min(idx)..=src.max(idx)]
                    .iter()
                    .all(|a| a.is_none())
            {
                hall[idx] = Some(apod);
                let tot = dfs(
                    rooms,
                    hall,
                    best,
                    score
                        + coeff * (src.max(idx) - src.min(idx) + cap - rooms[i].len()),
                    cap,
                );
                totals.push(tot);
                hall[idx] = None;
            }
        }

        rooms[i].push_front(apod);
    }

    totals.into_iter().min().unwrap_or(usize::MAX)
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let mut rms = [
            VecDeque::with_capacity(2),
            VecDeque::with_capacity(2),
            VecDeque::with_capacity(2),
            VecDeque::with_capacity(2),
        ];

        input.skip(2).take(2).for_each(|line| {
            line.split('#')
                .filter(|el| !el.trim().is_empty())
                .enumerate()
                .for_each(|(i, l)| rms[i].push_back(Apod::from(l)))
        });

        let mut hall = [None; 11];
        let mut best = BinaryHeap::new();

        dfs(&mut rms, &mut hall, &mut best, 0, 2);
        best.pop().unwrap()
    }

    fn solve_b(&self, input: Lines) -> usize {
        let mut rms = [
            VecDeque::with_capacity(4),
            VecDeque::with_capacity(4),
            VecDeque::with_capacity(4),
            VecDeque::with_capacity(4),
        ];

        input
            .skip(2)
            .take(2)
            .enumerate()
            .flat_map(|(i, line)| {
                if i == 0 {
                    [line, "  #D#C#B#A#"]
                } else {
                    ["  #D#B#A#C#", line]
                }
            })
            .for_each(|line| {
                line.split('#')
                    .filter(|el| !el.trim().is_empty())
                    .enumerate()
                    .for_each(|(i, l)| rms[i].push_back(Apod::from(l)))
            });

        let mut hall = [None; 11];
        let mut best = BinaryHeap::new();

        dfs(&mut rms, &mut hall, &mut best, 0, 4);
        best.pop().unwrap()
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 23);
    let solver = Solver {};

    match aoc::earn_star(problem.part_b(), solver) {
        Ok(resp) => println!("Response: {}", resp),
        Err(err) => panic!("Something went wrong: {}", err),
    }
}

// Need to run these tests in --release mode to speed them up
#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::StripMargin;

    #[test]
    fn test_example() {
        let solver = Solver {};
        let input = r"
            |#############
            |#...........#
            |###B#C#B#D###
            |  #A#D#C#A#
            |  #########
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 12521);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 44169);
    }

    #[test]
    fn test_solution() {
        let solver = Solver {};
        let input = r"
            |#############
            |#...........#
            |###C#C#A#B###
            |  #D#D#B#A#
            |  #########
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 18282);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 50132);
    }
}
