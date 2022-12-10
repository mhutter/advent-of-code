use crt::*;

mod crt {
    use log::debug;

    /// An instruction from the input, either `noop` or `addx N`
    #[derive(Debug)]
    pub enum Instruction {
        Noop,
        AddX(i64),
    }
    impl Instruction {
        /// Parse a line and return an instruction from it
        ///
        /// Panics on invalid input.
        pub fn parse(i: &str) -> Self {
            let words = i.split_ascii_whitespace().collect::<Vec<_>>();
            match words.as_slice() {
                ["noop"] => Self::Noop,
                ["addx", n] => Self::AddX(n.parse().unwrap()),
                _ => unreachable!("Invalid input: {i}"),
            }
        }

        /// Determine the number of cycles an instruction takes
        pub fn cycles(&self) -> u64 {
            match self {
                Self::Noop => 1,
                Self::AddX(_) => 2,
            }
        }
    }

    /// Models the internal state of our CPU
    #[derive(Debug)]
    pub struct Cpu {
        /// The current cycle we're on
        cycle: u64,

        /// The X register
        x: i64,

        /// Accumulator of all the samples
        acc: i64,

        /// when the next sample must be taken
        ///
        /// (in cycle 20 and every 40th after that)
        next_sample: u64,
    }
    impl Cpu {
        /// Correctly initialize a CPU as per problem statement
        ///
        /// X is set to 1, and the next sample will be in cycle 20.
        pub fn new() -> Self {
            Self {
                acc: 0,
                cycle: 0,
                next_sample: 20,
                x: 1,
            }
        }

        /// Exectute the given instruction
        pub fn execute(&mut self, i: Instruction) {
            debug!("{self:?} {i:?}");

            for _ in 0..i.cycles() {
                // Increase the cycle counter
                self.cycle += 1;

                // If we passed `next_sample`, we must take a sample
                if self.cycle >= self.next_sample {
                    // determine this cycle's signal strength
                    let sig_str = self.next_sample as i64 * self.x;
                    // Add together all measurements
                    self.acc += sig_str;
                    // determine when the next measurement must be
                    self.next_sample += 40;
                }
            }

            match i {
                Instruction::Noop => {}
                Instruction::AddX(n) => self.x += n,
            }
        }

        /// Return the current value of the `acc` register
        pub fn get_acc(&self) -> i64 {
            self.acc
        }
    }
}

pub fn day10p1(input: &str) -> i64 {
    let mut cpu = Cpu::new();

    input
        .lines()
        .map(Instruction::parse)
        .for_each(|i| cpu.execute(i));

    cpu.get_acc()
}

pub fn day10p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(13140, day10p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day10p2(INPUT));
    }

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
