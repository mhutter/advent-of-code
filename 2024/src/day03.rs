type I<'i> = &'i [u8];

const PREFIX: I = b"mul(";

// SAFETY: It is assumed that the input consists of valid ASCII characters only.

pub fn day03p1(mut input: I) -> u32 {
    let mut sum = 0;

    while !input.is_empty() {
        // Find the next occurence of `PREFIX`.
        let Some(i) = input.windows(PREFIX.len()).position(|w| w == PREFIX) else {
            // `PREFIX` was not found, we're probably done.
            break;
        };

        // Skip over `PREFIX`.
        input = &input[(i + PREFIX.len())..];

        let (rest, left) = take_number(input);
        input = rest;
        let Some(left) = left else {
            // No number was foud, move on.
            continue;
        };

        // Expect a comma here.
        if input[0] == b',' {
            input = &input[1..];
        } else {
            continue;
        }

        let (rest, right) = take_number(input);
        input = rest;
        let Some(right) = right else {
            // No number was foud, move on.
            continue;
        };

        // Expect a closing bracket here.
        if input[0] == b')' {
            input = &input[1..];
        } else {
            continue;
        }

        sum += left * right;
    }

    sum
}

fn take_number(mut input: I) -> (I, Option<u32>) {
    let mut buf = String::new();

    // Read at most 3 digits
    for _ in 0..3 {
        let c = input[0];
        match c {
            b'0'..=b'9' => {
                buf.push(c as char);
                input = &input[1..];
            }
            _ => break,
        }
    }
    if buf.is_empty() {
        // No digits were found.
        return (input, None);
    }

    let num = u32::from_str_radix(&buf, 10).expect("parse u32");

    (input, Some(num))
}

pub fn day03p2(_input: I) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox() {
        let mut foo: &[u8] = &[1, 2, 3];
        foo = &foo[1..];
        assert_eq!(foo, &[2, 3]);

        let mut bar: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let pos = bar.windows(3).position(|w| w == &[3, 4, 5]).unwrap();
        assert_eq!(pos, 2);
        bar = &bar[pos..];
        assert_eq!(bar, &[3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn part1_examples() {
        assert_eq!(161, day03p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day03p2(INPUT));
    }

    const INPUT: &[u8] = b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
}
