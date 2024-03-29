use std::{cmp::Reverse, ops::Div};

use monkey::*;

mod monkey {
    use std::ops::Rem;

    /// An item that is being hold by a monkey
    pub type Item = u64;

    /// A Monkey holding items
    pub struct Monkey {
        pub items: Vec<Item>,
        pub inspections: u64,
        pub operation: Box<dyn Fn(Item) -> Item>,
        pub divisor: Item,
        // pub test: Box<dyn Fn(Item) -> bool>,
        pub on_true: usize,
        pub on_false: usize,
    }
    impl std::fmt::Debug for Monkey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Monkey")
                .field("items", &self.items)
                .field("inspections", &self.inspections)
                .field("on_true", &self.on_true)
                .field("on_false", &self.on_false)
                .finish()
        }
    }

    impl Monkey {
        pub fn new<Op>(
            items: Vec<Item>,
            operation: Op,
            // test: T,
            divisor: Item,
            on_true: usize,
            on_false: usize,
        ) -> Self
        where
            Op: Fn(Item) -> Item + 'static,
        {
            Self {
                items,
                inspections: 0,
                operation: Box::new(operation),
                divisor,
                // test: Box::new(test),
                on_true,
                on_false,
            }
        }

        /// Decide where to throw the given item to
        pub fn throw_to(&self, item: &Item) -> usize {
            match item.rem(&self.divisor).lt(&1) {
                true => self.on_true,
                false => self.on_false,
            }
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines().skip(1);
            let items = lines
                .next()
                .expect("starting items")
                .split(": ")
                .nth(1)
                .expect("items")
                .split(", ")
                .map(|i| i.parse().expect("number"))
                .collect();

            let operation: Box<dyn Fn(Item) -> Item> = match lines
                .next()
                .expect("Operation")
                .split(": ")
                .nth(1)
                .expect("operation description")
                .split_ascii_whitespace()
                .collect::<Vec<_>>()
                .as_slice()
            {
                ["new", "=", "old", "*", "old"] => Box::new(move |i: Item| i.pow(2)),
                ["new", "=", "old", "*", n] => {
                    let n = n.parse::<Item>().expect("n to be a number");
                    Box::new(move |i: Item| i * n)
                }
                ["new", "=", "old", "+", n] => {
                    let n = n.parse::<Item>().expect("n to be a number");
                    Box::new(move |i: Item| i + n)
                }
                op => panic!("Unknown operation: {op:?}"),
            };

            let divisor = match lines
                .next()
                .expect("Test")
                .split(": ")
                .nth(1)
                .expect("test condition")
                .split_ascii_whitespace()
                .collect::<Vec<_>>()
                .as_slice()
            {
                ["divisible", "by", n] => n.parse::<Item>().expect("n to be a number"),
                t => panic!("Unknown test: {t:?}"),
            };

            let on_true = lines
                .next()
                .expect("If true")
                .split_ascii_whitespace()
                .last()
                .expect("target monkey")
                .parse()
                .unwrap();
            let on_false = lines
                .next()
                .expect("If false")
                .split_ascii_whitespace()
                .last()
                .expect("target monkey")
                .parse()
                .unwrap();

            Monkey::new(items, operation, divisor, on_true, on_false)
        })
        .collect::<Vec<_>>()
}

pub fn day11p1(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(input);
    let num_monkeys = monkeys.len();

    for _ in 0..20 {
        for i in 0..num_monkeys {
            while let Some(item) = monkeys[i].items.pop() {
                let m = &mut monkeys[i];

                m.inspections += 1;
                let item = (m.operation)(item).div(&3);
                let throw_to = m.throw_to(&item);
                monkeys[throw_to].items.push(item);
            }
        }
    }

    let mut business = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    business.sort_unstable_by_key(|i| Reverse(*i));

    business[0] * business[1]
}

pub fn day11p2(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(input);
    let num_monkeys = monkeys.len();
    let divisor_product = monkeys.iter().fold(1, |acc, m| acc * m.divisor);

    for r in 0..10_000 {
        if r % 1000 == 0 {
            println!("After {r}: {monkeys:?}");
        }

        for i in 0..num_monkeys {
            while let Some(item) = monkeys[i].items.pop() {
                let m = &mut monkeys[i];

                m.inspections += 1;
                let item = (m.operation)(item) % divisor_product;
                let throw_to = m.throw_to(&item);
                monkeys[throw_to].items.push(item);
            }
        }
    }

    let mut business = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    business.sort_unstable_by_key(|i| Reverse(*i));

    business[0] * business[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(10605, day11p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(2713310158, day11p2(INPUT));
    }

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
