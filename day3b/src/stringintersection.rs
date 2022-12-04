use std::collections::HashSet;

fn string_to_set(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    s.chars().for_each(|c| {
        set.insert(c);
    });
    set
}

pub fn string_intersection(strings: &Vec<&str>) -> HashSet<char> {
    if strings.len() == 0 {
        return HashSet::new();
    }
    let mut it = strings.iter();
    let mut result = string_to_set(it.next().unwrap());

    while let Some(e) = it.next() {
        result = result.intersection(&string_to_set(*e)).cloned().collect()
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_of_one() {
        let i = string_intersection(&vec!["abcdef"]);
        assert!(i.contains(&'a'));
        assert!(!i.contains(&'z'));
    }
    #[test]
    fn intersection_of_three() {
        let i = string_intersection(&vec!["abc", "cde", "efg"]);
        assert_eq!(i.len(), 0);
    }
    #[test]
    fn intersection_of_three_overlapping() {
        let i = string_intersection(&vec!["123", "145", "831"]);
        assert_eq!(i.len(), 1);
        assert!(i.contains(&'1'));
    }
}
