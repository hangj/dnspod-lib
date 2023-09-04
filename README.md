# DNSPod 

[DNSPod 简介](https://cloud.tencent.com/document/api/1427/56193)  


# Examples

```rust
extern crate anyhow;
extern crate reqwest;
extern crate dnspod_lib;

use anyhow::Result;
use dnspod_lib::prelude::*;

fn execute(request: impl ExtractHeaders) -> Result<Response> {
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

fn main() -> Result<()> {
    let res = execute(DescribeDomainList::default())?;
    println!("res: {:#?}", res);

    let res = execute(DescribeRecordList::new("example.com", None, None))?;
    println!("res: {:#?}", res);

    Ok(())
}
```


```console
 DNSPOD_SECRET_ID=your-secret-id DNSPOD_SECRET_KEY=your-secret-key cargo run
```


