pub fn day10p1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(find_corruption)
        .map(|e| match e {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unexpected error char: {}", e),
        })
        .sum()
}

pub fn day10p2(_input: &str) -> i32 {
    0
}

fn find_corruption(program: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in program.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let top = stack.pop().unwrap();
                if c != closing_bracket_for(top) {
                    return Some(c);
                }
            }
            _ => panic!("Unexpected char encountered: {}", c),
        }
    }

    None
}

fn closing_bracket_for(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid char: {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(26397, day10p1(INPUT));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, day10p2(INPUT));
    }

    #[test]
    fn test_find_corruption() {
        for program in valid_programs() {
            assert_eq!(None, find_corruption(program), "is corrupted: {}", program);
        }

        assert_eq!(Some(']'), find_corruption("(]"));
        assert_eq!(Some('>'), find_corruption("{()()()>"));
        assert_eq!(Some('}'), find_corruption("(((()))}"));
        assert_eq!(Some(')'), find_corruption("<([]){()}[{}])"));
    }

    fn valid_programs() -> Vec<&'static str> {
        vec![
            "([])",
            "{()()()}",
            "<([{}])>",
            "[<>({}){}[([])<>]]",
            "(((((((((())))))))))",
        ]
    }

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
}
