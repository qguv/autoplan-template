/// week 01: 01-parsing
use autoplan::SASPlus;

test_each_file::test_each_file! { for ["in", "ans"] in "./tests/samples/01-parsing" as p01_parsing => test }

fn test([input, expected]: [&'static str; 2]) {
    let (rest, (problem, state)) = SASPlus::parse(input).expect("parse error");
    let (_, observed) = problem.query(&state, rest).expect("query error");
    assert_eq!(expected, observed);
}
