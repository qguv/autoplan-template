use nom::IResult;

use super::SASPlus;
use super::State;

impl SASPlus {
    pub fn parse(s: &str) -> IResult<&str, (SASPlus, State)> {
        todo!();
    }
}
