use crate::{
    make_err,
    util::problem::{Part, Problem},
    Solution,
};
use std::{env, fmt, fs, path, result};

const BASE_URL: &str = "https://adventofcode.com";

type ClientResult<T> = result::Result<T, crate::Error>;

/// Client to fetch problem inputs and submit answers for the Advent of Code challenge.
///
/// # Example
///
/// ```no_run
/// # use adventofcode::{self as aoc, aoc_problem, util::problem::Problem};
/// # fn solve(_: String) -> i32 { 42 }
///
/// fn solve_day(client: &aoc::Client, problem: Problem) -> Result<String, aoc::Error> {
///     let input = client.get_problem_input(problem)?;
///     let answer = solve(input);
///     let status = client.submit_answer(problem.part_a(), answer)?;
///     Ok(status)
/// }
///
/// let client = aoc::Client::try_new().unwrap();
/// # let problem = aoc_problem!(year=2020, day=25);
/// assert_eq!(solve_day(&client, problem).unwrap(), "Correct!");
/// ```
pub struct Client {
    cookie: String,
}

impl Client {
    /// Attempts to instantiate a new client.
    ///
    /// Returns `Err` if the session cookie is not set via the environment variable.
    pub fn try_new() -> ClientResult<Self> {
        let cookie = format!("session={}", env::var("AOC_TOKEN")?);
        Ok(Self { cookie })
    }

    /// Gets input for a specific problem and caches it locally.
    pub fn get_problem_input(&self, problem: Problem) -> ClientResult<String> {
        let pth: path::PathBuf = [
            env::var("CACHE_DIR")?,
            problem.year.to_string(),
            format!("{:0width$}.txt", problem.day, width = 2),
        ]
        .iter()
        .collect();

        if !pth.exists() {
            let url =
                format!("{}/{}/day/{}/input", BASE_URL, problem.year, problem.day);

            let resp = ureq::get(&url)
                .set("cookie", &self.cookie)
                .call()?
                .into_string()?;

            fs::create_dir_all(&pth.parent().unwrap())?;
            fs::write(&pth, resp)?;
        }

        let input = fs::read_to_string(&pth)?;
        Ok(input)
    }

    /// Submits an answer to a specific problem.
    pub fn submit_answer(
        &self,
        problem: Problem,
        answer: impl fmt::Display,
    ) -> ClientResult<String> {
        let url = format!("{}/{}/day/{}/answer", BASE_URL, problem.year, problem.day);

        let lvl = match problem.part {
            Some(Part::A) => "1",
            Some(Part::B) => "2",
            None => {
                return make_err!("Missing problem part");
            }
        };

        let resp = ureq::post(&url)
            .set("cookie", &self.cookie)
            .send_form(&[("level", lvl), ("answer", &answer.to_string())])?
            .into_string()?;

        let expected = "That's the right answer!";
        if resp.contains(expected) {
            Ok(expected.to_string())
        } else {
            make_err!("{}", resp)
        }
    }
}

/// Earns a star by fetching a problem input and submitting an answer.
pub fn earn_star<T: Solution>(problem: Problem, solver: T) -> ClientResult<String> {
    let client = Client::try_new().expect("AOC_TOKEN is required to use the client");
    let input = client.get_problem_input(problem)?;

    match problem.part {
        None => {
            make_err!("Can't solve a problem if no part is specified :(")
        }
        Some(part) => {
            let ans = match part {
                Part::A => solver.solve_a(input.lines()),
                Part::B => solver.solve_b(input.lines()),
            };
            println!("{}", &ans);

            let resp = client.submit_answer(problem, ans)?;
            Ok(resp)
        }
    }
}
