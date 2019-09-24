use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "turnstile", about = "CI helper utility")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// URL to send HTTP POST request to when a checked url responds non-200 
    #[structopt(short = "k", long = "key", default_value = "")]
    pub key: String,

    /// Maximum number of concurrent HTTP requests
    #[structopt(short = "v", long = "value", default_value = "0")]
    pub value: usize,

    /// URL to send HTTP POST request to when a checked url responds non-200 
    #[structopt(short = "e", long = "env", default_value = "")]
    pub env: String,
}