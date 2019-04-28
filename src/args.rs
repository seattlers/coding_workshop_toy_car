use structopt::StructOpt;

#[cfg(test)]
mod unit_tests;

/// Args is a data structure representing the user's supplied command-line arguments supplied to the program.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {
//    /// Optional argument indicating absence or presence and amount of `some_feature`.
//    pub some_arg: Option<usize>,
}
