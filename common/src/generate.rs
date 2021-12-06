use std::{fmt::Debug, str::FromStr};

/// Converts a list of integers with (optional) signs into a list of T's
///
/// # Examples
///
/// ```
/// use common::generate::ints;

/// assert_eq!(vec![0,2,3,-4], ints("0\n2\n+3\n-4\n"));
/// ```
pub fn ints<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input.lines().map(|l| l.parse().unwrap()).collect()
}

/// Convert an input into individual characters
///
/// # Examples
///
/// ```
/// use common::generate::chars;
///
/// assert_eq!(vec!['h', 'e', 'l', 'l', 'o'], chars("hello"));
/// ```
pub fn chars(input: &str) -> Vec<char> {
    input.chars().collect()
}
