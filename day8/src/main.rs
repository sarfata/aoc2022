use std::cell::Cell;
use std::env;
use std::fs;

struct Garden(Vec<Vec<u32>>);
impl std::fmt::Debug for Garden {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let table = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(fmt, "{}", table)
    }
}
impl TryFrom<&str> for Garden {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Garden(
            value
                .lines()
                .map(|l| {
                    l.chars()
                        .filter_map(|c| c.to_digit(10))
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>(),
        ))
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ViewAngle {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}
#[derive(PartialEq, Eq, Debug)]
enum ScenicView {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Garden {
    fn visit(&self, visitor: impl Fn(usize, usize, u32) -> ()) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, h) in row.iter().enumerate() {
                visitor(x, y, *h);
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u32> {
        Some(*(self.0.get(y)?.get(x)?))
    }

    fn row(&self, y: usize) -> Option<Vec<u32>> {
        self.0.get(y).cloned()
    }

    fn col(&self, x: usize) -> Option<Vec<u32>> {
        let col = self
            .0
            .iter()
            .filter_map(|row| row.get(x))
            .cloned()
            .collect();

        Some(col)
    }

    fn is_tree_visible_from(&self, x: usize, y: usize, view: ViewAngle) -> Option<bool> {
        let mut index;
        let mut trees = match view {
            ViewAngle::LEFT | ViewAngle::RIGHT => {
                index = x;
                self.row(y)?
            }
            ViewAngle::TOP | ViewAngle::BOTTOM => {
                index = y;
                self.col(x)?
            }
        };
        if view == ViewAngle::RIGHT || view == ViewAngle::BOTTOM {
            trees.reverse();
            index = trees.len() - 1 - index;
        }

        // println!("Evaluation vis of {x} {y} from {view:?} => {index} in {trees:?}.");

        // If the tree is the first one in line then it's always visible
        if index == 0 {
            return Some(true);
        }

        // Otherwise we need to see if there is a bigger tree in front
        let max_height = trees.iter().take(index).max()?;
        // println!("max height: {max_height} h[index]={}", trees[index]);
        Some(trees[index] > *max_height)
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> Option<bool> {
        if self.is_tree_visible_from(x, y, ViewAngle::LEFT)?
            || self.is_tree_visible_from(x, y, ViewAngle::RIGHT)?
            || self.is_tree_visible_from(x, y, ViewAngle::TOP)?
            || self.is_tree_visible_from(x, y, ViewAngle::BOTTOM)?
        {
            Some(true)
        } else {
            Some(false)
        }
    }

    // TODO Reimplement with the visitor since we made one!
    fn visible_trees(&self) -> Option<u32> {
        let width = self.0.len();
        let height = self.0.get(0)?.len();

        let mut visible = 0;
        for y in 0..height {
            for x in 0..width {
                if self.is_tree_visible(x, y)? {
                    visible = visible + 1;
                }
            }
        }
        Some(visible)
    }

    fn scenic_score_towards(&self, x: usize, y: usize, v: ScenicView) -> Option<usize> {
        let trees: Vec<u32> = match v {
            ScenicView::DOWN => self.col(x)?.iter().skip(y + 1).cloned().collect(),
            ScenicView::UP => self.col(x)?.iter().take(y).rev().cloned().collect(),
            ScenicView::RIGHT => self.row(y)?.iter().skip(x + 1).cloned().collect(),
            ScenicView::LEFT => self.row(y)?.iter().take(x).rev().cloned().collect(),
        };

        // println!("scenic_score({x}, {y}, {v:?} -> trees={trees:?}");
        if trees.len() == 0 {
            return Some(0);
        }

        let house_height = self.get(x, y)?;
        let visible_trees = 1 + trees
            .iter()
            .position(|e| *e >= house_height)
            .or(Some(trees.len() - 1))
            .unwrap();
        Some(visible_trees)
    }

    fn scenic_score(&self, x: usize, y: usize) -> Option<usize> {
        Some(
            self.scenic_score_towards(x, y, ScenicView::UP)?
                * self.scenic_score_towards(x, y, ScenicView::DOWN)?
                * self.scenic_score_towards(x, y, ScenicView::LEFT)?
                * self.scenic_score_towards(x, y, ScenicView::RIGHT)?,
        )
    }

    fn best_scenic_score(&self) -> Option<usize> {
        let best_views = Cell::from(0);
        self.visit(|x, y, _h| {
            let score = self.scenic_score(x, y).unwrap();
            if score > best_views.get() {
                best_views.set(score);
                println!("Found new best view: {x} {y} => {score}");
            }
        });
        Some(best_views.get())
    }
}

#[test]
fn parse_ls() {
    match "".try_into() as Result<Garden, _> {
        Ok(x) => assert!(x.0.len() == 0),
        Err(e) => assert!(false, "{e:?}"),
    }
}

#[test]
fn test_outer_edge() -> Result<(), &'static str> {
    let g: Garden = include_str!("../test").try_into()?;

    // Outer edge is visible
    assert!(g.is_tree_visible_from(0, 0, ViewAngle::TOP).unwrap());
    assert!(g.is_tree_visible_from(0, 0, ViewAngle::LEFT).unwrap());
    assert!(g.is_tree_visible_from(4, 0, ViewAngle::TOP).unwrap());
    assert!(g.is_tree_visible_from(4, 0, ViewAngle::RIGHT).unwrap());
    assert!(g.is_tree_visible_from(4, 4, ViewAngle::RIGHT).unwrap());
    assert!(g.is_tree_visible_from(4, 4, ViewAngle::BOTTOM).unwrap());
    assert!(g.is_tree_visible_from(4, 4, ViewAngle::BOTTOM).unwrap());
    assert!(g.is_tree_visible_from(0, 4, ViewAngle::LEFT).unwrap());

    Ok(())
}
#[test]
fn test_left_middle() -> Result<(), &'static str> {
    let g: Garden = include_str!("../test").try_into()?;

    // Left middle is visible only from the right
    assert!(g.is_tree_visible_from(1, 2, ViewAngle::RIGHT).unwrap());
    assert!(!g.is_tree_visible_from(1, 2, ViewAngle::LEFT).unwrap());
    assert!(!g.is_tree_visible_from(1, 2, ViewAngle::TOP).unwrap());
    assert!(!g.is_tree_visible_from(1, 2, ViewAngle::BOTTOM).unwrap());

    Ok(())
}

#[test]
fn test_top_middle_5() -> Result<(), &'static str> {
    let g: Garden = include_str!("../test").try_into()?;

    assert!(g.is_tree_visible_from(2, 1, ViewAngle::TOP).unwrap());
    assert!(g.is_tree_visible_from(2, 1, ViewAngle::RIGHT).unwrap());
    assert!(!g.is_tree_visible_from(2, 1, ViewAngle::LEFT).unwrap());
    assert!(!g.is_tree_visible_from(2, 1, ViewAngle::BOTTOM).unwrap());

    Ok(())
}

#[test]
fn test_center_3() -> Result<(), &'static str> {
    let g: Garden = include_str!("../test").try_into()?;

    assert!(!g.is_tree_visible_from(2, 2, ViewAngle::TOP).unwrap());
    assert!(!g.is_tree_visible_from(2, 2, ViewAngle::RIGHT).unwrap());
    assert!(!g.is_tree_visible_from(2, 2, ViewAngle::LEFT).unwrap());
    assert!(!g.is_tree_visible_from(2, 2, ViewAngle::BOTTOM).unwrap());

    Ok(())
}
#[test]
fn sc_5() {
    let g: Garden = include_str!("../test").try_into().unwrap();

    assert_eq!(g.scenic_score_towards(2, 1, ScenicView::UP).unwrap(), 1);
    assert_eq!(g.scenic_score_towards(2, 1, ScenicView::LEFT).unwrap(), 1);
    assert_eq!(g.scenic_score_towards(2, 1, ScenicView::RIGHT).unwrap(), 2);
    assert_eq!(g.scenic_score_towards(2, 1, ScenicView::DOWN).unwrap(), 2);

    assert_eq!(g.scenic_score(2, 1).unwrap(), 4);
}
#[test]
fn sc_5b() {
    let g: Garden = include_str!("../test").try_into().unwrap();
    assert_eq!(g.scenic_score(2, 3).unwrap(), 8);
}

#[test]
fn sc_3_2() {
    let g: Garden = include_str!("../test").try_into().unwrap();
    // The text says the 5 at (2,3) is the best house but this is better
    assert_eq!(g.scenic_score(3, 2).unwrap(), 2);
}

fn main() -> Result<(), &'static str> {
    let filepath = env::args().nth(1).unwrap_or(String::from("input"));
    let data = fs::read_to_string(filepath).map_err(|_| "Unable to read file")?;

    let garden: Garden = data.as_str().try_into()?;
    println!("{garden:?}");
    println!(
        "Visible trees: {}",
        garden.visible_trees().ok_or("visible tree")?
    );

    println!(
        "Max view tree: {}",
        garden.best_scenic_score().ok_or("maxview")?
    );
    Ok(())
}
