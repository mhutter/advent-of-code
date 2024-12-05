type I<'i> = &'i [u8];
type N = u32;

const PREFIX: I = b"mul(";

const FN_MUL: I = b"mul(";
const FN_DO: I = b"do()";
const FN_DONT: I = b"don't()";

// SAFETY: It is assumed that the input consists of valid ASCII characters only.

pub fn day03p1(mut input: I) -> N {
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

fn take_number(mut input: I) -> (I, Option<N>) {
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

    let num = buf.parse::<N>().unwrap();

    (input, Some(num))
}

pub fn day03p2(mut input: I) -> N {
    let mut sum = 0;
    let mut enabled = true;

    while !input.is_empty() {
        match input {
            i if i.starts_with(FN_MUL) => {
                let (rest, product) = mul(input);
                input = rest;
                match product {
                    Some(p) if enabled => sum += p,
                    _ => {}
                }
            }
            i if i.starts_with(FN_DONT) => {
                enabled = false;
                input = &input[FN_DONT.len()..];
            }
            i if i.starts_with(FN_DO) => {
                enabled = true;
                input = &input[FN_DO.len()..];
            }
            _ => {
                input = &input[1..];
            }
        }
    }

    sum
}

fn mul(mut input: I) -> (I, Option<N>) {
    // Skip over `FN_MUL`
    input = &input[FN_MUL.len()..];

    let (mut input, left) = take_number(input);
    let Some(left) = left else {
        // No number was foud, move on.
        return (input, None);
    };

    // Expect a comma here.
    if input[0] == b',' {
        input = &input[1..];
    } else {
        return (input, None);
    }

    let (mut input, right) = take_number(input);
    let Some(right) = right else {
        // No number was foud, move on.
        return (input, None);
    };

    // Expect a closing bracket here.
    if input[0] == b')' {
        input = &input[1..];
    } else {
        return (input, None);
    }

    (input, Some(left * right))
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
        let pos = bar.windows(3).position(|w| w == [3, 4, 5]).unwrap();
        assert_eq!(pos, 2);
        bar = &bar[pos..];
        assert_eq!(bar, &[3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn part1_examples() {
        assert_eq!(161, day03p1(INPUT1));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(48, day03p2(INPUT2));
    }

    const INPUT1: &[u8] =
        b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &[u8] =
        b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
}
