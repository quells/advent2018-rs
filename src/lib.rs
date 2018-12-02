mod parse;

use std::io::prelude::Read;

#[allow(dead_code)]
fn load(input_file: &str) -> String {
    let filename = std::path::Path::new("./src").join("input").join(input_file);
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day01a() {
        let input = load("01a.txt");
        let sum = input.split('\n')
            .map(|line| parse::signed_integer(&line))
            .fold(0, |a, b| a + b);
        assert_eq!(486, sum);
    }

    #[test]
    fn day01b() {
        use std::collections::HashSet;

        let input = load("01a.txt");
        let mut observed: HashSet<isize> = HashSet::new();
        let mut acc: isize = 0;

        // Frequency starts at acc
        observed.insert(acc);
        
        // Skip empty lines
        let deltas: Vec<isize> = input.split('\n')
            .filter(|line| line.len() > 0)
            .map(|line| parse::signed_integer(&line))
            .collect();
        
        // Repeat pattern until a frequency is observed twice
        'outer: for round in std::iter::repeat(deltas) {
            for x in round {
                acc += x;
                let is_new = observed.insert(acc);
                if !is_new {
                    break 'outer;
                }
            }
        }
        
        assert_eq!(69285, acc);
    }

    #[test]
    fn day02a() {
        let input = load("02a.txt");
        let lines: Vec<&str> = input.split('\n').collect();

        let twice = (&lines).into_iter()
            .filter(|l| parse::contains_repeated_characters(l, 2))
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        
        let thrice = (&lines).into_iter()
            .filter(|l| parse::contains_repeated_characters(l, 3))
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        
        let checksum = twice * thrice;
        assert_eq!(5952, checksum);
    }
}
