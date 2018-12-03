mod bitmap;
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
        let counts: Vec<_> = input.split('\n')
            .map(|l| parse::count_repeated_characters(l))
            .collect();
        
        let twice = (&counts).into_iter()
            .filter(|s| s.contains(&2))
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        let thrice = (&counts).into_iter()
            .filter(|s| s.contains(&3))
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        
        let checksum = twice * thrice;
        assert_eq!(5952, checksum);
    }

    #[test]
    fn day02b() {
        let input = load("02a.txt");
        let mut lines: Vec<&str> = input.split('\n').collect();
        
        let mut a = String::new();
        let mut b = String::new();
        
        loop {
            let l = lines.clone();
            let (first, rest) = match l.split_first() {
                Some(x) => x,
                None => break,
            };
            lines = rest.to_vec();
            
            let found = rest.into_iter()
                .filter(|r| parse::differing_character_count(first, r) == 1)
                .next();
            match found {
                Some(other) => {
                    a = first.to_string();
                    b = other.to_string();
                    break;
                },
                None => (),
            }
        }

        let common = parse::same_characters(&a, &b);

        assert_eq!("krdmtuqjgwfoevnaboxglzjph", common);
    }

    #[test]
    fn day03a() {
        let input = load("03a.txt");
        let claims: Vec<parse::FabricClaim> = input.split('\n')
            .map(|line| parse::FabricClaim::from_str(line))
            .collect();
        
        let (mut w, mut h) = (0, 0);
        for claim in &claims {
            let cw = claim.x + claim.w;
            let ch = claim.y + claim.h;
            if cw > w { w = cw; }
            if ch > h { h = ch; }
        }

        let mut bitmap: bitmap::Bitmap<u16> = bitmap::Bitmap::new(w, h, 0);
        for claim in &claims {
            bitmap.draw_rectangle(claim.x, claim.y, claim.w, claim.h, |x| x + 1);
        }

        let overlaps = bitmap.field.into_iter()
            .filter(|x| *x > 1)
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        
        assert_eq!(124850, overlaps);
    }
}
