use num_bigint::{BigUint, ToBigUint};

mod worry;

type WorryLevel = u32;

struct Monkey {
    items: Vec<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    test_divide_by: u32,
    yes_throw_to: usize,
    no_throw_to: usize,
    inspections: usize,
}

impl Monkey {
    fn new(
        initial_items: &[WorryLevel],
        operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
        test_divide_by: u32,
        yes_throw_to: usize,
        no_throw_to: usize,
    ) -> Self {
        Self {
            items: Vec::from(initial_items),
            operation,
            test_divide_by,
            yes_throw_to,
            no_throw_to,
            inspections: 0,
        }
    }

    /// Inspect all items in the monkey hand and return a list of items to add to other monkeys.
    fn inspect_all_items(&self) -> Vec<(usize, WorryLevel)> {
        let mut transfers = vec![];
        // for i in 0..self.items.len() {
        self.items.iter().for_each(|it| {
            // println!("  Monkey inspects an item with level of {}", *it);
            let mut new_value = self.operation.as_ref()(*it);
            // println!("    multiplied to {}", new_value);
            new_value /= 3;
            // println!("    divided to {}", new_value);

            let next_monkey = if &new_value % self.test_divide_by == 0 {
                self.yes_throw_to
            } else {
                self.no_throw_to
            };
            // println!(
            //     "    {} - level {} thrown to {}",
            //     new_value % self.test_divide_by == 0,
            //     new_value,
            //     next_monkey
            // );
            transfers.push((next_monkey, new_value));
        });
        transfers
    }
}

fn run_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        // println!("Monkey {i}");

        let m = &monkeys[i];
        let transfers = m.inspect_all_items();
        monkeys[i].inspections += transfers.len();
        for (to, val) in transfers {
            monkeys[to].items.push(val);
        }
        monkeys[i].items.clear();
    }
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    for (i, m) in monkeys.iter().enumerate() {
        println!(
            "Monkey {} ({} inspections): {}",
            i,
            m.inspections,
            m.items
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

fn main() {
    println!("Hello, world!");

    // let mut monkeys = vec![
    //     Monkey::new(
    //         &[79, 98],
    //         Box::from(|x: &BigUint| x * 19.to_biguint().unwrap()),
    //         23,
    //         2,
    //         3,
    //     ),
    //     Monkey::new(
    //         &[54, 65, 75, 74],
    //         Box::from(|x: &BigUint| x + 6.to_biguint().unwrap()),
    //         19,
    //         2,
    //         0,
    //     ),
    //     Monkey::new(&[79, 60, 97], Box::from(|x: &BigUint| x * x), 13, 1, 3),
    //     Monkey::new(
    //         &[74],
    //         Box::from(|x: &BigUint| x + 3.to_biguint().unwrap()),
    //         17,
    //         0,
    //         1,
    //     ),
    // ];

    let mut monkeys = vec![
        Monkey::new(&[64], Box::from(|x| x * 7), 13, 1, 3),
        Monkey::new(&[60, 84, 84, 65], Box::from(|x| x + 7), 19, 2, 7),
        Monkey::new(&[52, 67, 74, 88, 51, 61], Box::from(|x| x * 3), 5, 5, 7),
        Monkey::new(&[67, 72], Box::from(|x| x + 3), 2, 1, 2),
        Monkey::new(
            &[80, 79, 58, 77, 68, 74, 98, 64],
            Box::from(|x| x * x),
            17,
            6,
            0,
        ),
        Monkey::new(&[62, 53, 61, 89, 86], Box::from(|x| x + 8), 11, 4, 6),
        Monkey::new(&[86, 89, 82], Box::from(|x| x + 2), 7, 3, 0),
        Monkey::new(&[92, 81, 70, 96, 69, 84, 83], Box::from(|x| x + 4), 3, 4, 5),
    ];

    for i in 1..=20 {
        run_round(&mut monkeys);
        if i == 1 || i == 20 || i % 1000 == 0 {
            println!("After round {i}, the monkeys are:");
            print_monkeys(&monkeys)
        } else if i % 100 == 0 {
            println!(".");
        }
    }
    // Answer 1 was 55216
}
