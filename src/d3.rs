use std::{error::Error, fs};

use nom::{
    bytes::complete::{tag, take_while},
    character::{
        complete::{anychar, char},
        is_digit,
    },
    multi::{many0, many_till},
    sequence::{delimited, tuple},
    IResult,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/d3.txt")?;

    let (_, pairs) = parse(&content).unwrap();

    let ans = pairs
        .iter()
        .map(|(s1, _, s2)| {
            let n1 = s1.parse::<i32>().unwrap();
            let n2 = s2.parse::<i32>().unwrap();
            n1 * n2
        })
        .sum::<i32>();

    println!("Part1 {ans}");

    Ok(())
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, char, &str)>> {
    many0(next_mul)(input)
}

fn next_mul(input: &str) -> IResult<&str, (&str, char, &str)> {
    let (input, (_, found)) = many_till(anychar, mul)(input)?;
    Ok((input, found))
}

fn mul(input: &str) -> IResult<&str, (&str, char, &str)> {
    delimited(
        tag("mul("),
        tuple((
            take_while(is_digit_char),
            char(','),
            take_while(is_digit_char),
        )),
        tag(")"),
    )(input)
}

fn is_digit_char(chr: char) -> bool {
    is_digit(chr as u8)
}
