fn count_crabs(input: &[i32]) -> Vec<i32> {
    let max = *input.into_iter().max().unwrap();
    let mut crabs = vec![0; (max + 1) as usize];

    // Count crabs in each position
    for i in input {
        let e = crabs.get_mut(*i as usize).unwrap();
        *e += 1;
    }

    crabs
}

fn cost_for(distance: i32) -> i32 {
    if distance < 2 {
        return distance;
    }

    distance + cost_for(distance - 1)
}

pub fn day07p1(input: &[i32]) -> i32 {
    let crabs = count_crabs(input);

    let mut lowest = 0;
    for i in 0..=(crabs.len() as i32) {
        let cost: i32 = crabs
            .iter()
            .enumerate()
            .map(|(pos, crabs)| ((pos as i32) - i).abs() * crabs)
            .sum();
        if cost < lowest || lowest == 0 {
            lowest = cost;
        }
    }

    lowest
}

pub fn day07p2(input: &[i32]) -> i32 {
    let crabs = count_crabs(input);

    let mut lowest = 0;
    for i in 0..=(crabs.len() as i32) {
        let cost: i32 = crabs
            .iter()
            .enumerate()
            .map(|(pos, crabs)| cost_for(((pos as i32) - i).abs()) * crabs)
            .sum();
        if cost < lowest || lowest == 0 {
            lowest = cost;
        }
    }

    lowest
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static [i32] = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn part1_examples() {
        assert_eq!(37, day07p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(168, day07p2(INPUT));
    }

    #[test]
    fn cost_for_distance() {
        assert_eq!(0, cost_for(0));
        assert_eq!(1, cost_for(1));
        assert_eq!(1 + 2, cost_for(2));
        assert_eq!(1 + 2 + 3, cost_for(3));
        assert_eq!(1 + 2 + 3 + 4, cost_for(4));
    }
}
