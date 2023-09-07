use clap::Parser;
use dnspod_lib::action::Action;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
    #[arg(long, env = "DNSPOD_SECRET_ID")]
    pub secret_id: String,
    #[arg(long, env = "DNSPOD_SECRET_KEY")]
    pub secret_key: String,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
