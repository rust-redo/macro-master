#[cfg(test)]
mod arg_tests {
    use macro_rules::{arg, Arg, ArgValueType};
    #[test]
    fn should_parse_bool() {
        assert_eq!(
            arg!(--help <bool> "Print help"),
            Arg {
                id: "help",
                pattern: "--help",
                value_type: ArgValueType::Bool,
                usage: "Print help"
            }
        )
    }

    #[test]
    fn should_parse_string() {
        assert_eq!(
            arg!(--file <string> "Search file path"),
            Arg {
                id: "file",
                pattern: "--file",
                value_type: ArgValueType::String,
                usage: "Search file path"
            }
        )
    }

    #[test]
    fn should_parse_number() {
        assert_eq!(
            arg!(--port <number> "TCP port"),
            Arg {
                id: "port",
                pattern: "--port",
                value_type: ArgValueType::Number,
                usage: "TCP port"
            }
        )
    }
}
