use std::env;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug)]
struct Dock {
    stacks: Vec<Vec<char>>,
}

impl TryFrom<&str> for Dock {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines().collect::<Vec<&str>>();
        lines.reverse();

        let count = lines[0].trim().split_whitespace().count();

        let mut stacks: Vec<Vec<char>> = Vec::new();
        for _ in 0..count {
            stacks.push(Vec::new());
        }
        for i in 1..lines.len() {
            for s in 0..count {
                let c = lines[i].chars().nth(s * 4 + 1).unwrap_or(' ');
                match c {
                    'A'..='Z' => stacks[s].push(c),
                    _ => (),
                }
            }
        }
        Ok(Dock { stacks })
    }
}

#[derive(Debug)]
struct MoveOrder {
    qty: usize,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for MoveOrder {
    type Error = ParseIntError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        Ok(MoveOrder {
            qty: parts[1].parse()?,
            from: parts[3].parse()?,
            to: parts[5].parse()?,
        })
    }
}

impl Dock {
    // For first part of puzzle
    #[allow(dead_code)]
    fn run9000(&mut self, m: &MoveOrder) -> Option<()> {
        for _c in 0..m.qty {
            let popped = self.stacks.get_mut(m.from - 1)?.pop()?;
            self.stacks.get_mut(m.to - 1)?.push(popped);

            println!("Move: {m:?} => {self:?}");
        }
        Some(())
    }

    // For second part of puzzle
    fn run9001(&mut self, m: &MoveOrder) -> Option<()> {
        let from_stack = self.stacks.get(m.from - 1)?.clone(); // clone to avoid borrowing stacks twice - how do i avoid this?
        let to_stack = self.stacks.get_mut(m.to - 1)?;
        for i in 0..m.qty {
            to_stack.push(from_stack.get(from_stack.len() - m.qty + i)?.clone());
        }

        let from_stack = self.stacks.get_mut(m.from - 1)?;
        for _i in 0..m.qty {
            from_stack.pop();
        }

        println!("Move: {m:?} => {self:?}");
        Some(())
    }

    fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&' ').to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = &args[1];
    let content = fs::read_to_string(filepath).expect("unable to read");
    let (start_config, move_orders) = content.split_once("\n\n").unwrap();

    let mut dock: Dock = start_config.try_into().unwrap();
    let moves: Vec<MoveOrder> = move_orders.lines().map(|l| l.try_into().unwrap()).collect();

    for m in moves {
        dock.run9001(&m);
    }

    println!("Top of docks: {}", dock.tops());
}
