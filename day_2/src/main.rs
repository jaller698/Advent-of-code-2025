use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut summed_id = 0;
    let fancy_regex = fancy_regex::Regex::new(r"^(\d+)\1+$").unwrap();

    // split the input by commas,
    for id in input.split(',') {
        let id_range = id.trim();
        let lower_bound: i64 = id_range
            .split('-')
            .next()
            .unwrap()
            .parse()
            .expect("parse lower bound");
        let upper_bound: i64 = id_range
            .split('-')
            .nth(1)
            .unwrap()
            .parse()
            .expect("parse upper bound");
        for id in lower_bound..=upper_bound {
            let id_str = id.to_string();
            for hit in fancy_regex.find_iter(&id_str) {
                let repeated_id: i64 = hit.unwrap().as_str().parse().unwrap();
                println!("Found repeated ID: {} in ID: {}", repeated_id, id_range);
                summed_id += repeated_id;
            }
        }
    }

    println!("{}", summed_id);
}
