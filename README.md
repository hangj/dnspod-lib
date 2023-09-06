# DNSPod 

该库使用 [腾讯云 DNSPod API 3.0](https://docs.dnspod.cn/api/api3/) 版本, 不兼容旧版 API  

[DNSPod 简介](https://cloud.tencent.com/document/api/1427/56193)  


如果你在找 dnspod 的命令行工具, 请到 [dnspod-cli](https://crates.io/crates/dnspod-cli)

# Examples

```rust
extern crate anyhow;
extern crate reqwest;
extern crate dnspod_lib;

use anyhow::Result;
use dnspod_lib::prelude::*;

fn main() -> Result<()> {
    let res = execute(
        DescribeDomainList {
            Type: DomainType::ALL,
            Offset: 0,
            Limit: 0,
            GroupId: 0,
            Keyword: None,
        }
        .into()
    )?;
    println!("res: {:#?}", res);

    let res = execute(
        DescribeRecordList {
            Domain: "youran.de".into(),
            Subdomain: None,
            Keyword: None,
        }
        .into(),
    )?;
    println!("res: {:#?}", res);

    Ok(())
}

fn execute(request: Action) -> Result<Response> {
    let client = reqwest::blocking::Client::new();

    let secret_id = std::env::var("DNSPOD_SECRET_ID")?;
    let secret_key = std::env::var("DNSPOD_SECRET_KEY")?;

    let url = request.url();
    let body = request.body();
    let headers = request.headers(&secret_id, &secret_key);
    let headers = (&headers).try_into()?;

    let request = client
        .post(url)
        .headers(headers)
        .body(body)
        .build()?;

    let res: Response = client
        .execute(request)?
        .json()?;

    Ok(res)
}
```

## Run

```console
 DNSPOD_SECRET_ID=your-secret-id DNSPOD_SECRET_KEY=your-secret-key cargo run
```


