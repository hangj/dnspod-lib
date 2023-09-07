#![doc = include_str!("../README.md")]

mod args;
mod utils;

use anyhow::Result;

use args::Args;
use dnspod_lib::prelude::*;
use dnspod_lib::response::{RecordListItem, Response};

#[allow(non_snake_case)]
fn main() -> anyhow::Result<()> {
    let Args {
        secret_id,
        secret_key,
        domain,
        subdomain,
        value,
    } = Args::parse_args();

    let value = value.unwrap_or(utils::get_public_ip()?);
    let subdomain = if subdomain.is_empty() {
        None
    } else {
        Some(subdomain)
    };

    let client = Client::new(secret_id, secret_key);

    let res = client.execute(DescribeRecordList {
        Domain: domain.clone(),
        Subdomain: subdomain,
        Keyword: None,
    })?;

    let record_list = res
        .Response
        .RecordList
        .ok_or(anyhow::anyhow!("No record list returned!"))?;
    if record_list.is_empty() {
        return Err(anyhow::anyhow!("record list is empty!"));
    }

    let RecordListItem {
        RecordId,
        Value,
        Name: SubDomain,
        Type: _,
        ..
    } = record_list[0].clone();

    // IP 地址没有变化
    if value == Value {
        return Ok(());
    }
    let Value = value;

    client.execute(ModifyDynamicDNS {
        Domain: domain,
        SubDomain,
        RecordId,
        RecordLine: dnspod_lib::data_types::RecordLine::默认,
        Value,
        Ttl: 60,
    })?;

    Ok(())
}

struct Client {
    secret_id: String,
    secret_key: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(secret_id: String, secret_key: String) -> Self {
        Self {
            secret_id,
            secret_key,
            client: reqwest::blocking::Client::new(),
        }
    }
    pub fn execute(&self, request: impl ExtractCommonParams) -> Result<Response> {
        let secret_id = self.secret_id.as_str();
        let secret_key = self.secret_key.as_str();

        let client = &self.client;

        let url = request.url();
        let body = request.body();
        let headers = request.headers(&secret_id, &secret_key);
        let headers = (&headers).try_into()?;

        let request = client.post(url).headers(headers).body(body).build()?;

        let res: Response = client.execute(request)?.json()?;

        if res.Response.Error.is_some() {
            let err = dnspod_lib::serde_json::to_string_pretty(&res)?;
            return Err(anyhow::anyhow!("{}", err));
        }

        Ok(res)
    }
}
