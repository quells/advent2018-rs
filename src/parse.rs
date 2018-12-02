use std::collections::{HashSet, HashMap};
use std::str::FromStr;

#[allow(dead_code)]
pub fn signed_integer(src: &str) -> isize {
    if src.to_owned().len() < 2 {
        return 0;
    }

    let (sign_char, rest) = (*src).split_at(1);
    let sign: isize = match sign_char {
        "+" =>  1,
        "-" => -1,
        _ => return 0,
    };
    let unsigned = match isize::from_str(rest) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    
    sign * unsigned
}

#[allow(dead_code)]
pub fn contains_repeated_characters(src: &str, count: usize) -> bool {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for c in src.chars() {
        let count = char_counts.get(&c).unwrap_or(&0) + 1;
        char_counts.insert(c, count);
    }

    let counts: HashSet<usize> = char_counts.into_iter()
        .map(|(_, count)| count)
        .filter(|x| *x > 1)
        .collect();
    
    counts.contains(&count)
}

#[cfg(test)]
mod tests {
    use crate::parse::*;

    #[test]
    fn test_signed_integer() {
        let test_vectors: Vec<(&str, isize)> = vec![
            ("",      0),
            ("+1",    1),
            ("+22",  22),
            ("-3",   -3),
            ("-44", -44),
            ("+",     0),
            ("-",     0),
            ("abc",   0),
        ];

        for (input, expected) in test_vectors {
            assert_eq!(expected, signed_integer(input));
        }
    }

    #[test]
    fn test_contains_repeated_characters() {
        let test_vectors: Vec<(&str, usize, bool)> = vec![
            ("abcdef", 2, false), ("abcdef", 3, false),
            ("bababc", 2, true), ("bababc", 3, true),
            ("abbcde", 2, true), ("abbcde", 3, false),
            ("abcccd", 2, false), ("abcccd", 3, true),
            ("aabcdd", 2, true), ("aabcdd", 3, false),
            ("abcdee", 2, true), ("abcdee", 3, false),
            ("ababab", 2, false), ("ababab", 3, true),
        ];

        for (input, count, expected) in test_vectors {
            assert_eq!(expected, contains_repeated_characters(input, count));
        }
    }

    #[test]
    fn test_contains_repeated_characters_checksum() {
        let lines = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
        
        let twice = (&lines).into_iter()
            .filter(|l| contains_repeated_characters(l, 2))
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        
        let thrice = (&lines).into_iter()
            .filter(|l| contains_repeated_characters(l, 3))
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        
        assert_eq!(4, twice);
        assert_eq!(3, thrice);
    }
}