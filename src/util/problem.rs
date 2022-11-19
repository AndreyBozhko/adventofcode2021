use std::fmt::Display;
use std::str::Lines;

/// Advent of Code problems have parts A and B.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Part {
    A,
    B,
}

/// Metadata for Advent of Code problem - year, day, and part.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Problem {
    pub(crate) year: u32,
    pub(crate) day: u32,
    pub(crate) part: Option<Part>,
}

impl Problem {
    /// Instantiate a Problem for a given year and day.
    pub fn new(year: u32, day: u32) -> Self {
        assert!((2015..=2021).contains(&year), "Year out of range: {}", year);
        assert!((1..=25).contains(&day), "Day out of range: {}", day);
        Self {
            year,
            day,
            part: None,
        }
    }

    /// Return part A of this problem.
    pub fn part_a(self) -> Self {
        Self {
            part: Some(Part::A),
            ..self
        }
    }

    /// Return part B of this problem.
    pub fn part_b(self) -> Self {
        Self {
            part: Some(Part::B),
            ..self
        }
    }
}

/// Trait implemented by all Advent of Code problem solvers.
pub trait Solution {
    type Output: Display + Sized;
    /// Compute answer for the Advent of Code problem (part A).
    fn solve_a(&self, input: Lines) -> Self::Output;
    /// Compute answer for the Advent of Code problem (part B).
    fn solve_b(&self, input: Lines) -> Self::Output;
}

/// Macro to instantiate Problem for a given year and day.
///
/// # Examples
/// ```
/// # use adventofcode::aoc_problem;
///
/// let problem = aoc_problem!(year = 2021, day = 25);
/// ```
/// Note that the macro expects key-value styled arguments - otherwise macro fails to compile.
/// ```compile_fail
/// # use adventofcode::aoc_problem;
///
/// let _ = aoc_problem!(2021, 25); // shouldn't compile
/// ```
#[macro_export]
macro_rules! aoc_problem {
    (year=$y:expr, day=$d:expr) => {
        adventofcode::util::problem::Problem::new($y, $d)
    };
    ($($_:tt)*) => {
        compile_error!("Expected usage: aoc_problem!(year=..., day=...)");
    };
}
