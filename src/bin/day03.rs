use adventofcode::{self as aoc, aoc_problem, Solution};
use std::str::Lines;

struct Solver;

impl Solution for Solver {
    type Output = u32;

    fn solve_a(&self, input: Lines) -> u32 {
        let nums: Vec<_> = input
            .map(|line| (line.len(), u32::from_str_radix(line, 2).unwrap()))
            .collect();
        let sz = nums[0].0;

        let gamma: u32 = (0..sz)
            .map(|shift| 1_u32 << shift)
            .map(|pow| {
                nums.iter().filter(|&&(_, n)| n & pow > 0).count() >= nums.len() / 2
            })
            .zip(0..sz)
            .map(
                |(bit_non_empty, y)| {
                    if bit_non_empty {
                        1_u32 << y as u32
                    } else {
                        0
                    }
                },
            )
            .sum();

        gamma * ((1_u32 << sz) - gamma - 1)
    }

    fn solve_b(&self, input: Lines) -> u32 {
        let nums: Vec<_> = input
            .map(|line| (line.len(), u32::from_str_radix(line, 2).unwrap()))
            .collect();
        let sz = nums[0].0;

        let nums1: Vec<_> = nums.iter().map(|&x| x.1).collect();
        let nums2 = nums1.clone();

        let oxy = find_rating(nums1, sz, |x, y| x >= y);
        let co2 = find_rating(nums2, sz, |x, y| x < y);
        oxy * co2
    }
}

fn find_rating<Pred>(mut nums: Vec<u32>, sz: usize, predicate: Pred) -> u32
where
    Pred: Fn(usize, usize) -> bool,
{
    for pow in (0..sz).rev() {
        let pw = 1_u32 << pow;
        let (one, zero): (Vec<_>, Vec<_>) = nums.iter().partition(|&&x| x & pw > 0);

        if predicate(one.len(), zero.len()) {
            nums = one;
        } else {
            nums = zero;
        }

        if nums.len() == 1 {
            return nums[0];
        }
    }
    nums[0]
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 3);
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
            |00100
            |11110
            |10110
            |10111
            |10101
            |01111
            |00111
            |11100
            |10000
            |11001
            |00010
            |01010
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 198);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 230);
    }
}
