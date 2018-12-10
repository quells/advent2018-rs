extern crate time;
extern crate regex;
extern crate rayon;

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

        use crate::bitmap::Bitmap;
        let mut bitmap: Bitmap<u16> = Bitmap::new(w, h, 0);
        for claim in &claims {
            bitmap.draw_rectangle(claim.x, claim.y, claim.w, claim.h, |x| x + 1);
        }

        let overlaps = bitmap.field.into_iter()
            .filter(|x| *x > 1)
            .map(|_| 1usize)
            .fold(0, |a, b| a + b);
        
        assert_eq!(124850, overlaps);
    }

    #[test]
    fn day03b() {
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

        use std::collections::HashSet;
        use crate::bitmap::Bitmap;
        let mut bitmap: Bitmap<HashSet<parse::FabricClaim>> = Bitmap::new(w, h, HashSet::new());

        // Paint the bitmap with *all* of the claims on each pixel
        for claim in &claims {
            bitmap.draw_rectangle(claim.x, claim.y, claim.w, claim.h, |pixel| {
                let mut copy = pixel.clone();
                copy.insert(*claim);
                copy
            });
        }

        // Identify claims which have been painted on the same pixel as other claims
        let mut to_delete = HashSet::new();
        for pixel in &bitmap.field {
            if pixel.len() > 1 {
                for claim in pixel {
                    to_delete.insert(claim);
                }
            }
        }

        // Un-paint claims marked to be deleted
        // This is an optimization over scanning the entire bitmap
        let mut mutable_copy = bitmap.clone();
        for claim in to_delete {
            mutable_copy.draw_rectangle(claim.x, claim.y, claim.w, claim.h, |p| {
                let mut copy = p.clone();
                copy.remove(&claim);
                copy
            });
        }
        
        // Grab claims which remain
        let mut remaining: HashSet<parse::FabricClaim> = HashSet::new();
        for pixel in mutable_copy.field {
            for claim in pixel {
                remaining.insert(claim);
            }
        }
        
        assert_eq!(1, remaining.len());

        let uncut = remaining.into_iter().next().unwrap();
        assert_eq!(1097, uncut.id);
    }

    #[test]
    fn day04a() {
        use std::collections::{BTreeSet, HashMap};
        use parse::{GuardEvent, GuardLog};
        type RowIndex = usize;
        type GuardId = usize;
        type Minute = usize;

        let input = load("04a.txt");
        let logs: BTreeSet<GuardLog> = input.split('\n')
            .map(|line| GuardLog::from_str(line))
            .collect();
        
        let unique_guard_ids: BTreeSet<GuardId> = (&logs).into_iter()
            .map(|log| {
                match log.e {
                    GuardEvent::BeginShift(id) => Some(id),
                    _ => None,
                }
            })
            .filter(|id| id.is_some())
            .map(|id| id.unwrap())
            .collect();
        let guard_ids: Vec<GuardId> = unique_guard_ids.into_iter().collect();
        let guard_indices: HashMap<GuardId, RowIndex> = (&guard_ids).into_iter()
            .enumerate()
            .map(|(row_idx, guard_id)| (*guard_id, row_idx))
            .collect();

        use crate::bitmap::Bitmap;
        let mut sleep_schedule = Bitmap::new(60, guard_ids.len(), 0usize);

        let mut current_guard: Option<GuardId> = None;
        let mut asleep: Option<Minute> = None;
        for log in logs {
            match log.e {
                GuardEvent::BeginShift(id) => current_guard = Some(id),
                GuardEvent::FallAsleep => asleep = Some(log.ts.tm_min as usize),
                GuardEvent::WakeUp => {
                    let sleep_duration = (log.ts.tm_min as usize) - asleep.unwrap();
                    let row_idx = (&guard_indices).get(&current_guard.unwrap()).unwrap();
                    &sleep_schedule.draw_rectangle(asleep.unwrap(), *row_idx, sleep_duration, 1, |x| x + 1);
                },
            }
        }

        let (sleepiest_row_index, _) = (&sleep_schedule).rows().into_iter()
            .map(|row| row.into_iter().sum())
            .enumerate()
            .fold((0, 0), |a, b| {
                let (_, a_count) = a;
                let (_, b_count) = b;
                if a_count > b_count {
                    a
                } else {
                    b
                }
            });
        let sleepiest_guard_id = guard_ids.get(sleepiest_row_index).unwrap();
        
        let schedule_rows = sleep_schedule.rows();
        let sleepiest_guard_schedule = schedule_rows.get(sleepiest_row_index).unwrap();
        let (sleepiest_minute, _) = sleepiest_guard_schedule.into_iter()
            .enumerate()
            .fold((0, &0), |a, b| {
                let (_, a_count) = a;
                let (_, b_count) = b;
                if a_count > b_count {
                    a
                } else {
                    b
                }
            });
        
        let answer = sleepiest_guard_id*sleepiest_minute;
        assert_eq!(95199, answer);
    }

    #[test]
    fn day04b() {
        use std::collections::{BTreeSet, HashMap};
        use parse::{GuardEvent, GuardLog};
        type RowIndex = usize;
        type GuardId = usize;
        type Minute = usize;

        let input = load("04a.txt");
        let logs: BTreeSet<GuardLog> = input.split('\n')
            .map(|line| GuardLog::from_str(line))
            .collect();
        
        let unique_guard_ids: BTreeSet<GuardId> = (&logs).into_iter()
            .map(|log| {
                match log.e {
                    GuardEvent::BeginShift(id) => Some(id),
                    _ => None,
                }
            })
            .filter(|id| id.is_some())
            .map(|id| id.unwrap())
            .collect();
        let guard_ids: Vec<GuardId> = unique_guard_ids.into_iter().collect();
        let guard_indices: HashMap<GuardId, RowIndex> = (&guard_ids).into_iter()
            .enumerate()
            .map(|(row_idx, guard_id)| (*guard_id, row_idx))
            .collect();

        use crate::bitmap::Bitmap;
        let mut sleep_schedule = Bitmap::new(60, guard_ids.len(), 0usize);

        let mut current_guard: Option<GuardId> = None;
        let mut asleep: Option<Minute> = None;
        for log in logs {
            match log.e {
                GuardEvent::BeginShift(id) => current_guard = Some(id),
                GuardEvent::FallAsleep => asleep = Some(log.ts.tm_min as usize),
                GuardEvent::WakeUp => {
                    let sleep_duration = (log.ts.tm_min as usize) - asleep.unwrap();
                    let row_idx = (&guard_indices).get(&current_guard.unwrap()).unwrap();
                    &sleep_schedule.draw_rectangle(asleep.unwrap(), *row_idx, sleep_duration, 1, |x| x + 1);
                },
            }
        }

        let (sleepiest_row_idx, (sleepiest_minute, _)) = sleep_schedule.rows().into_iter()
            .map(|row| {
                row.into_iter()
                    .enumerate()
                    .fold((0, 0), |a, b| {
                        let (_, a_count) = a;
                        let (_, b_count) = b;
                        if a_count > b_count {
                            a
                        } else {
                            b
                        }
                    })
            })
            .enumerate()
            .fold((0, (0, 0)), |a, b| {
                let (_, (_, a_count)) = a;
                let (_, (_, b_count)) = b;
                if a_count > b_count {
                    a
                } else {
                    b
                }
            });
        
        let sleepiest_guard_id = guard_ids.get(sleepiest_row_idx).unwrap();
        
        let answer = sleepiest_guard_id*sleepiest_minute;
        assert_eq!(7887, answer);
    }

    fn react(polymer: &[i8]) -> Vec<i8> {
        let mut polymer = polymer.to_vec();
        let mut idx = 0;

        loop {
            let copy = polymer.clone();
            if idx >= copy.len() - 1 {
                return polymer;
            }

            let a = copy.get(idx).unwrap();
            let b = copy.get(idx+1).unwrap();
            if a + b == 0 {
                let mut lhs = copy[.. idx].to_vec();
                let mut rhs = copy[idx+2 ..].to_vec();
                lhs.append(&mut rhs);
                polymer = lhs;
                if idx > 0 {
                    idx -= 1;
                }
                continue;
            }

            idx += 1;
        }
    }

    #[test]
    fn day05a() {
        let input = load("05a.txt");
        let polarized: Vec<i8> = input.into_bytes().into_iter()
            .map(|c| {
                let signed = c as i16;
                match c.is_ascii_uppercase() {
                    true => signed - ('A' as i16) + 1,
                    false => ('a' as i16) - signed - 1,
                }
            })
            .map(|c| c as i8)
            .collect();
        let reacted = react(&polarized);

        for (idx, a) in (&reacted).into_iter().enumerate() {
            if idx < reacted.len() - 1 {
                let b = -reacted.get(idx+1).unwrap();
                assert_ne!(*a, b);
            }
        }

        assert_eq!(11194, reacted.len());
    }

    #[test]
    fn day05b() {
        use rayon::prelude::*;
        
        let input = load("05a.txt");
        let polarized: Vec<i8> = input.into_bytes().into_iter()
            .map(|c| {
                let signed = c as i16;
                match c.is_ascii_uppercase() {
                    true => signed - ('A' as i16) + 1,
                    false => ('a' as i16) - signed - 1,
                }
            })
            .map(|c| c as i8)
            .collect();
        
        let inhibitors: Vec<i8> = (1 ..= 26).collect();
        
        let results: Vec<(i8, usize)> = inhibitors.to_vec().par_iter()
            .map(|inhibitor| {
                let subset: Vec<_> = (&polarized).into_iter().filter(|x| **x != *inhibitor && -**x != *inhibitor).map(|x| *x).collect();
                let reacted_len = react(&subset).len();
                (*inhibitor, reacted_len)
            })
            .collect();
        
        let (inhibitor, shortest) = results.into_iter()
            .fold((0, std::usize::MAX), |a, b| {
                let (_, a_length) = a;
                let (_, b_length) = b;
                if a_length < b_length {
                    a
                } else {
                    b
                }
            });

        assert_eq!(3, inhibitor);
        assert_eq!(4178, shortest);
    }
}
