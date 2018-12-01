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
}