use besmlib;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "BESM-6 Simulator")]
pub struct Cmdline {
    #[structopt(flatten)]
    options: besmlib::Options,
}

pub fn main() -> Result<(), std::io::Error> {
    let options = Cmdline::from_args().options;

    let _result = besmlib::run(options, &mut std::io::stdout())?;
    Ok(())
}
