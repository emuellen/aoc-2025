advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut start = 50;
    let mut counter = 0;
    for change in Parser::new(input.as_bytes()) {
        start = (start + change) % 100;
        if start == 0 {
            counter += 1;
        }
    }
    Some(counter as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut last_zero = false;
    let mut start = 50;

    for change in Parser::new(input.as_bytes()) {
        start += change;

        if start <= 0 || start >= 100 {
            result += start.abs() / 100;
            if !last_zero && start <= 0 {
                result += 1;
            }
            start %= 100;
            if start < 0 {
                start += 100;
            }
        }
        last_zero = start == 0;
    }

    Some(result as u64)
}

struct Parser<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.buf.len() {
            return None;
        }
        let slice = &self.buf[self.pos..];
        let i = slice
            .iter()
            .position(|&c| c == b'\n')
            .unwrap_or(slice.len());
        self.pos += i + 1;
        Some(parse_line(&slice[..i]))
    }
}

fn parse_line(bytes: &[u8]) -> i32 {
    if bytes.len() < 2 {
        return 0;
    }
    let sign = ((bytes[0] & 2) as i32) - 1;
    let mut num = 0;
    for b in &bytes[1..] {
        if b.is_ascii_digit() {
            num = num * 10 + (b - b'0') as i32;
        }
    }
    sign * num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1084));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6475));
    }
}
