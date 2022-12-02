use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(PartialEq, Debug)]
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

impl RPS {
    fn from(s: &str) -> RPS {
        match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissor,
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
}

impl RPSRound {
    fn read(s: &str) -> RPSRound {
        let letters = s.split(" ").collect::<Vec<&str>>();
        RPSRound {
            opponent: RPS::from(letters[0]),
            you: RPS::from(letters[1]),
        }
    }
}

fn process_input(lines: io::Lines<BufReader<File>>) -> Vec<RPSRound> {
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

fn score_round(r: &RPSRound) -> i32 {
    let mut score = match r.you {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    };

    if r.you.beats(&r.opponent) {
        score += 6
    } else if r.you == r.opponent {
        score += 3
    }

    println!("Round {:?} => {score}", r);

    score
}

fn main() {
    let score = match read_lines("./input") {
        Ok(lines) => process_input(lines).iter().map(score_round).sum(),
        _ => 0,
    };
    println!("Your score with this strategy would be {score}.");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
