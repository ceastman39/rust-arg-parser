// #![crate_type = "lib"]
// #![crate_name = "arg_parser"]

#[allow(unused_macros)]

// Remove when done?
#[allow(dead_code)]

/*
  ===================================
  TODO:
      * Write more docs lole
  ===================================
*/

pub mod arg_parser {

    /// This is where the errors and warnings are defined for the
    //  `arg_parser` library.
    pub mod errors {
        #[derive(Debug)]
        pub enum Warning {
            PlaceholderWarning,
            FlagAlreadyExists,
        }

        #[derive(Debug)]
        pub enum Error {
            GenericError,
            PlaceholderError,
        }
    }

    use std::collections::HashMap;
    //use errors::*;



    /// The values to be put into the HashMap inside of ArgumentParser
    pub struct Argument {
        /// The description of what the argument does.
        /// This is used when printing the help modules, or if an
        /// invalid argument is used.
        desc: String,
        /// The actual argument values passed in.
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

        /// This adds an arugment to the ArgumentParser.
        /// Returns: an Option, None if everything is successful, a warning
        /// if something lame happens.
        pub fn add_arg(&mut self, flag: &str, desc: &str) -> Option<errors::Warning> {
            match self.args.insert(flag.to_string(), Argument::new(desc)) {
                None => None,
                Some(_) => Some(errors::Warning::FlagAlreadyExists),
            }
        }

        /// Resturns the formatted description of the program.
        pub fn get_desc(&mut self) -> &str {
            "Not implemented lole"
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
            a.add_arg("-a", "Does nothing, really.");
            match a.add_arg("-a", "Does nothing, really.") {
                    Some(w) => println!("successfully got warning: {:?}", w),
                    None => panic!("Got None, expected a warning: {:?}", errors::Warning::FlagAlreadyExists),
                };
        }

        #[test]
        fn get_description() {
            let mut a: ArgumentParser = ArgumentParser::new();
            a.add_arg("-b", "Some description.");
            println!("{}", a.get_desc());
        }
    }
}
