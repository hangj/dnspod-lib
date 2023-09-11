use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Args {
    #[arg(long, env = "DNSPOD_SECRET_ID")]
    pub secret_id: String,
    #[arg(long, env = "DNSPOD_SECRET_KEY")]
    pub secret_key: String,
    #[arg(short, long)]
    pub domain: String,
    #[arg(short, long, default_value = "")]
    pub subdomain: String,
    /// IP 地址, 默认自动获取公网 IP
    #[arg(short, long)]
    pub value: Option<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
