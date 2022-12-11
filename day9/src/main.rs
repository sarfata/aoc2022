mod point;
use point::*;

struct Rope {
    knots: Vec<Point>,
    tail_history: VisitHistory,
}

impl Rope {
    fn new(count_knots: usize, x: i32, y: i32) -> Self {
        // This is my justification to do some unwrap on item 0 and last item later...
        assert!(count_knots >= 2);

        let mut knots = vec![];
        for _ in 0..count_knots {
            knots.push(Point::new(x, y));
        }
        Rope {
            knots,
            tail_history: VisitHistory::new(),
        }
    }
    fn record_tail_position(&mut self) {
        self.tail_history.visit(*self.knots.last().unwrap());
    }

    fn move_head(&mut self, m: Move) {
        println!("Move {m:?}");

        // Make sure we record this initial state
        self.record_tail_position();

        // Execute the movement
        for _ in 0..m.1 {
            self.knots[0] = &self.knots[0] + &m.0;
            // self.head = &self.head + &m.0;

            self.adjust_knots();

            // And make sure we record this visit
            self.record_tail_position();
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

    fn adjust_knots(&mut self) {
        // Walk through all the knots - Starting from the one behind the head
        for i in 1..self.knots.len() {
            let v: Vector = self.knots[i].vec_to(&self.knots[i - 1]);

            self.knots[i] = self.knots[i] + &Self::next_tail_move(&v);
        }
    }

    fn print_world(&self) {
        let w = 22;
        let h = 22;
        for y in (0..h).rev() {
            for x in 0..w {
                let p = Point::new(x, y);
                if let Some(p) = self.knots.iter().position(|&x| x == p) {
                    if p == 0 {
                        print!("H");
                    } else if p == self.knots.len() - 1 {
                        print!("T");
                    } else {
                        print!("{p}");
                    }
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
    fn print_visited(&self) {
        let w = 100;
        let h = 100;
        for y in (0..h).rev() {
            for x in 0..w {
                let p = Point::new(x, y);
                if self.tail_history.0.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
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

    let mut rope = Rope::new(10, 11, 5);

    println!("Start:");
    rope.print_world();

    for line in data.lines() {
        let m: Move = line.try_into()?;
        println!("Executing {m:?}");
        rope.move_head(m);
        rope.print_world();
    }

    println!("Visited {} boxes.", rope.tail_visits());
    rope.print_visited();

    Ok(())
}
