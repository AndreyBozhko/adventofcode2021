use adventofcode::{self as aoc, aoc_problem, map, set, Solution};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::Lines;

trait SortedString {
    fn as_sorted_string(&self) -> String;
}

impl SortedString for HashSet<char> {
    fn as_sorted_string(&self) -> String {
        self.iter().sorted().collect()
    }
}

impl SortedString for &str {
    fn as_sorted_string(&self) -> String {
        self.chars().sorted().collect()
    }
}

struct Solver;

fn decode(inputs: Vec<String>, outputs: Vec<String>) -> usize {
    let input_sets: Vec<HashSet<_>> =
        inputs.iter().map(|el| el.chars().collect()).collect();

    let d_1 = input_sets.iter().find(|el| el.len() == 2).unwrap();
    let d_4 = input_sets.iter().find(|el| el.len() == 4).unwrap();
    let d_7 = input_sets.iter().find(|el| el.len() == 3).unwrap();
    let d_8 = input_sets.iter().find(|el| el.len() == 7).unwrap();
    let d_3 = input_sets
        .iter()
        .find(|el| el.len() == 5 && el.is_superset(d_1))
        .unwrap();
    let d_6 = input_sets
        .iter()
        .find(|el| el.len() == 6 && !el.is_superset(d_1))
        .unwrap();
    let d_9 = input_sets
        .iter()
        .find(|el| el.len() == 6 && el.is_superset(d_3))
        .unwrap();
    let d_0 = input_sets
        .iter()
        .find(|el| el.len() == 6 && el != &d_6 && el != &d_9)
        .unwrap();
    let d_5 = input_sets
        .iter()
        .find(|el| el.len() == 5 && el != &d_3 && el.is_subset(d_9))
        .unwrap();
    let d_2 = input_sets
        .iter()
        .find(|el| el.len() == 5 && el != &d_3 && el != &d_5)
        .unwrap();

    let mapping = map![
        d_0.as_sorted_string() => 0,
        d_1.as_sorted_string() => 1,
        d_2.as_sorted_string() => 2,
        d_3.as_sorted_string() => 3,
        d_4.as_sorted_string() => 4,
        d_5.as_sorted_string() => 5,
        d_6.as_sorted_string() => 6,
        d_7.as_sorted_string() => 7,
        d_8.as_sorted_string() => 8,
        d_9.as_sorted_string() => 9,
    ];

    outputs
        .iter()
        .map(|el| mapping.get(el).unwrap())
        .zip([1000, 100, 10, 1])
        .map(|(&digit, power)| digit * power)
        .sum()
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let output: Vec<_> = input
            .flat_map(|line| line.split_once('|').unwrap().1.split_whitespace())
            .collect();

        let sizes: HashSet<usize> = set![2, 3, 4, 7];
        output.iter().filter(|el| sizes.contains(&el.len())).count()
    }

    fn solve_b(&self, input: Lines) -> usize {
        input
            .map(|line| line.split_once('|').unwrap())
            .map(|(left, right)| {
                (
                    left.trim().split_whitespace().map(str::to_string).collect(),
                    right
                        .trim()
                        .split_whitespace()
                        .map(|s| s.as_sorted_string())
                        .collect(),
                )
            })
            .map(|(left, right)| decode(left, right))
            .sum()
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 8);
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
            > be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            > edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            > fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            > fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            > aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            > fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            > dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            > bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            > egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            > gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "
        .trim()
        .strip_margin_of("> ");

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 26);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 61229);
    }
}
