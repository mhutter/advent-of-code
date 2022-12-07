use input::*;

mod input {
    use std::{collections::HashMap, str::FromStr};

    #[derive(Debug)]
    pub enum Line {
        Command(Command),
        Entry(Entry),
    }
    impl FromStr for Line {
        type Err = color_eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(if let Some(cmd) = s.try_strip_prefix("$ ") {
                Command::from_str(cmd)?.into()
            } else {
                Entry::from_str(s)?.into()
            })
        }
    }
    impl From<Command> for Line {
        fn from(c: Command) -> Self {
            Self::Command(c)
        }
    }
    impl From<Entry> for Line {
        fn from(e: Entry) -> Self {
            Self::Entry(e)
        }
    }

    #[derive(Debug)]
    pub enum Command {
        Cd(String),
        CdRoot,
        CdUp,
        Ls,
    }
    impl FromStr for Command {
        type Err = color_eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s == "ls" {
                return Ok(Self::Ls);
            }

            s.try_strip_prefix("cd ")
                .map(|whence| match whence {
                    "/" => Self::CdRoot,
                    ".." => Self::CdUp,
                    _ => Self::Cd(whence.into()),
                })
                .ok_or(color_eyre::eyre::eyre!("Invalid command: {s}"))
        }
    }

    #[derive(Debug)]
    pub enum Entry {
        Dir(String),
        File(String, usize),
    }
    impl FromStr for Entry {
        type Err = color_eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.try_strip_prefix("dir ")
                .map(|dir| Self::Dir(dir.into()))
                .or_else(|| {
                    let (size, name) = s.split_once(' ').expect("file size and name");
                    Some(Self::File(
                        name.into(),
                        size.parse().expect("size to be a valid usize"),
                    ))
                })
                .ok_or(color_eyre::eyre::eyre!("Invalid Entry: {s}"))
        }
    }

    #[derive(Debug, Default)]
    pub struct Dir {
        files: HashMap<String, usize>,
        subdirs: HashMap<String, Self>,
        size: usize,
    }

    impl Dir {
        pub fn insert_at(&mut self, path: &[String], entry: Entry) {
            if let Entry::File(_, size) = entry {
                self.size += size;
            }

            if path.is_empty() {
                match entry {
                    Entry::Dir(name) => {
                        self.subdirs.entry(name).or_default();
                    }
                    Entry::File(name, size) => {
                        self.files.insert(name, size);
                    }
                };
                return;
            }

            let subdir = path[0].clone();
            let path = &path[1..];
            self.subdirs
                .entry(subdir)
                .or_default()
                .insert_at(path, entry)
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn all_sizes(&self) -> Vec<usize> {
            self.subdirs
                .iter()
                .fold(Vec::from([self.size]), |mut acc, (_, dir)| {
                    acc.append(&mut dir.all_sizes());
                    acc
                })
        }
    }

    impl FromStr for Dir {
        type Err = color_eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut stack: Vec<String> = Vec::new();
            let mut root = Self::default();

            s.lines()
                .map(Line::from_str)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .for_each(|line| match line {
                    Line::Command(Command::CdRoot) => stack.clear(),
                    Line::Command(Command::CdUp) => {
                        stack.pop();
                    }
                    Line::Command(Command::Cd(dir)) => stack.push(dir),
                    Line::Command(Command::Ls) => (),
                    Line::Entry(entry) => root.insert_at(&stack, entry),
                });

            Ok(root)
        }
    }

    trait TryStripPrefixExt {
        fn try_strip_prefix(&self, prefix: &str) -> Option<&Self>;
    }
    impl TryStripPrefixExt for str {
        fn try_strip_prefix(&self, prefix: &str) -> Option<&str> {
            self.starts_with(prefix)
                .then(|| self.trim_start_matches(prefix))
        }
    }
}

pub fn day07p1(input: &str) -> color_eyre::Result<usize> {
    let root: Dir = input.parse()?;
    Ok(root.all_sizes().into_iter().filter(|&s| s <= 100000).sum())
}

pub fn day07p2(_input: &str) -> color_eyre::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        let root: input::Dir = INPUT.parse().unwrap();
        assert_eq!(48381165, root.size());
    }

    #[test]
    fn part1_examples() {
        assert_eq!(95437, day07p1(INPUT).unwrap());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day07p2(INPUT).unwrap());
    }

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
}
