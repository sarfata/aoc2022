mod point;
use point::*;

struct Rope {
    head: Point,
    tail: Point,
    tail_history: VisitHistory,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
            tail_history: VisitHistory::new(),
        }
    }
    fn move_head(&mut self, m: Move) {
        println!("Move {m:?}");

        // Make sure we record this initial state
        self.tail_history.visit(self.tail);

        // We have to print upside down to make sure that +1 on y is UP.
        for _ in 0..m.1 {
            self.head = &self.head + &m.0;

            self.adjust_tail();

            // And make sure we record this visit
            self.tail_history.visit(self.tail);
        }
    }

    fn next_tail_move(v: &Vector) -> Vector {
        // Take the vector between the tail and the head (pointing towards the head) and return the vector of the transformation to apply.
        match v {
            // Nul Vector - Do not move.
            Vector(0, 0) => Vector(0, 0),
            // Any of the 8 direct positions around: do not move
            Vector(1, 0)
            | Vector(1, 1)
            | Vector(0, 1)
            | Vector(-1, 1)
            | Vector(-1, 0)
            | Vector(-1, -1)
            | Vector(0, -1)
            | Vector(1, -1) => Vector(0, 0),
            // Two gaps, directly up/down
            Vector(2, 0) => Vector(1, 0),
            Vector(-2, 0) => Vector(-1, 0),
            Vector(0, 2) => Vector(0, 1),
            Vector(0, -2) => Vector(0, -1),
            // Two gaps but not directly up/down - use the diagonal
            Vector(2, 1) | Vector(2, 2) | Vector(1, 2) => Vector(1, 1),
            Vector(-2, 1) | Vector(-2, 2) | Vector(-1, 2) => Vector(-1, 1),
            Vector(-2, -1) | Vector(-2, -2) | Vector(-1, -2) => Vector(-1, -1),
            Vector(2, -1) | Vector(2, -2) | Vector(1, -2) => Vector(1, -1),

            _ => panic!("Unexpected vector angle {}", v.angle()),
        }
    }

    fn adjust_tail(&mut self) {
        let mut v = self.tail.vec_to(&self.head);

        loop {
            // Get the next move and stop when not moving anymore
            let m = Self::next_tail_move(&v);
            if m == Vector(0, 0) {
                break;
            }
            self.tail = self.tail + &m;

            // Update the rope vector
            v = self.tail.vec_to(&self.head);
        }
    }

    fn print_world(&self) {
        let w = 10;
        let h = 10;
        for y in (0..h).rev() {
            for x in 0..w {
                let p = Point::new(x, y);
                if self.head == p {
                    print!("H");
                } else if self.tail == p {
                    print!("T");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }

    fn tail_visits(&self) -> usize {
        self.tail_history.0.len()
    }
}

use std::collections::HashSet;
struct VisitHistory(HashSet<Point>);
impl VisitHistory {
    fn new() -> Self {
        VisitHistory(HashSet::new())
    }

    fn visit(&mut self, p: Point) {
        self.0.insert(p);
    }
}

use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let filepath = env::args().nth(1).unwrap_or(String::from("input"));
    let data = fs::read_to_string(filepath).map_err(|_| "Unable to read file")?;

    let mut rope = Rope::new();

    println!("Start:");
    rope.print_world();

    for line in data.lines() {
        let m: Move = line.try_into()?;
        println!("Executing {m:?}");
        rope.move_head(m);
        rope.print_world();
    }

    println!("Visited {} boxes.", rope.tail_visits());

    Ok(())
}
