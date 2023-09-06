use anyhow::{anyhow, Result};

/// 获取本机的公网 IP
/// https://4.ipw.cn => 192.168.1.1
/// https://test.ipw.cn => 192.168.1.1
/// https://ipinfo.io/ip => 192.168.1.1
/// https://ipecho.net/plain => 192.168.1.1 (如果有 IPv6 地址的话, 则返回 IPv6)
/// https://httpbin.org/ip => {"origin": "192.168.1.1"}
/// http://6.ipw.cn => 有 IPv6 地址则返回, 否则出错
pub fn get_public_ip() -> Result<String> {
    const URLS: &[&'static str] = &[
        "https://4.ipw.cn",
        "https://test.ipw.cn",
        "https://ipinfo.io/ip",
    ];

    let i = rand::random::<usize>() % URLS.len();
    let url = URLS[i];

    let res = reqwest::blocking::get(url)?;
    if !res.status().is_success() {
        return Err(anyhow!("HTTP status: {}", res.status()));
    }

    let ip = res.text()?;

    Ok(ip)
}
