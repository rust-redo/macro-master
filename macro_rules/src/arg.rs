#[derive(PartialEq, Debug)]
pub enum ArgValueType {
    String,
    Number,
    Bool,
}

#[derive(PartialEq, Debug)]
pub struct Arg<'a> {
    pub id: &'a str,
    pub pattern: &'a str,
    pub value_type: ArgValueType,
    pub usage: &'a str,
}

/// description: 
///     arg! converts command argument pattern to Arg struct,
///     check test cases in macro_rules/tests/arg.rs
///
/// e.g: 
///     arg!(--help <bool> "Print help") => Arg {
///         id: "help",
///         pattern: "--help",
///         value_type: ArgValueType::Bool,
///         usage: "Print help"
///     }
///
/// run test: 
///     `cd macro_rules && cargo run arg_tests`
#[macro_export]
macro_rules! arg {
   () => {}
}
