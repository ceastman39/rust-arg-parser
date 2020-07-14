#![crate_name = "arg_parser"]
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

    use std::collections::HashMap;

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
        ///  Returns: a bool indicating if a value was overritten when inserting
        ///  into the ArgumentParser.
        ///  (ie: returns `true` if the `flag` string already existed as a key
        ///  in the ArgumentParser and was overwritten, `false` otherwise.)
        pub fn add_arg(&mut self, flag: &str, desc: &str) -> bool {
            match self.args.insert(flag.to_string(), Argument::new(desc)) {
                None => false,
                Some(_) => true,
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
                    false => (),
                    true => panic!("Doesn't return correct value when inserting duplicate arguments.")
                };
        }
    }
}
