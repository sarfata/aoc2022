use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(PartialEq, Debug, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
struct RPSRound {
    opponent: RPS,
    you: RPS,
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn from(s: &str) -> RoundResult {
        match s {
            "X" => RoundResult::Lose,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            &_ => panic!("Invalid input {s}"),
        }
    }
}

impl RPS {
    fn from(s: &str) -> RPS {
        match s {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissor,
            &_ => panic!("Invalid input {s}"),
        }
    }

    fn beats(&self, other: &RPS) -> bool {
        // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
        match self {
            RPS::Rock => other == &RPS::Scissor,
            RPS::Scissor => other == &RPS::Paper,
            RPS::Paper => other == &RPS::Rock,
        }
    }

    fn response_for(&self, r: RoundResult) -> RPS {
        match r {
            RoundResult::Draw => *self,
            // We want to win ...
            RoundResult::Win => match self {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissor,
                RPS::Scissor => RPS::Rock,
            },
            // We want to lose ...
            RoundResult::Lose => match self {
                RPS::Rock => RPS::Scissor,
                RPS::Paper => RPS::Rock,
                RPS::Scissor => RPS::Paper,
            },
        }
    }
}

impl RPSRound {
    fn read(s: &str) -> RPSRound {
        let letters = s.split(" ").collect::<Vec<&str>>();

        let o = RPS::from(letters[0]);
        let result = RoundResult::from(letters[1]);

        RPSRound {
            opponent: o,
            you: (o.response_for(result)),
        }
    }

    fn score(&self) -> i32 {
        let mut score = match self.you {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3,
        };

        if self.you.beats(&self.opponent) {
            score += 6
        } else if self.you == self.opponent {
            score += 3
        }

        println!("Round {:?} => {score}", self);

        score
    }
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> Vec<RPSRound> {
    let mut rounds = Vec::new();
    for line in lines {
        if let Ok(l) = line {
            println!("Reading {l}");
            rounds.push(RPSRound::read(&l));
        }
    }
    println!("Loaded {} rounds.", rounds.len());
    rounds
}

fn main() {
    match read_lines("./input") {
        Ok(lines) => {
            let score: i32 = parse_lines(lines).iter().map(|r| r.score()).sum();
            println!("Your score with this strategy would be {score}.");
        }
        _ => {
            panic!("Cannot parse input")
        }
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
