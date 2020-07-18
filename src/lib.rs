// #![crate_type = "lib"]
// #![crate_name = "arg_parser"]

#[allow(unused_macros)]

// Remove when done?
#[allow(dead_code)]

/*
  ===================================
  TODO:
      * Write more docs lole
      * Alternate flags for full string args.
        - eg "-h" and "--help" and resolve to the same argument.
  ===================================
*/

pub mod arg_parser {
        use std::collections::HashMap;
        /// This is where the errors and warnings are defined for the library.
        #[derive(Debug)]
        pub enum Warning {
            FlagAlreadyExists,
        }

        #[derive(Debug)]
        pub enum Error {
            UnknownArgumentError
        }

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
        /// Creates a new argument parser with default "-h" flag.
        pub fn new() -> ArgumentParser {
            let mut a = ArgumentParser { args: HashMap::new() };
            a.add_arg("-h", "Print this message.");
            return a
        }

        /// This adds an arugment to the ArgumentParser.
        /// Returns an Option: None if everything is successful, a warning
        /// if something lame happens.
        pub fn add_arg(&mut self, flag: &str, desc: &str) -> Option<Warning> {
            match self.args.insert(flag.to_string(), Argument::new(desc)) {
                None => None,
                Some(_) => Some(Warning::FlagAlreadyExists),
            }
        }

        /// Prints the description
        pub fn print_desc(&mut self) {
            for e in &self.args {
                println!("Flag: {}, Description: {}", e.0, e.1.desc);
            }
        }

        /// Passes the arguments in a Vec<String> passed to it.
        pub fn parse_args(&mut self, a: Vec<String>) {
            todo!();
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
                    None => panic!("Got None, expected a warning: {:?}", Warning::FlagAlreadyExists),
                };
        }

        #[test]
        fn get_description() {
            let mut a: ArgumentParser = ArgumentParser::new();
            a.add_arg("-a", "Does something cool.");
            a.add_arg("-b", "Does something entirely different from the previous one.");
            a.print_desc();
        }

        #[test]
        fn parse_args() {
            let mut a: ArgumentParser = ArgumentParser::new();
            a.add_arg("-a", "Does nothing, really.");
            a.add_arg("-b", "Doesn't do much either.");
            a.parse_args(std::env::args().collect());
        }
    }
}
