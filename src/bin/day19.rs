use adventofcode::{self as aoc, aoc_problem, set, Solution};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::ops::Sub;
use std::str::Lines;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Coord {
    fn dist(&self, other: &Self) -> (i32, i32) {
        (
            (self.x - other.x).abs()
                + (self.y - other.y).abs()
                + (self.z - other.z).abs(),
            (self.x - other.x).pow(2)
                + (self.y - other.y).pow(2)
                + (self.z - other.z).pow(2),
        )
    }

    fn rotate(&self, how: usize) -> Coord {
        let Coord { x, y, z } = *self;
        let rotations = [
            // around x
            (x, y, z),
            (x, z, -y),
            (x, -y, -z),
            (x, -z, y),
            (-x, -y, z),
            (-x, z, y),
            (-x, y, -z),
            (-x, -z, -y),
            // around y
            (-y, x, z),
            (-y, z, -x),
            (-y, -x, -z),
            (-y, -z, x),
            (y, -x, z),
            (y, z, x),
            (y, x, -z),
            (y, -z, -x),
            // around z
            (z, y, -x),
            (z, -x, -y),
            (z, -y, x),
            (z, x, y),
            (-z, y, x),
            (-z, x, -y),
            (-z, -y, -x),
            (-z, -x, y),
        ];

        let (nx, ny, nz) = rotations[how];
        Coord {
            x: nx,
            y: ny,
            z: nz,
        }
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Coord>,
}

impl Scanner {
    fn rotate_and_translate_points(&mut self, how: usize, ddd: Coord) {
        self.beacons.iter_mut().for_each(|b| {
            *b = b.rotate(how) - ddd;
        });
    }
}

struct Solver;

impl Solver {
    fn parse_input(&self, mut input: Lines) -> Vec<Scanner> {
        let mut scanners = vec![];
        while let Some(_) = input.next() {
            let mut beacons = vec![];
            for line in input.by_ref() {
                if line.is_empty() {
                    break;
                }
                let p: Vec<_> = line
                    .split(',')
                    .map(|el| el.parse::<i32>().unwrap())
                    .collect();
                beacons.push(Coord {
                    x: p[0],
                    y: p[1],
                    z: p[2],
                })
            }
            scanners.push(Scanner { beacons })
        }
        scanners
    }

    fn solve(&self, scanners: &mut Vec<Scanner>) -> Vec<(usize, Coord)> {
        let mut seen = set![];
        let mut queue = VecDeque::from([0]);
        let mut abs_positions = vec![(0, Coord { x: 0, y: 0, z: 0 })];

        while seen.len() < scanners.len() {
            let left = queue.pop_front().unwrap();
            seen.insert(left);
            for right in 0..scanners.len() {
                if seen.contains(&right) {
                    continue;
                }
                if let Some((how, dxdydz)) =
                    try_align(&scanners[left], &scanners[right])
                {
                    scanners[right].rotate_and_translate_points(how, dxdydz);
                    queue.push_back(right);
                    abs_positions.push((right, dxdydz));
                }
            }
        }
        abs_positions
    }
}

fn try_align(s1: &Scanner, s2: &Scanner) -> Option<(usize, Coord)> {
    let mut pts1 = set![];
    let mut pts2 = set![];
    let mut common = vec![];
    for i1 in 0..s1.beacons.len() {
        for j1 in i1 + 1..s1.beacons.len() {
            for i2 in 0..s2.beacons.len() {
                for j2 in i2 + 1..s2.beacons.len() {
                    if s1.beacons[i1].dist(&s1.beacons[j1])
                        == s2.beacons[i2].dist(&s2.beacons[j2])
                    {
                        pts1.insert(i1);
                        pts1.insert(j1);
                        pts2.insert(i2);
                        pts2.insert(j2);
                        common.push((i1, j1, i2, j2));
                    }
                }
            }
        }
    }
    if pts1.len() < 12 || pts2.len() < 12 {
        return None;
    }

    let chosen = common[0];
    for how in 0..24 {
        let p0 = s1.beacons[chosen.0];
        let p1 = s1.beacons[chosen.1];
        let p2 = s2.beacons[chosen.2].rotate(how);
        let p3 = s2.beacons[chosen.3].rotate(how);
        if p0 - p1 == p2 - p3 {
            return Some((how, p2 - p0));
        }
        if p0 - p1 == p3 - p2 {
            return Some((how, p3 - p0));
        }
    }

    None
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        let mut scanners = self.parse_input(input);

        let _ = self.solve(&mut scanners);

        let all_points = scanners
            .iter()
            .flat_map(|s| s.beacons.iter().copied())
            .collect::<HashSet<_>>();

        all_points.len()
    }

    fn solve_b(&self, input: Lines) -> usize {
        let mut scanners = self.parse_input(input);

        let abs_positions = self.solve(&mut scanners);

        abs_positions
            .iter()
            .cartesian_product(abs_positions.iter())
            .map(|((_, p1), (_, p2))| p1.dist(p2).0)
            .max()
            .unwrap() as usize
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 19);
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
            |--- scanner 0 ---
            |404,-588,-901
            |528,-643,409
            |-838,591,734
            |390,-675,-793
            |-537,-823,-458
            |-485,-357,347
            |-345,-311,381
            |-661,-816,-575
            |-876,649,763
            |-618,-824,-621
            |553,345,-567
            |474,580,667
            |-447,-329,318
            |-584,868,-557
            |544,-627,-890
            |564,392,-477
            |455,729,728
            |-892,524,684
            |-689,845,-530
            |423,-701,434
            |7,-33,-71
            |630,319,-379
            |443,580,662
            |-789,900,-551
            |459,-707,401
            |
            |--- scanner 1 ---
            |686,422,578
            |605,423,415
            |515,917,-361
            |-336,658,858
            |95,138,22
            |-476,619,847
            |-340,-569,-846
            |567,-361,727
            |-460,603,-452
            |669,-402,600
            |729,430,532
            |-500,-761,534
            |-322,571,750
            |-466,-666,-811
            |-429,-592,574
            |-355,545,-477
            |703,-491,-529
            |-328,-685,520
            |413,935,-424
            |-391,539,-444
            |586,-435,557
            |-364,-763,-893
            |807,-499,-711
            |755,-354,-619
            |553,889,-390
            |
            |--- scanner 2 ---
            |649,640,665
            |682,-795,504
            |-784,533,-524
            |-644,584,-595
            |-588,-843,648
            |-30,6,44
            |-674,560,763
            |500,723,-460
            |609,671,-379
            |-555,-800,653
            |-675,-892,-343
            |697,-426,-610
            |578,704,681
            |493,664,-388
            |-671,-858,530
            |-667,343,800
            |571,-461,-707
            |-138,-166,112
            |-889,563,-600
            |646,-828,498
            |640,759,510
            |-630,509,768
            |-681,-892,-333
            |673,-379,-804
            |-742,-814,-386
            |577,-820,562
            |
            |--- scanner 3 ---
            |-589,542,597
            |605,-692,669
            |-500,565,-823
            |-660,373,557
            |-458,-679,-417
            |-488,449,543
            |-626,468,-788
            |338,-750,-386
            |528,-832,-391
            |562,-778,733
            |-938,-730,414
            |543,643,-506
            |-524,371,-870
            |407,773,750
            |-104,29,83
            |378,-903,-323
            |-778,-728,485
            |426,699,580
            |-438,-605,-362
            |-469,-447,-387
            |509,732,623
            |647,635,-688
            |-868,-804,481
            |614,-800,639
            |595,780,-596
            |
            |--- scanner 4 ---
            |727,592,562
            |-293,-554,779
            |441,611,-461
            |-714,465,-776
            |-743,427,-804
            |-660,-479,-426
            |832,-632,460
            |927,-485,-438
            |408,393,-506
            |466,436,-512
            |110,16,151
            |-258,-428,682
            |-393,719,612
            |-211,-452,876
            |808,-476,-593
            |-575,615,604
            |-485,667,467
            |-680,325,-822
            |-627,-443,-432
            |872,-547,-609
            |833,512,582
            |807,604,487
            |839,-516,451
            |891,-625,532
            |-652,-548,-490
            |30,-46,-14
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 79);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 3621);
    }
}
