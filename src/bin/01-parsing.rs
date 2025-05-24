/// week 01: 01-parsing
use autoplan::SASPlus;
use std::{
    error::Error,
    io::{Read as _, stdin},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = String::new();
    stdin().read_to_string(&mut data)?;
    let s: &str = data.as_ref();

    // can't pass error upward, because the error might refer to the buffer allocated _in_ this function
    // so we use an expect and panic
    let (s, (problem, state)) = SASPlus::parse(s).expect("could not parse SAS+");
    let (_, out) = problem.query(&state, s).expect("query error");

    print!("{}", out);
    Ok(())
}
