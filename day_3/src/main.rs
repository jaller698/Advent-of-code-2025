use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let lines: Vec<String> = input
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let mut total_sum: u64 = 0;
    for line in lines {
        println!("processing line: {line}");
        total_sum += find_highest_12digit_comb(line);
    }
    println!("total sum: {total_sum}");
}

fn find_highest_comb(line: String) -> i64 {
    let numbers = line.trim().split("");
    let mut highest_sum = 0;
    println!("checking line: {line}");

    for number in numbers {
        if number.is_empty() {
            continue;
        }
        let i_number: i64 = number.parse().unwrap_or(0);
        let index = line.find(number).unwrap_or(0);

        let line_copy = line.chars().skip(index + 1).collect::<String>();

        for number2 in line_copy.trim().split("") {
            if number2.is_empty() {
                continue;
            }
            let i_number2: i64 = number2.parse().unwrap();
            let sum = i_number * 10 + i_number2;
            if sum > highest_sum {
                highest_sum = sum;
            }
        }
    }
    println!("found sum {highest_sum} for line: {line}");
    highest_sum
}

pub fn find_highest_12digit_comb(line: String) -> u64 {
    let bytes: Vec<u8> = line.bytes().filter(|b| b"0123456789".contains(b)).collect();
    let n = bytes.len();
    let k = 12usize;

    if n < k {
        return 0;
    }

    let mut pos = 0usize;
    let mut result: u64 = 0;

    // Greedy: for each of the k output positions, pick the largest digit
    // that still leaves enough digits for the remaining positions.
    // This produces the lexicographically (hence numerically) largest k-length subsequence
    // and runs in O(n * k) in the worst case (with an early-out on finding '9').
    for i in 0..k {
        let remaining = k - i;
        let end = n - remaining; // inclusive upper bound for where the chosen digit can come from

        let mut best_digit = b'0';
        let mut best_idx = pos;

        // scan the window [pos ..= end] for the maximum digit (break early if 9)
        for j in pos..=end {
            let d = bytes[j];
            if d > best_digit {
                best_digit = d;
                best_idx = j;
                if best_digit == b'9' {
                    break; // can't do better than '9'
                }
            }
        }

        result = result * 10 + (best_digit - b'0') as u64;
        pos = best_idx + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_highest_comb_single_line() {
        let example_line = "987654321111111".to_string();
        let result = find_highest_comb(example_line);
        assert_eq!(result, 98);
    }

    #[test]
    fn test_find_highest_comb_all_lines() {
        let example_lines = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        let mut total_sum: i64 = 0;
        for line in example_lines {
            total_sum += find_highest_comb(line);
        }
        assert_eq!(total_sum, 357);
    }

    #[test]
    fn test_find_highest_12digit_comb() {
        let example_line = "987654321111111".to_string();
        let result = find_highest_12digit_comb(example_line);
        assert_eq!(result, 987654321111);
    }

    #[test]
    fn test_find_highest_12digit_comb_all_lines() {
        let example_lines = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        let mut total_sum: u64 = 0;
        for line in example_lines {
            total_sum += find_highest_12digit_comb(line);
        }
        assert_eq!(total_sum, 3121910778619);
    }
}
