use std::fmt::{Display, Formatter};

// add `mod` lines to include source files from the sasplus directory
mod parse;
mod query;

#[derive(Clone)]
pub struct SASPlus {
    // TODO
}

pub struct State {
    // TODO
    // alternatively: `type State = ...;`
}

impl Display for SASPlus {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        todo!();
    }
}
