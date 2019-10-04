use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "turnstile", about = "CI helper utility", rename_all="kebab-case")]
pub struct Opt {
    /// Fail on decrease in value 
    #[structopt(long)]
    pub decrease: bool,

    #[structopt(short = "k", long = "key", default_value = "")]
    pub key: String,

    #[structopt(short = "v", long = "value", default_value = "0")]
    pub value: u64,
}