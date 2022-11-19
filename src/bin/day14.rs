use adventofcode::{self as aoc, aoc_problem, map, Solution};
use std::collections::HashMap;
use std::str::Lines;

type Pair = [char; 2];

struct Solver;

impl Solver {
    fn parse_input(&self, mut input: Lines) -> (String, HashMap<Pair, Vec<Pair>>) {
        let template = input.next().unwrap().to_string();
        input.next();

        let mut rules0 = map![];

        for line in input {
            let (from, to) = line.split_once(" -> ").expect("Expect one delimiter");
            let fr: Pair = [from.chars().next().unwrap(), from.chars().nth(1).unwrap()];
            rules0.insert(fr, to.chars().next().unwrap());
        }

        let mut rules: HashMap<Pair, Vec<Pair>> = map![];
        for (&k, &v) in rules0.iter() {
            for vv in [[k[0], v], [v, k[1]]] {
                if rules0.contains_key(&vv) {
                    rules.entry(k).or_default().push(vv);
                }
            }
        }
        (template, rules)
    }
}

fn find_quantities(
    template: String,
    rules: HashMap<Pair, Vec<Pair>>,
    iterations: usize,
) -> usize {
    let initial = template.chars().collect::<Vec<_>>();

    let odd = [
        initial.first().copied().unwrap(),
        initial.last().copied().unwrap(),
    ];
    let mut initial: HashMap<Pair, usize> = initial
        .windows(2)
        .map(|w| [w[0], w[1]])
        .fold(map![], |mut m, el| {
            *m.entry(el).or_default() += 1;
            m
        });

    for _ in 0..iterations {
        let updated: HashMap<Pair, usize> = initial
            .iter()
            .flat_map(|(&k, &v)| rules.get(&k).unwrap().iter().map(move |&el| (el, v)))
            .fold(map![], |mut m, (k, v)| {
                *m.entry(k).or_default() += v;
                m
            });
        initial = updated;
    }

    let counts =
        initial
            .iter()
            .fold(map![odd[0] => 1, odd[1] => 1], |mut m, (k, &v)| {
                *m.entry(k[0]).or_default() += v;
                *m.entry(k[1]).or_default() += v;
                m
            });

    let minc = counts.values().min().copied().unwrap();
    let maxc = counts.values().max().copied().unwrap();

    (maxc - minc) / 2
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let (template, rules) = self.parse_input(input);
        find_quantities(template, rules, 10)
    }

    fn solve_b(&self, input: Lines) -> usize {
        let (template, rules) = self.parse_input(input);
        find_quantities(template, rules, 40)
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 14);
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
            |NNCB
            |
            |CH -> B
            |HH -> N
            |CB -> H
            |NH -> C
            |HB -> C
            |HC -> B
            |HN -> C
            |NN -> C
            |BH -> H
            |NC -> B
            |NB -> B
            |BN -> B
            |BB -> N
            |BC -> B
            |CC -> N
            |CN -> C
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 1588);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 2_188_189_693_529);
    }
}
