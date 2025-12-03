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
        let lo_len = range.0.to_string().len();
        let hi_len = range.1.to_string().len();
        for length in lo_len..=hi_len {
            if !check_all_duplicates {
                result += sum(range.0, range.1, length);
            } else {
                result += sum2(range.0, range.1, length as u32) as u64;
            }
        }
        continue;
    }
    result
}

fn parse(number: &str) -> u64 {
    if number.len() == 0 || number.as_bytes()[0] == b'0' {
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

fn sum_first_n(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn sum_range(start: u64, end: u64) -> u64 {
    sum_first_n(end) - sum_first_n(start - 1)
}

fn sum(start: u64, end: u64, length: usize) -> u64 {
    if length % 2 != 0 {
        return 0;
    }
    let lo_half = 10_u64.pow(((length / 2) - 1) as u32);
    let hi_half = 10 * lo_half - 1;

    let min = if length < start.to_string().len() {
        lo_half
    } else {
        let min = start / (lo_half * 10);
        if (min * (10 * lo_half + 1)) < start {
            min + 1
        } else {
            min
        }
    };

    let max = if length < end.to_string().len() {
        hi_half
    } else {
        let max = end / (lo_half * 10);
        if (max * (10 * lo_half + 1)) > end {
            max - 1
        } else {
            max
        }
    };
    sum_range(min, max) * (10 * lo_half + 1)
}

fn sum_rep(lo: u64, hi: u64, length: u32, rep: u32) -> u128 {
    // discount any repetition lengths which are not a factor of length
    if rep == 0 || length % rep != 0 {
        return 0;
    }

    // In the original Python, `rep` is rep-length, then reassigned to "number of repetitions"
    // here we keep both meanings separate for clarity.
    let rep_len = rep as u128; // length of each repeating block (parameter)
    let length_u128 = length as u128;
    let reps = length_u128 / rep_len; // number of repetitions

    // pattern length in digits:
    let pattern_len = length_u128 / reps; // == rep_len, but we keep the logic close to Python

    // 10 ** (pattern_len - 1)
    let lo_rep = pow10((pattern_len - 1) as u32);
    let hi_rep = 10 * lo_rep - 1;

    let lo_u = lo as u128;
    let hi_u = hi as u128;

    // what's the smallest invalid number that's >= lo and has this length and this degree of repetition?
    let min_pattern = if length_u128 > num_digits(lo_u) {
        lo_rep
    } else {
        let base = pow10(pattern_len as u32); // == lo_rep * 10
        let divisor = base.pow((reps - 1) as u32); // (lo_rep * 10) ** (reps - 1)

        let mut min_ = lo_u / divisor;

        // build the repeated number from this candidate pattern
        let mut tmp = min_;
        for _ in 0..(reps - 1) {
            tmp = tmp * base + min_;
        }
        if tmp < lo_u {
            min_ += 1;
        }
        min_
    };

    // what's the largest invalid number that's <= hi and has this length and this degree of repetition?
    let max_pattern = if length_u128 < num_digits(hi_u) {
        hi_rep
    } else {
        let base = pow10(pattern_len as u32); // == lo_rep * 10
        let divisor = base.pow((reps - 1) as u32);

        let mut max_ = hi_u / divisor;

        let mut tmp = max_;
        for _ in 0..(reps - 1) {
            tmp = tmp * base + max_;
        }
        if tmp > hi_u {
            max_ -= 1;
        }
        max_
    };

    if min_pattern > max_pattern {
        return 0;
    }

    // So now I know the range of invalid partial numbers, I must sum that range
    // and then multiply by (lo_rep * 10 + 1)**(rep - 1)
    let mut ret = sum_range_2(min_pattern, max_pattern);
    let adder = ret;
    let base = 10 * lo_rep;

    for _ in 0..(reps - 1) {
        ret = ret * base + adder;
    }

    ret
}

fn sum2(lo: u64, hi: u64, length: u32) -> u128 {
    // maximum repetition length to consider
    let max_rep = length / 2;

    // array (vector) for the sums of invalid numbers for each repetition length
    let mut rep_counts = vec![0u128; (max_rep + 1) as usize];

    // fill rep_counts[l] = sum_rep(lo, hi, length, l)
    for l in 1..=max_rep {
        rep_counts[l as usize] = sum_rep(lo, hi, length, l);
    }

    // variable to accumulate the final result
    let mut ret: u128 = 0;

    // iterate over all possible repetition lengths
    for l in 1..=max_rep {
        if length % l != 0 {
            continue;
        }

        // add the sum for this repetition length
        ret += rep_counts[l as usize];

        // subtract sums for any shorter repetition length that divides l (to avoid double-counts)
        for x in 1..l {
            if l % x == 0 {
                ret -= rep_counts[x as usize];
            }
        }
    }

    ret
}

/// Inclusive sum of [a, b]
fn sum_range_2(a: u128, b: u128) -> u128 {
    if a > b {
        return 0;
    }
    let n = b - a + 1;
    (a + b) * n / 2
}

/// Number of decimal digits in n
fn num_digits(mut n: u128) -> u128 {
    if n == 0 {
        return 1;
    }
    let mut d = 0;
    while n > 0 {
        d += 1;
        n /= 10;
    }
    d
}

/// 10^exp for exp up to 38-ish safely in u128
fn pow10(exp: u32) -> u128 {
    10u128.pow(exp)
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
