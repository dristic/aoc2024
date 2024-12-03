use std::{error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/d2.txt")?;

    let mut reports = Vec::new();

    for line in content.lines() {
        let parts = line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        reports.push(parts);
    }

    let safe = reports
        .iter()
        .map(|v| is_safe(v, None))
        .filter(|r| *r)
        .count();

    println!("Ans1 {safe}");

    let safe = reports
        .iter()
        .map(|v| is_safe_dampener(v))
        .filter(|r| *r)
        .count();

    println!("Ans2 {safe}");

    Ok(())
}

fn is_safe(levels: &[i32], skip: Option<usize>) -> bool {
    let mut last_delta = None;
    let mut last = None;
    let skip = skip.unwrap_or(usize::MAX);

    for (i, n) in levels.iter().enumerate() {
        if i == skip {
            continue;
        }

        if let Some(last) = last {
            let delta: i32 = n - last;

            if delta.abs() > 3 || delta.abs() < 1 {
                return false;
            }

            if let Some(last_delta) = last_delta {
                if delta ^ last_delta < 0 {
                    return false;
                }
            }

            last_delta = Some(delta);
        }

        last = Some(n);
    }

    true
}

fn is_safe_dampener(levels: &[i32]) -> bool {
    if !is_safe(levels, None) {
        let result = levels
            .iter()
            .enumerate()
            .map(|(i, _)| is_safe(levels, Some(i)))
            .any(|r| r);

        return result;
    }

    true
}
