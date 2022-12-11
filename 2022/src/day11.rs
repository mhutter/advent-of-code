use std::cmp::Reverse;

use monkey::*;

mod monkey {
    /// An item that is being hold by a monkey
    pub type Item = u64;

    /// A Monkey holding items
    pub struct Monkey {
        pub items: Vec<Item>,
        pub inspections: u64,
        pub operation: Box<dyn Fn(Item) -> Item>,
        pub test: Box<dyn Fn(Item) -> bool>,
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
        pub fn new<Op, T>(
            items: Vec<Item>,
            operation: Op,
            test: T,
            on_true: usize,
            on_false: usize,
        ) -> Self
        where
            Op: Fn(Item) -> Item + 'static,
            T: Fn(Item) -> bool + 'static,
        {
            Self {
                items,
                inspections: 0,
                operation: Box::new(operation),
                test: Box::new(test),
                on_true,
                on_false,
            }
        }
    }
}

pub fn day11p1(input: &str) -> u64 {
    let mut monkeys = input
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
                ["new", "=", "old", "*", "old"] => Box::new(move |i: Item| i * i),
                ["new", "=", "old", "*", n] => {
                    let n = n.parse::<u64>().expect("n to be a number");
                    Box::new(move |i: Item| i * n)
                }
                ["new", "=", "old", "+", n] => {
                    let n = n.parse::<u64>().expect("n to be a number");
                    Box::new(move |i: Item| i + n)
                }
                op => panic!("Unknown operation: {op:?}"),
            };

            let test = match lines
                .next()
                .expect("Test")
                .split(": ")
                .nth(1)
                .expect("test condition")
                .split_ascii_whitespace()
                .collect::<Vec<_>>()
                .as_slice()
            {
                ["divisible", "by", n] => {
                    let n = n.parse::<u64>().expect("n to be a number");
                    move |i| (i % n) == 0
                }
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

            Monkey::new(items, operation, test, on_true, on_false)
        })
        .collect::<Vec<_>>();
    let num_monkeys = monkeys.len();

    for _ in 0..20 {
        for mi in 0..num_monkeys {
            while let Some(item) = monkeys[mi].items.pop() {
                let m = &mut monkeys[mi];
                m.inspections += 1;
                let item = (m.operation)(item) / 3;
                let throw_to = match (m.test)(item) {
                    true => m.on_true,
                    false => m.on_false,
                };
                monkeys[throw_to].items.push(item);
            }
        }
    }

    let mut business = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    business.sort_unstable_by_key(|i| Reverse(*i));

    business[0] * business[1]
}

pub fn day11p2(_input: &str) -> u64 {
    0
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
        assert_eq!(0, day11p2(INPUT));
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
