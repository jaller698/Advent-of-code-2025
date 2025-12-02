use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut lines: Vec<String> = input
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    // Determine starting position: if the first non-empty line is a number, use it.
    let mut pos: usize = 50; // default start
    if !lines.is_empty() && lines[0].chars().all(|c| c.is_ascii_digit()) {
        pos = lines.remove(0).parse::<usize>().expect("parse start") % 100;
    }

    let mut total_hits = 0usize;

    for raw in lines {
        // allow "L68", "L 68", "R1000", etc.
        let token = raw.split_whitespace().collect::<Vec<_>>().join("");
        if token.len() < 2 {
            eprintln!("ignoring malformed line: {}", raw);
            continue;
        }
        let dir = token.chars().next().unwrap();
        let steps: usize = token[1..].parse().expect("parse steps");

        // compute t0: the first click count (>=1 and <=100) at which dial hits zero
        let t0 = match dir {
            'R' | 'r' => {
                let t = (100usize + 100 - (pos % 100)) % 100; // (100 - pos) % 100
                if t == 0 { 100 } else { t }
            }
            'L' | 'l' => {
                let t = pos % 100;
                if t == 0 { 100 } else { t }
            }
            _ => panic!("unexpected direction (use R or L): {}", dir),
        };

        if steps >= t0 {
            total_hits += 1 + (steps - t0) / 100;
        }

        // update position
        pos = match dir {
            'R' | 'r' => (pos + steps) % 100,
            'L' | 'l' => (pos + 100 - (steps % 100)) % 100,
            _ => unreachable!(),
        };
    }

    println!("{}", total_hits);
}
