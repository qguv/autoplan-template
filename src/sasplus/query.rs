use nom::IResult;

use super::SASPlus;
use super::State;

impl SASPlus {
    pub fn query<'a>(&self, state: &State, s: &'a str) -> IResult<&'a str, String> {
        todo!();
    }
}
