use std::path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Options {
    /// Enable trace output
    #[structopt(short = "t", long = "trace")]
    trace: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Input file
    #[structopt(name = "input.oct", parse(from_os_str))]
    input: path::PathBuf,
}

pub fn run(options: Options) {
    println!("{:?}", options);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
