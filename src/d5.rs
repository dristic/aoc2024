use std::{collections::HashMap, error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/d5.txt")?;

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = Vec::new();

    for line in content.lines() {
        if line.contains("|") {
            let parts = line.split("|").collect::<Vec<_>>();
            let n1 = parts[0].parse::<i32>().unwrap();
            let n2 = parts[1].parse::<i32>().unwrap();
            
            let entry = rules.entry(n1).or_default();
            entry.push(n2);
        } else if line.contains(",") {
            let vals = line
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(vals);
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for update in updates {
        let mut seen = Vec::new();
        let mut correct = true;
        for n in &update {
            if let Some(before) = rules.get(n) {
                // Make sure this is not before any numbers we have seen.
                for s in &seen {
                    if before.contains(s) {
                        correct = false;
                    }
                }
            }

            seen.push(*n);
        }

        if correct {
            let mid = update[update.len() / 2];
            part1 += mid;
        } else {
            let mid = fix(&rules, &update);
            part2 += mid;
        }
    }

    println!("Part1 {part1}");
    println!("Part2 {part2}");

    Ok(())
}

fn fix(rules: &HashMap<i32, Vec<i32>>, update: &[i32]) -> i32 {
    let mut fixed = Vec::new();

    for n in update {
        let mut idx = 0;

        if let Some(before) = rules.get(n) {
            for (i, f) in fixed.iter().enumerate() {
                if before.contains(f) {
                    idx = i;
                    break;
                }

                idx += 1;
            }
        } else {
            idx = fixed.len();
        }

        fixed.insert(idx, *n);
    }

    fixed[fixed.len() / 2]
}
