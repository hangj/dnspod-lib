#![doc = include_str!("../README.md")]

mod args;
use args::Args;

use anyhow::Result;
use dnspod_lib::{prelude::*, define_action_list, consts};

// 自定义一个代码中没有实现的请求
define_action_list! {
    /// 获取域名信息
    /// https://cloud.tencent.com/document/api/1427/56173
    @[url = consts::DNSPOD_URL] // 可以重载 url, version, region
    pub struct DescribeDomain {
        /// 域名分组类型，默认为ALL
        pub Domain: String,
    }
}

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

fn execute(request: impl ExtractCommonParams, secret_id: &str, secret_key: &str) -> Result<serde_json::Value> {
    let client = reqwest::blocking::Client::new();

    let url = request.url();
    let body = request.body();
    let headers = request.headers(&secret_id, &secret_key);
    let headers = (&headers).try_into()?;

    let request = client.post(url).headers(headers).body(body).build()?;

    let res: serde_json::Value = client.execute(request)?.json()?;

    Ok(res)
}
