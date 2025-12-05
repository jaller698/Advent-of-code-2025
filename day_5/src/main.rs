use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let allowed_range = parse_allowed_ranges(&input);
    let number_of_ingredients = find_amount_of_valid_ingredients(&allowed_range);
    println!("Amount of valid ingredients: {}", number_of_ingredients);
}

fn parse_allowed_ranges(input: &str) -> Vec<(i64, i64)> {
    // parse all lines as integer ranges until first empty line
    let allowed_range = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let mut parts = line.split('-');
            let start = parts.next()?.parse::<i64>().ok()?;
            let end = parts.next()?.parse::<i64>().ok()?;
            Some((start, end))
        })
        .fold(Vec::new(), |mut ranges, (start, end)| {
            ranges.push((start, end));
            ranges
        });

    allowed_range
}

fn parse_ingredients(input: &str) -> Vec<i64> {
    // parse all lines after first empty line as integers
    let ingredients = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1) // skip the empty line itself
        .filter_map(|line| line.trim().parse::<i64>().ok())
        .fold(Vec::new(), |mut ingredients, value| {
            ingredients.push(value);
            ingredients
        });

    ingredients
}

fn find_valid_ingredients(allowed_ranges: &[(i64, i64)], ingredients: &[i64]) -> Vec<i64> {
    ingredients
        .iter()
        .cloned()
        .filter(|&ingredient| {
            allowed_ranges
                .iter()
                .any(|&(start, end)| ingredient >= start && ingredient <= end)
        })
        .collect()
}

fn get_all_valid_ingredients(allowed_ranges: &[(i64, i64)]) -> i64 {
    if allowed_ranges.is_empty() {
        return 0;
    }

    let mut ranges = allowed_ranges.to_vec();
    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(i64, i64)> = vec![];
    for &(start, end) in &ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    merged.iter().map(|(start, end)| end - start + 1).sum()
}

fn find_amount_of_valid_ingredients(allowed_ranges: &[(i64, i64)]) -> i64 {
    get_all_valid_ingredients(allowed_ranges)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_parse_input() {
        let allowed_range = parse_allowed_ranges(INPUT);
        assert_eq!(allowed_range, vec![(3, 5), (10, 14), (16, 20), (12, 18)]);
        let ingredients = parse_ingredients(INPUT);
        assert_eq!(ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_find_valid_ingredients() {
        let allowed_range = parse_allowed_ranges(INPUT);
        let ingredients = parse_ingredients(INPUT);
        let valid_ingredients = find_valid_ingredients(&allowed_range, &ingredients);
        assert_eq!(valid_ingredients, vec![5, 11, 17]);
        let total: i64 = valid_ingredients.len().try_into().unwrap();
        assert_eq!(total, 3);
    }

    #[test]
    fn test_find_amount_of_valid_ingredients() {
        let allowed_range = parse_allowed_ranges(INPUT);
        let amount = find_amount_of_valid_ingredients(&allowed_range);
        assert_eq!(amount, 14);
    }
}
