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

const CRT_WIDTH: usize = 40;
const CRT_LINES: usize = 6;

struct CPU<'p> {
    reg_x: i32,
    cycle: usize,
    pgm: &'p mut dyn Iterator<Item = Instruction>,
    signal_strength: i32,
    crt: [char; CRT_WIDTH * CRT_LINES],
}
impl<'a> CPU<'a> {
    fn new(pgm: &'a mut dyn Iterator<Item = Instruction>) -> Self {
        CPU {
            pgm,
            reg_x: 1,
            cycle: 0,
            signal_strength: 0,
            crt: [' '; 240],
        }
    }

    fn run(&mut self) -> i32 {
        while let Some(i) = self.pgm.next() {
            println!(":: X={} Cycle={} I={i:?}", self.reg_x, self.cycle);
            match i {
                Instruction::Noop => self.i_noop(),
                Instruction::AddX(x) => self.i_add(x),
            }
            self.display();
            let mut _line = String::new();
            // std::io::stdin()
            //     .read_line(&mut _line)
            //     .expect("Failed to read line");
        }
        self.signal_strength
    }

    fn display(&self) {
        self.crt
            .chunks(CRT_WIDTH)
            .for_each(|line| println!("{}", line.iter().collect::<String>()));
    }

    fn i_noop(&mut self) {
        self.cycle();
    }

    fn i_add(&mut self, x: i32) {
        self.cycle();
        self.cycle();
        self.reg_x += x;
    }

    fn cycle(&mut self) {
        // CRT Drawing
        let visible_points = self.reg_x - 1..=self.reg_x + 1;
        let cur_x = self.cycle % CRT_WIDTH;
        let cur_y = (self.cycle / CRT_WIDTH) % CRT_LINES;
        if visible_points.contains(&(cur_x as i32)) {
            self.crt[cur_x + CRT_WIDTH * cur_y] = '#';
        } else {
            self.crt[cur_x + CRT_WIDTH * cur_y] = '.';
        }

        self.cycle += 1;
        if self.cycle == 20 || (self.cycle > 20 && (self.cycle - 20) % 40 == 0) {
            self.signal_strength += self.cycle as i32 * self.reg_x;
        }
    }
}

fn main() -> Result<(), &'static str> {
    let filepath = env::args().nth(1).unwrap_or(String::from("input"));
    let data = fs::read_to_string(filepath).map_err(|_| "Unable to read file")?;

    let mut pgm = data.lines().filter_map(|l| l.try_into().ok());
    let mut cpu = CPU::new(&mut pgm);
    println!("Executed program - signal_strength={}", cpu.run());
    cpu.display();

    Ok(())
}

#[test]
fn test_part_1() {
    let data = include_str!("../input");
    let mut pgm = data.lines().filter_map(|l| l.try_into().ok());
    let mut cpu = CPU::new(&mut pgm);
    assert_eq!(cpu.run(), 14920);
}
