use std::error::Error;

use clap::{arg, command};

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

mod map;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([day] "Day to run"))
        .arg(arg!(
            -t --test ... "Use test input"
        ))
        .get_matches();

    let days = [d1::run, d2::run, d3::run, d4::run, d5::run];

    // You can check the value provided by positional arguments, or option arguments
    if let Some(day) = matches.get_one::<String>("day") {
        println!("Running day {day}");
        let num = day.parse::<usize>()?;

        days[num - 1]()?;
    }

    Ok(())
}
