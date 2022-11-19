use adventofcode::{self as aoc, aoc_problem, make_err, map, Solution};
use itertools::Itertools;
use std::collections::HashMap;
use std::str::Lines;

#[derive(Clone, Copy, Debug)]
enum Pixel {
    Dark,
    Light,
}

type Image = HashMap<(i32, i32), Pixel>;

impl Pixel {
    fn value(&self) -> usize {
        match self {
            Pixel::Dark => 0,
            Pixel::Light => 1,
        }
    }
}

impl TryFrom<char> for Pixel {
    type Error = aoc::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Pixel::Dark),
            '#' => Ok(Pixel::Light),
            _ => make_err!("Pixel not recognized: {}", c),
        }
    }
}

struct Solver;

impl Solver {
    fn parse_input(&self, mut input: Lines) -> (Vec<Pixel>, Image) {
        let mut algo = vec![];
        for line in input.by_ref() {
            if line.is_empty() {
                break;
            }
            algo.extend(line.chars().map(|c| Pixel::try_from(c).unwrap()));
        }

        let m: Image = input
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().map(move |(j, c)| {
                    ((i as i32, j as i32), Pixel::try_from(c).unwrap())
                })
            })
            .fold(map![], |mut m, (k, v)| {
                m.insert(k, v);
                m
            });

        (algo, m)
    }
}

fn transform(mut image: Image, algo: &[Pixel], steps: usize) -> Image {
    assert_eq!(algo.len(), 512);
    assert!(matches!(algo[0], Pixel::Dark) || matches!(algo[511], Pixel::Dark));

    let mut upd = map![];

    let mut left = 0;
    let mut right = image.iter().map(|(&(x, _), _)| x).max().unwrap();

    for step in 0..steps {
        let default = if step % 2 == 1 { algo[0] } else { Pixel::Dark };

        left -= 1;
        right += 1;

        for x in left..=right {
            for y in left..=right {
                let idx = (-1..=1)
                    .cartesian_product(-1..=1)
                    .map(|(dx, dy)| {
                        image.get(&(x + dx, y + dy)).unwrap_or(&default).value()
                    })
                    .zip(0..9)
                    .map(|(val, k)| 2_usize.pow(8 - k) * val)
                    .sum::<usize>();
                upd.insert((x, y), algo.get(idx).copied().unwrap());
            }
        }
        image = upd;
        upd = map![];
    }

    image
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let (algo, image) = self.parse_input(input);
        let image = transform(image, &algo, 2);
        image
            .iter()
            .filter(|&(_, &p)| matches!(p, Pixel::Light))
            .count()
    }

    fn solve_b(&self, input: Lines) -> usize {
        let (algo, image) = self.parse_input(input);
        let image = transform(image, &algo, 50);
        image
            .iter()
            .filter(|&(_, &p)| matches!(p, Pixel::Light))
            .count()
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 20);
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
            |..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
            |#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
            |.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
            |.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
            |.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
            |...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
            |..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
            |
            |#..#.
            |#....
            |##..#
            |..#..
            |..###
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 35);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 3351);
    }
}
