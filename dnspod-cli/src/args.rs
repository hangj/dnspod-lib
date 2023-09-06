use clap::Parser;
use dnspod_lib::prelude::*;

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
pub struct Args {
    // #[subcommand(value_enum, default_value_t=CliAction(Action::DescribeRecordList))]
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
