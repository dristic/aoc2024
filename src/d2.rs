use std::{error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/d2s.txt")?;

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for line in content.lines() {
        let parts: Vec<_> = line.split("   ").collect();
        v1.push(parts[0].parse::<i32>()?);
        v2.push(parts[1].parse::<i32>()?);
    }

    Ok(())
}