#![doc = include_str!("../README.md")]

mod args;
use args::Args;

use anyhow::Result;
use dnspod_lib::prelude::*;

fn main() -> Result<()> {
    let Args {
        action,
        secret_id,
        secret_key,
    } = Args::parse_args();

    let res = execute(action, &secret_id, &secret_key)?;
    let res = serde_json::to_string_pretty(&res)?;
    println!("{}", res);

    Ok(())
}

fn execute(request: Action, secret_id: &str, secret_key: &str) -> Result<Response> {
    let client = reqwest::blocking::Client::new();

    let url = request.url();
    let body = request.body();
    let headers = request.headers(&secret_id, &secret_key);
    let headers = (&headers).try_into()?;

    let request = client.post(url).headers(headers).body(body).build()?;

    let res: Response = client.execute(request)?.json()?;

    Ok(res)
}
