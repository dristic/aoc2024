use std::{error::Error, fs};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{anychar, char},
        is_digit,
    },
    combinator::value,
    multi::{many0, many_till},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Clone)]
enum Value {
    Mul((i32, i32)),
    Do,
    Dont,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/d3.txt")?;

    let (_, values) = parse_muls(&content).unwrap();

    let ans = values
        .iter()
        .filter_map(|v| {
            if let Value::Mul((n1, n2)) = v {
                Some(n1 * n2)
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("Part1 {ans}");

    let (_, values) = parse_muls(&content).unwrap();

    let mut enabled = true;
    let mut ans = 0;

    for val in values {
        match val {
            Value::Mul((n1, n2)) => if enabled { ans += n1 * n2; },
            Value::Do => enabled = true,
            Value::Dont => enabled = false,
        }
    }

    println!("Part2 {ans}");

    Ok(())
}

fn parse_muls(input: &str) -> IResult<&str, Vec<Value>> {
    many0(next_mul)(input)
}

fn next_mul(input: &str) -> IResult<&str, Value> {
    let (input, (_, found)) = many_till(anychar, val)(input)?;
    Ok((input, found))
}

fn val(input: &str) -> IResult<&str, Value> {
    if let Ok((input, (n1, _, n2))) = mul(input) {
        Ok((
            input,
            Value::Mul((n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap())),
        ))
    } else {
        alt((
            value(Value::Do, tag("do()")),
            value(Value::Dont, tag("don't()")),
        ))(input)
    }
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
