#[allow(unused_macros)]
pub mod arg_parser {

    use std::collections::HashMap;
    use std::result;

    #[derive(Debug)]
    pub enum ArgParseError {
        ArgAlreadyExists,
    }

    struct Argument {
        desc: String,
        val: Vec<String>,
    }

    impl Argument {
        fn new(d: &str) -> Argument {
            Argument { desc: d.to_string(), val: Vec::new() }
        }
    }

    pub struct ArgumentParser {
        args: HashMap<String, Argument>,
    }

    impl ArgumentParser {
        pub fn new() -> ArgumentParser {
            ArgumentParser { args: HashMap::new() }
        }

        pub fn add_arg(&mut self, flag: &str, desc: &str) -> Result<(),ArgParseError>{
            match self.args.contains_key(flag) {
                false => {
                    self.args.insert(flag.to_string(), Argument::new(desc));
                    Ok(())
                },
                true => Err(ArgParseError::ArgAlreadyExists),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn create_argparser() {
            let _a: ArgumentParser = ArgumentParser::new();
        }

        #[test]
        fn add_argument() {
            let mut a: ArgumentParser = ArgumentParser::new();
            match a.add_arg("-a", "Does nothing, really.") {_ => (),};
            match a.add_arg("-a", "Does nothing, really.") {_ => (),};
        }
    }
}
