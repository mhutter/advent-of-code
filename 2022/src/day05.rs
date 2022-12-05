use supply::*;

mod supply {
    use std::{collections::HashMap, fmt::Debug};

    /// Represents a crate on a stack
    ///
    /// Can be converted from a `&str` in the format `[A]` where `A` is a single, upper case
    /// letter.
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct Crate(char);

    impl From<&str> for Crate {
        fn from(s: &str) -> Self {
            let c = s.chars().nth(1).expect("crate must have format `[A]`");
            Self(c)
        }
    }
    impl TryFrom<&[char]> for Crate {
        type Error = ();

        fn try_from(value: &[char]) -> Result<Self, Self::Error> {
            let c = value[1];
            if c.is_ascii_uppercase() {
                Ok(Self(c))
            } else {
                Err(())
            }
        }
    }
    impl Debug for Crate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", &self.0)
        }
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Stacks<'a>(HashMap<&'a str, Vec<Crate>>);

    impl<'a> From<&'a str> for Stacks<'a> {
        fn from(s: &'a str) -> Self {
            let mut drawing = s.lines().rev();
            let stack_names = drawing
                .next()
                .expect("find stack names")
                .split_ascii_whitespace()
                .collect::<Vec<_>>();
            let mut names = stack_names.iter().cycle();
            let mut stacks = Stacks::with_names(stack_names.clone());

            for row in drawing {
                let crates = row
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(4)
                    .map(Crate::try_from)
                    .collect::<Vec<_>>();

                for crt in crates {
                    let stack = names.next().unwrap();

                    if let Ok(crt) = crt {
                        stacks.place_on(stack, crt);
                    }
                }
            }

            stacks
        }
    }

    impl<'a> Stacks<'a> {
        /// Create new stacks with the given names
        pub fn with_names(names: Vec<&'a str>) -> Self {
            Self(names.into_iter().map(&|name| (name, Vec::new())).collect())
        }

        /// Place the given `Crate` on the `stack`
        pub fn place_on(&mut self, stack: &str, crt: Crate) {
            self.0
                .get_mut(stack)
                .unwrap_or_else(|| panic!("invalid stack: {stack}"))
                .push(crt);
        }

        /// Remove one crate from the given `stack` and return it
        pub fn take_from(&mut self, stack: &str) -> Option<Crate> {
            self.0
                .get_mut(stack)
                .unwrap_or_else(|| panic!("invalid stack: {stack}"))
                .pop()
        }

        /// Execute a single instruction by moving one crate after another
        pub fn execute_single_crate(&mut self, instruction: Instruction) -> Result<(), ()> {
            for _ in 0..instruction.amount {
                let crt = self.take_from(instruction.from).ok_or(())?;
                self.place_on(instruction.to, crt);
            }

            Ok(())
        }

        /// get the crates from the top of each stack, ordered by stack name.
        pub fn get_top_crates(&self) -> String {
            let mut crates = self
                .0
                .iter()
                .map(|(&k, v)| (k, v.last().unwrap()))
                .collect::<Vec<_>>();
            crates.sort_by_key(|(k, _)| k.to_string());

            crates.into_iter().map(|(_, v)| v.0).collect()
        }
    }

    #[derive(Debug)]
    pub struct Instruction<'a> {
        amount: usize,
        from: &'a str,
        to: &'a str,
    }

    impl<'a> TryFrom<&'a str> for Instruction<'a> {
        type Error = ();

        fn try_from(value: &'a str) -> Result<Self, Self::Error> {
            // expected format: 'move 1 from 2 to 3'
            let mut parts = value.split_ascii_whitespace();
            let amount = parts
                .nth(1)
                .expect("amount from instruction")
                .parse()
                .expect("amount to be a number");

            Ok(Self {
                amount,
                from: parts.nth(1).expect("'from' from instruction"),
                to: parts.nth(1).expect("'to' from instruction"),
            })
        }
    }
}

pub fn day05p1(input: &str) -> String {
    let (drawing, procedure) = input
        .split_once("\n\n")
        .expect("Input must contain drawing and procedure");

    let mut stacks = Stacks::from(drawing);

    procedure.lines().for_each(|line| {
        let instruction = Instruction::try_from(line).expect("read instruction");
        stacks
            .execute_single_crate(instruction)
            .expect("execute instruction");
    });

    stacks.get_top_crates()
}

pub fn day05p2(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!("CMZ", &day05p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!("MCD", &day05p2(INPUT));
    }

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
}
