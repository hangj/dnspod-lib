# dnspoddns

```console
cargo install ddnspod

ddnspod -h
A ddns cli for dnspod

Usage: ddnspod [OPTIONS] --secret-id <SECRET_ID> --secret-key <SECRET_KEY> --domain <DOMAIN>

Options:
      --secret-id <SECRET_ID>    [env: DNSPOD_SECRET_ID=]
      --secret-key <SECRET_KEY>  [env: DNSPOD_SECRET_KEY=]
  -d, --domain <DOMAIN>          
  -s, --subdomain <SUBDOMAIN>    [default: ]
  -v, --value <VALUE>            IP 地址, 默认自动获取公网 IP
  -h, --help                     Print help
  -V, --version                  Print version
```

You may also see [dnspod-cli](https://crates.io/crates/dnspod-cli)

