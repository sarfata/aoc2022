use std::f32::consts::PI;
use std::ops;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Point(i32, i32);
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vector(pub i32, pub i32);
impl Vector {
    pub fn length(&self) -> f32 {
        (((self.0).pow(2) + (self.1).pow(2)) as f32).sqrt()
    }
    pub fn angle(&self) -> i32 {
        // We use the (0,0) in top left convention so we need to take the opposite of the y coordinates
        let angle = (self.1 as f32).atan2(self.0 as f32);

        // Convert to rounded degrees because that will easier for this project
        let degrees = (360.0 * angle / (2.0 * PI)).round() as i32;
        degrees
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
    pub fn vec_to(&self, other: &Point) -> Vector {
        Vector(other.0 - self.0, other.1 - self.1)
    }
}

#[derive(Debug)]
pub struct Move(pub Dir, pub i32);

impl TryFrom<&str> for Move {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((l, r)) = value.split_once(" ") {
            Ok(Move(
                match l {
                    "R" => Dir::R,
                    "L" => Dir::L,
                    "U" => Dir::U,
                    "D" => Dir::D,
                    _ => return Err("Invalid movement command"),
                },
                r.parse().map_err(|_| "Invalid movement len")?,
            ))
        } else {
            Err("Invalid line")
        }
    }
}

#[derive(Debug)]
pub enum Dir {
    U,
    L,
    D,
    R,
}

impl ops::Add<&Dir> for Point {
    type Output = Point;

    fn add(self, m: &Dir) -> Point {
        match m {
            Dir::U => self + &Vector(0, 1),
            Dir::D => self + &Vector(0, -1),
            Dir::L => self + &Vector(-1, 0),
            Dir::R => self + &Vector(1, 0),
        }
    }
}
impl ops::Add<&Dir> for &Point {
    type Output = Point;

    fn add(self, m: &Dir) -> Point {
        *self + m
    }
}
impl ops::Add<&Vector> for Point {
    type Output = Point;

    fn add(self, v: &Vector) -> Point {
        Point(self.0 + v.0, self.1 + v.1)
    }
}

#[test]
fn test_vec() {
    let p0 = Point::new(2, 2);

    // To same
    assert_eq!(p0.vec_to(&p0).length(), 0.0);
    assert_eq!(p0.vec_to(&p0).angle(), 0);

    // To a point on the right
    let p1 = Point::new(3, 2);
    assert_eq!(p0.vec_to(&p1).length(), 1.0);
    assert_eq!(p0.vec_to(&p1).angle(), 0);

    // To a point on the top
    let p1 = Point::new(2, 1);
    assert_eq!(p0.vec_to(&p1).length(), 1.0);
    assert_eq!(p0.vec_to(&p1).angle(), 90);

    // To a point on the left
    let p1 = Point::new(1, 2);
    assert_eq!(p0.vec_to(&p1).length(), 1.0);
    assert_eq!(p0.vec_to(&p1).angle(), 180);

    // To a point on the top-right
    let p1 = Point::new(3, 1);
    assert_eq!((p0.vec_to(&p1).length() * 10.0).round() / 10.0, 1.4);
    assert_eq!(p0.vec_to(&p1).angle(), 45);

    // To a point lower on the right
    let p1 = Point::new(3, 3);
    assert_eq!((p0.vec_to(&p1).length() * 10.0).round() / 10.0, 1.4);
    assert_eq!(p0.vec_to(&p1).angle(), -45);
}
