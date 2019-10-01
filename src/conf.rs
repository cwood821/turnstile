use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "turnstile", about = "CI helper utility")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(short = "k", long = "key", default_value = "")]
    pub key: String,

    #[structopt(short = "v", long = "value", default_value = "0")]
    pub value: u64,
}