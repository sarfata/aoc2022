// pub mod stringset;
mod stringintersection;

use std::env;
use std::fs;

use stringintersection::string_intersection;

fn score(c: &char) -> u32 {
    if c.is_lowercase() {
        return *c as u32 - ('a' as u32) + 1;
    } else {
        return *c as u32 - ('A' as u32) + 27;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(&file_path).expect("Unable to read {file_path}");
    let mut lines = content.split("\n");

    let mut group = Vec::new();

    let mut total = 0;
    while let Some(l) = &lines.next() {
        group.push(l.clone());
        if group.len() == 3 {
            let intersection = string_intersection(&group);
            assert!(intersection.len() == 1);
            total += score(intersection.iter().next().unwrap());

            group.clear();
        }
    }
    println!("Score is {total}")

    // let mut total: u32 = 0;
    // for l in lines {
    //     if let Ok(c) = find_shared_item(l) {
    //         println!("Line {l} => {c} => {}", score(&c));
    //         total = total + score(&c);
    //     }
    // }
    // println!("Total {total}");
}
