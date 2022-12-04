use std::collections::BTreeSet;
use std::env;
use std::fs;

fn find_shared_item(l: &str) -> Result<char, ()> {
    let bag1 = &l[..l.len() / 2];
    let bag2 = &l[l.len() / 2..];
    assert!(bag1.len() == bag2.len());

    let mut seen_in_bag1 = BTreeSet::new();
    bag1.chars().for_each(|c| {
        seen_in_bag1.insert(c);
    });

    let common = bag2
        .chars()
        .filter(|c| seen_in_bag1.contains(c))
        .collect::<Vec<char>>();

    if common.len() > 0 {
        return Ok(common[0]);
    } else {
        return Err(());
    }
}

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
    let lines = content.split("\n");

    let mut total: u32 = 0;
    for l in lines {
        if let Ok(c) = find_shared_item(l) {
            println!("Line {l} => {c} => {}", score(&c));
            total = total + score(&c);
        }
    }
    println!("Total {total}");
}
