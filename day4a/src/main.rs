use std::collections::HashSet;
use std::env;
use std::fs;

fn range2set(s: &str) -> HashSet<i32> {
    let (start, end) = s.split_once('-').unwrap();

    let a = start.parse::<i32>().unwrap();
    let b = end.parse::<i32>().unwrap() + 1;

    (a..b).collect()
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = &args[1];
    let content = fs::read_to_string(filepath).expect("unable to read");
    let lines = content.split("\n");

    let x = lines
        .filter(|l| {
            if let Some((s1, s2)) = l.split_once(',') {
                let elf1 = range2set(s1);
                let elf2 = range2set(s2);

                let r = elf1.is_subset(&elf2) || elf1.is_superset(&elf2);

                println!("{s1} ?? {s2} => {r}");
                r
            } else {
                false
            }
        })
        .count();
    println!("{x} binomes where one fully contains the other.");
}
