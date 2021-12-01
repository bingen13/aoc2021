extern crate nom;
use nom::{
    bytes::complete::{take_till, take_while},
    character::complete::{anychar, digit1},
    IResult,
};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Policy {
    min: u8,
    max: u8,
    c: char,
    password: String,
}
impl Policy {
    fn verify(&self) -> bool {
        (self.password.chars().nth((self.min - 1).into()).unwrap() == self.c)
            ^ (self.password.chars().nth((self.max - 1).into()).unwrap() == self.c)
    }
}

fn get_elem(i: &str) -> IResult<&str, &str> {
    take_till(separator)(i)
}

fn separator(c: char) -> bool {
    c == '-' || c == ' ' || c == ':'
}

fn get_separator(i: &str) -> IResult<&str, &str> {
    take_while(separator)(i)
}

fn get_bound(i: &str) -> IResult<&str, &str> {
    digit1(i)
}

fn parse_policy(i: &str) -> IResult<&str, Policy> {
    // Get minimum.
    let (i, elem) = get_elem(i)?;
    let (_, min) = get_bound(elem)?;
    let min = min.parse::<u8>().unwrap();
    // separator
    let (i, _) = get_separator(i)?;
    // Get maximum.
    let (i, elem) = get_elem(i)?;
    let (_, max) = get_bound(elem)?;
    let max = max.parse::<u8>().unwrap();
    // separator
    let (i, _) = get_separator(i)?;
    // Get key character.
    let (i, c) = anychar(i)?;
    // Separator and the rest is the password.
    let (password, rest) = get_separator(i)?;
    Ok((
        rest,
        Policy {
            min: min,
            max: max,
            c: c,
            password: password.to_string(),
        },
    ))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut count = 0;
    let mut entries = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for i in lines {
            if let Ok(i) = i {
                if let Ok((_, p)) = parse_policy(&i) {
                    entries.push(p);
                }
            }
        }
    }
    for i in entries.iter() {
        if i.verify() {
            count += 1
        }
    }
    println!("{}", count);
}
