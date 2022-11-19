use adventofcode::{self as aoc, aoc_problem, Solution};
use itertools::Itertools;
use std::collections::{LinkedList, VecDeque};
use std::fmt::Debug;
use std::ops::Add;
use std::str::Lines;

#[derive(Clone, Debug)]
struct SFNum {
    value: LinkedList<(usize, u32)>,
}

impl Add for SFNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ll = LinkedList::new();
        for part in [&self.value, &rhs.value] {
            part.iter().for_each(|&(lvl, val)| {
                ll.push_back((lvl + 1, val));
            });
        }
        while Self::explode(&mut ll) || Self::split(&mut ll) {}
        Self { value: ll }
    }
}

impl From<&str> for SFNum {
    fn from(input: &str) -> Self {
        let mut lvl = 0;
        let mut list = LinkedList::new();
        for c in input.chars() {
            match c {
                '[' => {
                    lvl += 1;
                }
                ']' => {
                    lvl -= 1;
                }
                ',' => {}
                c => {
                    list.push_back((lvl, c.to_digit(10).unwrap()));
                }
            }
        }
        Self { value: list }
    }
}

impl SFNum {
    fn magnitude(&self) -> u32 {
        let mut queue = VecDeque::new();

        fn reduce(q: &mut VecDeque<(usize, u32)>) -> bool {
            if q.len() < 2 {
                return false;
            }
            let (right, left) = (q.pop_back().unwrap(), q.pop_back().unwrap());
            if left.0 == right.0 {
                q.push_back((left.0 - 1, 3 * left.1 + 2 * right.1));
                true
            } else {
                q.push_back(left);
                q.push_back(right);
                false
            }
        }

        for &el in &self.value {
            queue.push_back(el);
            while reduce(&mut queue) {}
        }

        queue.pop_back().unwrap().1
    }

    fn split(list: &mut LinkedList<(usize, u32)>) -> bool {
        let pos = list.iter().find_position(|&&(_, el)| el >= 10);
        match pos {
            None => false,
            Some((pos, _)) => {
                let mut split = list.split_off(pos);
                let el = split.pop_front().unwrap();
                list.push_back((el.0 + 1, el.1 / 2));
                list.push_back((el.0 + 1, el.1 - el.1 / 2));
                list.extend(split);
                true
            }
        }
    }

    fn explode(list: &mut LinkedList<(usize, u32)>) -> bool {
        let pos = list.iter().find_position(|&&(lvl, _)| lvl > 4);
        match pos {
            None => false,
            Some((pos, _)) => {
                let mut split = list.split_off(pos);

                let (left, right) =
                    (split.pop_front().unwrap(), split.pop_front().unwrap());
                if let Some(el) = list.pop_back() {
                    list.push_back((el.0, el.1 + left.1));
                }
                list.push_back((left.0 - 1, 0));
                if let Some(el) = split.pop_front() {
                    split.push_front((el.0, el.1 + right.1));
                }
                list.extend(split);
                true
            }
        }
    }
}

struct Solver;

impl Solver {
    fn parse_input(&self, input: Lines) -> Vec<SFNum> {
        input.map(SFNum::from).collect()
    }
}

impl Solution for Solver {
    type Output = u32;

    fn solve_a(&self, input: Lines) -> u32 {
        let nums = self.parse_input(input);

        let mut it = nums.into_iter();
        let mut res = it.next().unwrap();
        for n in it {
            res = res + n;
        }

        res.magnitude()
    }

    fn solve_b(&self, input: Lines) -> u32 {
        let nums = self.parse_input(input);

        (0..nums.len())
            .cartesian_product(0..nums.len())
            .filter(|&(i, j)| i != j)
            .map(|(i, j)| (nums[i].clone() + nums[j].clone()).magnitude())
            .max()
            .unwrap()
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 18);
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
    fn test_example_1() {
        let solver = Solver {};
        let input = r"
            > [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            > [[[5,[2,8]],4],[5,[[9,9],0]]]
            > [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            > [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            > [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            > [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            > [[[[5,4],[7,7]],8],[[8,3],8]]
            > [[9,3],[[9,9],[6,[4,9]]]]
            > [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            > [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        "
        .trim()
        .strip_margin_of("> ");

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 4140);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 3993);
    }
}
