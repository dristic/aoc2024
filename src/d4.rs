use std::{error::Error, fs};

use crate::map::Map;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/d4.txt")?;

    let map: Map<char> = Map::from_str(&content);

    let mut ans = 0;
    for (i, c) in map.iter().enumerate() {
        let (x, y) = map.get_loc(i);

        if *c == 'X' {
            ans += DIRS.iter().filter(|&&d| test_xmas(&map, x, y, d)).count();
        }
    }

    println!("Part1 {ans}");

    let mut ans = 0;
    for (i, c) in map.iter().enumerate() {
        let (x, y) = map.get_loc(i);

        if *c == 'A' && test_x_mas(&map, x, y) {
            ans += 1;
        }
    }

    println!("Part2 {ans}");

    Ok(())
}

fn test_xmas(map: &Map<char>, x: i32, y: i32, d: (i32, i32)) -> bool {
    if let Some('M') = map.get_xy(x + d.0, y + d.1) {
        if let Some('A') = map.get_xy(x + (d.0 * 2), y + (d.1 * 2)) {
            if let Some('S') = map.get_xy(x + (d.0 * 3), y + (d.1 * 3)) {
                return true;
            }
        }
    }

    false
}

fn test_x_mas(map: &Map<char>, x: i32, y: i32) -> bool {
    // Check we can get each corner.
    if let Some(c1) = map.get_xy(x - 1, y - 1) {
        if let Some(c2) = map.get_xy(x + 1, y - 1) {
            if let Some(c3) = map.get_xy(x - 1, y + 1) {
                if let Some(c4) = map.get_xy(x + 1, y + 1) {
                    // Check that the characters are in the right spots.
                    if c1 == c2 || c2 == c4 || c4 == c3 || c3 == c1 {
                        // Check that we have the right characters.
                        let mut chars = [*c1, *c2, *c3, *c4];
                        chars.sort();
                        if chars == ['M', 'M', 'S', 'S'] {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}
