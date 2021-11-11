pub fn day02p1(input: &str) -> i32 {
    input.lines().map(paper_for).sum()
}

pub fn day02p2(input: &str) -> i32 {
    input.len() as i32
}

fn paper_for(dimensions: &str) -> i32 {
    let mut parts = dimensions.split("x");
    let l = parts.next().unwrap().parse::<i32>().unwrap();
    let w = parts.next().unwrap().parse::<i32>().unwrap();
    let h = parts.next().unwrap().parse::<i32>().unwrap();
    let s = [l * w, l * h, w * h];

    let total: i32 = s.iter().sum();
    let slack = s.iter().min().unwrap();

    slack + 2 * total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(58, paper_for("2x3x4"));
        assert_eq!(43, paper_for("1x1x10"));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day02p2(&[]))
    }
}
