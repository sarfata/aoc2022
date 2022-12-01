use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct ElfDiet {
    total_calories: i64,
    badge: i32,
}

fn main() {
    // File hosts must exist in current path before this produces output
    let mut diets = Vec::new();

    let mut current_total = 0;
    let mut badge = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(calories) = line {
                if calories == "" {
                    // println!("Elf {badge} is all set with {current_total}");
                    diets.push(ElfDiet {
                        total_calories: current_total,
                        badge: badge,
                    });
                    current_total = 0;
                    badge += 1;
                } else {
                    current_total += calories.parse::<i64>().unwrap();
                    // println!("Adding {calories} to current total ({current_total})");
                }
            }
        }
    }

    diets.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    println!("Top 3 Elves:");
    for i in 0..3 {
        println!("  Elf {} with {}", diets[i].badge, diets[i].total_calories);
    }
    println!(
        "Top 3 elves have a total of {} calories.",
        diets[0..3]
            .iter()
            .fold(0, |sum, elf| sum + elf.total_calories)
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
