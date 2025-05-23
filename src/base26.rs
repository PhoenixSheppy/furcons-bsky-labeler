pub fn encode(mut num: u64) -> String {
    let mut result = String::new();
    while num > 0 {
        num -= 1;
        result.insert(0, (b'a' + (num % 26) as u8) as char);
        num /= 26;
    }
    result
}

pub fn decode(s: &str) -> Option<u64> {
    s.chars().try_fold(0, |acc, c| {
        if c < 'a' || c > 'z' {
            return None;
        }
        Some(acc * 26 + (c as u64 - 'a' as u64 + 1))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_decode() {
        for i in 0..20000 {
            let s = encode(i);
            println!("{i} -> {s}");
            assert_eq!(i, decode(&s).unwrap());
        }
    }
}
