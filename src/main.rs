use besmlib;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "BESM-6 Simulator")]
pub struct Cmdline {
    #[structopt(flatten)]
    options: besmlib::Options,
}

pub fn main() {
    let options = Cmdline::from_args().options;

    besmlib::run(options);
}
