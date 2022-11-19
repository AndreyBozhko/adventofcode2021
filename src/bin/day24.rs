use adventofcode::{self as aoc, aoc_problem, Solution};
use std::collections::HashSet;
use std::str::Lines;
// use std::fmt;
// use std::rc::Rc;
// use itertools::Itertools;
//
// #[derive(Clone, Debug)]
// enum Op {
//     Add,
//     Mul,
//     Div,
//     Rem,
//     Eql,
// }
//
// impl fmt::Display for Op {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         use Op::*;
//         match self {
//             Add => write!(f, "+"),
//             Mul => write!(f, "*"),
//             Div => write!(f, "/"),
//             Rem => write!(f, "%"),
//             Eql => write!(f, "="),
//         }
//     }
// }
//
// #[derive(Clone, Debug)]
// enum Cons {
//     Lit(i64),
//     Var(String),
//     Expr(Op, Rc<Cons>, Rc<Cons>),
// }
//
// impl Cons {
//     fn new_expr(op: Op, left: Cons, right: Cons) -> Cons {
//         use Cons::*;
//         use Op::*;
//         match (op, left, right) {
//             (op, Lit(l), Lit(r)) => match op {
//                 Add => Lit(l + r),
//                 Mul => Lit(l * r),
//                 Div => Lit(l / r),
//                 Rem => Lit(l % r),
//                 Eql => Lit(if l == r { 1 } else { 0 }),
//             },
//             // eql (vars are always 1 <= v <= 9)
//             (Eql, Lit(v), Var(_)) if !(1..=9).contains(&v) => Lit(0),
//             (Eql, Var(_), Lit(v)) if !(1..=9).contains(&v) => Lit(0),
//             // mul by 0
//             (Mul, Lit(0), _) => Lit(0),
//             (Mul, _, Lit(0)) => Lit(0),
//             // mul by 1
//             (Mul, l, Lit(1)) => l,
//             (Mul, Lit(1), r) => r,
//             // add 0
//             (Add, l, Lit(0)) => l,
//             (Add, Lit(0), r) => r,
//             // else
//             (op, l, r) => Expr(op, Rc::new(l), Rc::new(r)),
//         }
//     }
// }
//
// impl fmt::Display for Cons {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Cons::Lit(a) => write!(f, "{}", a),
//             Cons::Var(w) => write!(f, "{}", w),
//             Cons::Expr(op, left, right) => {
//                 write!(f, "({} {} {})", op, left, right)
//             }
//         }
//     }
// }
//
// struct Alu<'a> {
//     instructions: &'a [&'a str],
//     registers: HashMap<&'static str, Cons>,
// }
//
// impl fmt::Display for Alu<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "ALU: w={}  x={}  y={}  z={}",
//             self.registers["w"],
//             self.registers["x"],
//             self.registers["y"],
//             self.registers["z"]
//         )
//     }
// }
//
// impl<'a> Alu<'a> {
//     fn new(instructions: &'a [&'a str]) -> Self {
//         Self {
//             instructions,
//             registers: map![
//                 "w" => Cons::Lit(0),
//                 "x" => Cons::Lit(0),
//                 "y" => Cons::Lit(0),
//                 "z" => Cons::Lit(0)],
//         }
//     }
//     fn eval(&mut self, inputs: VecDeque<Cons>) -> Cons {
//         let mut inps = inputs;
//
//         for line in self.instructions {
//             let (op, rest) = line.split_once(' ').unwrap();
//             match op {
//                 "inp" => match inps.pop_front() {
//                     None => {
//                         return self.registers["z"].clone();
//                     }
//                     Some(v) => {
//                         let reg = match rest {
//                             "w" => "w",
//                             "x" => "x",
//                             "y" => "y",
//                             "z" => "z",
//                             _ => unreachable!()
//                         };
//                         self.registers.insert(reg, v);
//                     }
//                 },
//                 op => {
//                     let (reg, val) = rest.split_once(' ').unwrap();
//                     let val = self
//                         .registers
//                         .get(val)
//                         .cloned()
//                         .unwrap_or_else(|| Cons::Lit(val.parse().unwrap()));
//                     let reg = self.registers.get_mut(reg).unwrap();
//                     let op = match op {
//                         "add" => Op::Add,
//                         "mul" => Op::Mul,
//                         "div" => Op::Div,
//                         "mod" => Op::Rem,
//                         "eql" => Op::Eql,
//                         _ => panic!(),
//                     };
//                     *reg = Cons::new_expr(op, reg.clone(), val);
//                 }
//             }
//         }
//
//         self.registers["z"].clone()
//     }
// }

fn get_model<T>(search_range: T) -> usize
where
    T: Clone + Iterator<Item = i8>,
{
    // d1 + 4 = dE
    // d2 + 6 = dD
    // d3 - 6 = dC
    // d4 - 1 = d9
    // d5 + 0 = d6
    // d7 - 4 = d8
    // dA + 7 = dB

    let mut digits = [0; 15];
    let allowed: HashSet<_> = search_range.clone().collect();

    for ch in search_range {
        for pos in 1..=14 {
            if digits[pos] != 0 {
                continue;
            }
            if pos == 1 && allowed.contains(&(ch + 4)) {
                digits[pos] = ch;
                digits[14] = ch + 4;
            }
            if pos == 2 && allowed.contains(&(ch + 6)) {
                digits[pos] = ch;
                digits[13] = ch + 6;
            }
            if pos == 3 && allowed.contains(&(ch - 6)) {
                digits[pos] = ch;
                digits[12] = ch - 6;
            }
            if pos == 4 && allowed.contains(&(ch - 1)) {
                digits[pos] = ch;
                digits[9] = ch - 1;
            }
            if pos == 5 && allowed.contains(&ch) {
                digits[pos] = ch;
                digits[6] = ch;
            }
            if pos == 7 && allowed.contains(&(ch - 4)) {
                digits[pos] = ch;
                digits[8] = ch - 4;
            }
            if pos == 10 && allowed.contains(&(ch + 7)) {
                digits[pos] = ch;
                digits[11] = ch + 7;
            }
        }
    }

    digits
        .into_iter()
        .rev()
        .zip(0..)
        .map(|(d, p)| d as usize * 10_usize.pow(p))
        .sum()
}

struct Solver;

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, _input: Lines) -> usize {
        get_model((1..=9).rev())
    }

    fn solve_b(&self, _input: Lines) -> usize {
        get_model(1..=9)
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 24);
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
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 1
            |add x 15
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 13
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 1
            |add x 10
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 16
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 1
            |add x 12
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 2
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 1
            |add x 10
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 8
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 1
            |add x 14
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 11
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 26
            |add x -11
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 6
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 1
            |add x 10
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 12
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 26
            |add x -16
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 2
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 26
            |add x -9
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 2
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 1
            |add x 11
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 15
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 26
            |add x -8
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 1
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 26
            |add x -8
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 10
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 26
            |add x -10
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 14
            |mul y x
            |add z y
            |inp w
            |mul x 0
            |add x z
            |mod x 26
            |div z 26
            |add x -9
            |eql x w
            |eql x 0
            |mul y 0
            |add y 25
            |mul y x
            |add y 1
            |mul z y
            |mul y 0
            |add y w
            |add y 10
            |mul y x
            |add z y
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 53_999_995_829_399);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 11_721_151_118_175);
    }
}
