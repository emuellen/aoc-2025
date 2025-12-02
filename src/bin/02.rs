advent_of_code::solution!(2);

// 38158151648
pub fn part_one(input: &str) -> Option<u64> {
    Some(run(input, false))
}

// 45283684555
pub fn part_two(input: &str) -> Option<u64> {
    Some(run(input, true))
}

fn run(input: &str, check_all_duplicates: bool) -> u64 {
    let ranges: Vec<_> = input
        .split(',')
        .map(|range| {
            let x: Vec<&str> = range.split('-').collect();
            (parse(x[0]), parse(x[1]))
        })
        .collect();
    let mut result = 0;
    for range in ranges {
        for number in range.0..=range.1 {
            if is_repeated_pattern(&number.to_string(), check_all_duplicates) {
                result += number;
            }
        }
    }
    result
}

fn is_repeated_pattern(number: &str, check_all_duplicates: bool) -> bool {
    let bytes = number.as_bytes();
    let len = bytes.len();

    if len < 2 || (!check_all_duplicates && !len.is_multiple_of(2)) {
        return false;
    }

    let starting_point = if check_all_duplicates { 1 } else { len / 2 };

    for chunk_size in starting_point..=len / 2 {
        if !len.is_multiple_of(chunk_size) {
            continue;
        }

        let pattern = &bytes[..chunk_size];
        let mut i = chunk_size;

        while i < len {
            if &bytes[i..i + chunk_size] != pattern {
                break;
            }
            i += chunk_size;
        }

        if i == len {
            return true;
        }
    }

    false
}

fn parse(number: &str) -> u64 {
    if number.starts_with("0") {
        return 0;
    }
    let mut num = 0;
    for b in number.as_bytes() {
        if b.is_ascii_digit() {
            num = num * 10 + (b - b'0') as u64;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
