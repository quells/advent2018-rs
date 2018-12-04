use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::{Once, ONCE_INIT};

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

#[inline(always)]
#[allow(dead_code)]
pub fn same_characters(a: &str, b: &str) -> String {
    let a = a.chars();
    let b = b.chars();

    a.into_iter().zip(b)
        .filter(|(a, b)| a == b)
        .map(|(a, _b)| a)
        .collect()
}

#[inline(always)]
#[allow(dead_code)]
pub fn differing_character_count(a: &str, b: &str) -> usize {
    let a = a.chars();
    let b = b.chars();

    a.into_iter().zip(b)
        .filter(|(a, b)| a != b)
        .map(|_| 1usize)
        .fold(0, |a, b| a + b)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct FabricClaim {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[allow(dead_code)]
impl FabricClaim {
    pub fn new(id: usize, x: usize, y: usize, w: usize, h: usize) -> FabricClaim {
        FabricClaim{id, x, y, w, h}
    }

    pub fn from_str(spec: &str) -> FabricClaim {
        // #123 @ 3,2: 5x4
        let parts: Vec<&str> = spec.split(' ')
            .filter(|part| part.to_string().len() > 1)
            .collect();
        
        let mut id_chars = parts[0].chars();
        id_chars.next();
        let id = usize::from_str(&id_chars.as_str()).unwrap();

        let pos: Vec<&str> = parts[1].split(',')
            .flat_map(|x| x.split_terminator(':'))
            .filter(|x| x.to_string().len() > 0)
            .collect();
        let x = usize::from_str(&pos[0]).unwrap();
        let y = usize::from_str(&pos[1]).unwrap();

        let size: Vec<usize> = parts[2].split('x')
            .map(|x| usize::from_str(x).unwrap())
            .collect();
        let w = size[0];
        let h = size[1];
        
        FabricClaim::new(id, x, y, w, h)
    }

    pub fn area(&self) -> usize {
        self.w * self.h
    }
}

impl std::fmt::Debug for FabricClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}", self.id, self.x, self.y, self.w, self.h)
    }
}

use time::{Tm, strptime};
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GuardEvent {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

#[derive(Clone, PartialEq, Eq)]
pub struct GuardLog {
    pub ts: Tm,
    pub e: GuardEvent,
}

impl std::fmt::Debug for GuardLog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self.e {
            GuardEvent::BeginShift(id) => format!("#{} began shift", id),
            GuardEvent::FallAsleep => "fell asleep".to_string(),
            GuardEvent::WakeUp => "woke up".to_string(),
        };
        write!(f, "[{}] {}", self.ts.strftime("%Y-%m-%d %H:%M").unwrap(), message)
    }
}

impl PartialOrd for GuardLog {
    fn partial_cmp(&self, other: &GuardLog) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GuardLog {
    fn cmp(&self, other: &GuardLog) -> Ordering {
        self.ts.cmp(&other.ts)
    }
}

static INIT_GUARDLOG_MATCHER: Once = ONCE_INIT;
static mut GUARDLOG_MATCHER: Option<Regex> = None;
static mut GUARDEVENT_MATCHER: Option<Regex> = None;

impl GuardLog {
    pub fn from_str(s: &str) -> GuardLog {
        INIT_GUARDLOG_MATCHER.call_once(|| {
            unsafe {
                GUARDLOG_MATCHER = Some(Regex::new(r"^\[([0-9\- :]+)\] (.+)$").unwrap());
                GUARDEVENT_MATCHER = Some(Regex::new(r"#([0-9]+)").unwrap());
            }
        });
        /*
        [1518-08-17 00:01] Guard #1021 begins shift
        [1518-03-16 00:39] falls asleep
        [1518-03-10 00:56] wakes up
        */
        let glm = unsafe { GUARDLOG_MATCHER.clone().unwrap() };
        let time_event = glm.captures(s).unwrap();
        let gem = unsafe { GUARDEVENT_MATCHER.clone().unwrap() };
        
        let ts = strptime(&time_event[1], "%Y-%m-%d %H:%M").unwrap();
        
        let event = &time_event[2];
        let e = match gem.captures(&event) {
            Some(cap) => {
                let id = usize::from_str(&cap[1]).unwrap();
                GuardEvent::BeginShift(id)
            },
            None => {
                match event {
                    "wakes up" => GuardEvent::WakeUp,
                    _ => GuardEvent::FallAsleep,
                }
            }
        };

        GuardLog{ts, e}
    }
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

    #[test]
    fn test_same_characters() {
        let test_vectors: Vec<(&str, &str, &str)> = vec![
            ("abcde", "fghij", ""),
            ("abcde", "axcye", "ace"),
            ("fghij", "fguij", "fgij"),
            ("abcde", "abcde", "abcde"),
        ];

        for (a, b, expected) in test_vectors {
            assert_eq!(expected, same_characters(a, b));
        }
    }

    #[test]
    fn test_differing_characters() {
        let test_vectors: Vec<(&str, &str, usize)> = vec![
            ("abcde", "fghij", 5),
            ("abcde", "axcye", 2),
            ("fghij", "fguij", 1),
            ("abcde", "abcde", 0),
        ];

        for (a, b, expected) in test_vectors {
            assert_eq!(expected, differing_character_count(a, b));
        }
    }

    #[test]
    fn test_fabric_claim_from_str() {
        let test_vectors: Vec<(&str, FabricClaim)> = vec![
            ("#1 @ 123,456: 12x34", FabricClaim::new(1, 123, 456, 12, 34)),
        ];

        for (input, expected) in test_vectors {
            assert_eq!(expected, FabricClaim::from_str(input));
        }
    }

    #[test]
    fn test_fabric_claim_area() {
        let test_vectors: Vec<(&str, usize)> = vec![
            ("#1 @ 123,456: 12x34", 12*34),
        ];

        for (input, expected) in test_vectors {
            assert_eq!(expected, FabricClaim::from_str(input).area());
        }
    }

    #[test]
    fn test_guard_log_from_str() {
        println!("{:?}", GuardLog::from_str("[1518-08-17 00:01] Guard #1021 begins shift"));
        println!("{:?}", GuardLog::from_str("[1518-03-16 00:39] falls asleep"));
        println!("{:?}", GuardLog::from_str("[1518-03-10 00:56] wakes up"));
    }
}