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

pub fn day10p2(input: &str) -> u64 {
    let mut scores: Vec<u64> = input
        .lines()
        .filter_map(|program| parse(program).ok())
        .map(|stack| stack.into_iter().rev().map(closing_bracket_for).collect())
        .map(score_for)
        .collect();

    scores.sort();

    let n = (scores.len() - 1) / 2;
    scores[n]
}

type Stack = Vec<char>;
type ParseError = char;

fn parse(program: &str) -> Result<Stack, ParseError> {
    let mut stack = Vec::new();
    for c in program.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let top = stack.pop().unwrap();
                if c != closing_bracket_for(top) {
                    return Err(c);
                }
            }
            _ => panic!("Unexpected char encountered: {}", c),
        }
    }

    Ok(stack)
}

fn score_for(stack: Vec<char>) -> u64 {
    stack.into_iter().fold(0, |acc, c| {
        (acc * 5)
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Invalid c: {}", &c),
            }
    })
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
        assert_eq!(288957, day10p2(INPUT));
    }

    #[test]
    fn test_score_for() {
        assert_eq!(288957, score_for("}}]])})]".chars().collect()));
        assert_eq!(5566, score_for(")}>]})".chars().collect()));
        assert_eq!(1480781, score_for("}}>}>))))".chars().collect()));
        assert_eq!(995444, score_for("]]}}]}]}>".chars().collect()));
        assert_eq!(294, score_for("])}>".chars().collect()));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Ok(vec!['[', '(', '{', '(', '[', '[', '{', '{']),
            parse("[({(<(())[]>[[{[]{<()<>>")
        );

        assert_eq!(Err(']'), parse("(]"));
        assert_eq!(Err('>'), parse("{()()()>"));
        assert_eq!(Err('}'), parse("(((()))}"));
        assert_eq!(Err(')'), parse("<([]){()}[{}])"));
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

    #[test]
    fn test_is_corrupted() {
        for program in valid_programs() {
            assert!(!is_corrupted(program), "is corrupted: {}", program);
        }

        assert!(is_corrupted("(]"));
        assert!(is_corrupted("{()()()>"));
        assert!(is_corrupted("(((()))}"));
        assert!(is_corrupted("<([]){()}[{}])"));
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
