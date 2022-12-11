use std::env;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Noop,
    AddX(i32),
}
impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "noop" {
            Ok(Instruction::Noop)
        } else if let Some(("addx", r)) = value.split_once(" ") {
            Ok(Instruction::AddX(
                r.parse::<i32>().map_err(|_| "Invalid add-value")?,
            ))
        } else {
            Err("Invalid instruction")
        }
    }
}

struct CPU {}
impl CPU {
    fn execute(pgm: Vec<Instruction>) -> i32 {
        let mut reg_x = 1;
        let mut cycle = 0;
        let mut ix = 0;

        let mut signal_strength = 0;

        while ix < pgm.len() {
            match pgm[ix] {
                Instruction::Noop => {
                    cycle += 1;
                    ix += 1;

                    if cycle == 20 || (cycle - 20) % 40 == 0 {
                        signal_strength += cycle * reg_x;
                    }
                }
                Instruction::AddX(a) => {
                    cycle += 1;

                    if cycle == 20 || (cycle - 20) % 40 == 0 {
                        signal_strength += cycle * reg_x;
                    }

                    cycle += 1;

                    if cycle == 20 || (cycle - 20) % 40 == 0 {
                        signal_strength += cycle * reg_x;
                    }

                    reg_x = reg_x + a;
                    ix += 1;
                }
            }
        }
        signal_strength
    }
}

fn main() -> Result<(), &'static str> {
    let filepath = env::args().nth(1).unwrap_or(String::from("input"));
    let data = fs::read_to_string(filepath).map_err(|_| "Unable to read file")?;

    let pgm: Vec<Instruction> = data.lines().filter_map(|l| l.try_into().ok()).collect();
    println!(
        "Executed {} instructions - signal_strength={}",
        pgm.len(),
        CPU::execute(pgm)
    );

    Ok(())
}
