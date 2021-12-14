use std::collections::HashMap;

#[derive(Debug)]
struct Cave {
    id: String,
    small: bool,
    links: Vec<String>,
}

impl From<String> for Cave {
    fn from(id: String) -> Self {
        let small = id == id.to_lowercase();
        Self::new(id, small)
    }
}

impl Cave {
    fn new(id: String, small: bool) -> Self {
        let links = Vec::new();
        Self { id, small, links }
    }

    /// visits all the linked caves in order, and returns how many times the "end" cave has been reached.
    fn visit(&self, system: &HashMap<String, Cave>, path: &mut Vec<String>) -> usize {
        if self.id == "end" {
            // reached the end, valid path!
            return 1;
        }

        if self.small && path.contains(&self.id) {
            // already been here
            return 0;
        }

        path.push(self.id.clone());
        let c = self
            .links
            .iter()
            .map(|k| {
                let cave = system.get(k).unwrap();
                cave.visit(system, path)
            })
            .sum();
        path.pop();

        c
    }
}

pub fn day12p1(input: &str) -> usize {
    let mut system: HashMap<String, Cave> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();

        system
            .entry(parts[0].to_string())
            .or_insert_with_key(|k| Cave::from(k.clone()))
            .links
            .push(parts[1].to_string());

        system
            .entry(parts[1].to_string())
            .or_insert_with_key(|k| Cave::from(k.clone()))
            .links
            .push(parts[0].to_string());
    }

    let mut path = Vec::new();
    let start = system.get("start").unwrap();
    start.visit(&system, &mut path)
}

pub fn day12p2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(10, day12p1(INPUT1));
        assert_eq!(19, day12p1(INPUT2));
        assert_eq!(226, day12p1(INPUT3));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day12p2(INPUT1));
    }

    const INPUT1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
    const INPUT2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
    const INPUT3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";
}
