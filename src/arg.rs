#[derive(PartialEq, Debug)]
pub enum ArgValueType {
    String,
    Number,
    Bool
}

#[derive(PartialEq, Debug)]
pub struct Arg<'a> {
    id: &'a str,
    pattern: &'a str,
    value_type: ArgValueType,
    usage: &'a str,
}

/// description: arg! converts command argument pattern to Arg struct
/// 
/// e.g: arg!(--help <bool> "Print help") => Arg {
///     id: "help",
///     pattern: "--help",
///     value_type: ArgValueType::Bool,
///     usage: "Print help"
/// }
/// 
/// run test: `cargo run arg_tests`
#[macro_export]
macro_rules! arg {
    () => {
    };
}

#[cfg(test)]
pub mod arg_tests {
    use crate::{Arg, ArgValueType};

    #[test]
    fn should_parse_bool() {
        assert_eq!(arg!(--help <bool> "Print help"), Arg {
            id: "help",
            pattern: "--help",
            value_type: ArgValueType::Bool,
            usage: "Print help"
        })
    }

    #[test]
    fn should_parse_string() {
        assert_eq!(arg!(--file <string> "Search file path"), Arg {
            id: "file",
            pattern: "--file",
            value_type: ArgValueType::String,
            usage: "Search file path"
        })
    }

    #[test]
    fn should_parse_number() {
        assert_eq!(arg!(--port <number> "TCP port"), Arg {
            id: "port",
            pattern: "--port",
            value_type: ArgValueType::Number,
            usage: "TCP port"
        })
    }
}