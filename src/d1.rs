use std::{error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/d1.txt")?;

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for line in content.lines() {
        let parts: Vec<_> = line.split("   ").collect();
        v1.push(parts[0].parse::<i32>().unwrap());
        v2.push(parts[1].parse::<i32>().unwrap());
    }

    v1.sort();
    v2.sort();

    let mut p1 = 0;
    for (i, n) in v1.iter().enumerate() {
        let dist = (n - v2[i]).abs();
        //println!("{} {} = {}", n, v2[i], dist);
        p1 += dist;
    }

    println!("Ans1 {}", p1);

    let mut p2 = 0;
    for num in v1.iter() {
        let count = v2.iter().filter(|&n| *n == *num).count();
        let similar = *num * count as i32;
        p2 += similar;
    }

    println!("Ans2 {}", p2);

    Ok(())
}