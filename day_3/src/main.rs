use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut lines: Vec<String> = input
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let mut total_sum: i64 = 0;
    for line in lines {
        total_sum += find_highest_comb(line);
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
