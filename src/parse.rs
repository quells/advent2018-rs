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
pub fn count_repeated_characters(src: &str) -> HashSet<usize> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for c in src.chars() {
        let count = char_counts.get(&c).unwrap_or(&0) + 1;
        char_counts.insert(c, count);
    }

    char_counts.into_iter()
        .map(|(_, count)| count)
        .filter(|x| *x > 1)
        .collect()
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
        // (input, contains_twice, contains_thrice)
        let test_vectors: Vec<(&str, bool, bool)> = vec![
            ("abcdef", false, false),
            ("bababc", true, true),
            ("abbcde", true, false),
            ("abcccd", false, true),
            ("aabcdd", true, false),
            ("abcdee", true, false),
            ("ababab", false, true),
        ];

        for (input, expect_twice, expect_thrice) in test_vectors {
            let counts = count_repeated_characters(input);
            assert_eq!(expect_twice, counts.contains(&2));
            assert_eq!(expect_thrice, counts.contains(&3));
        }
    }

    #[test]
    fn test_contains_repeated_characters_checksum() {
        let lines = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
        let counts: Vec<HashSet<usize>> = lines.into_iter()
            .map(|l| count_repeated_characters(l))
            .collect();
        
        let twice = (&counts).into_iter()
            .filter(|s| s.contains(&2))
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        let thrice = (&counts).into_iter()
            .filter(|s| s.contains(&3))
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        
        assert_eq!(4, twice);
        assert_eq!(3, thrice);
    }
}