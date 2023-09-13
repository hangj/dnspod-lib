# dnspod-cli

A command line tool for dnspod

# Install

```console
cargo install dnspod-cli
```

# Usage

```console
dnspod-cli -h
A command line tool for dnspod

Usage: dnspod-cli --secret-id <SECRET_ID> --secret-key <SECRET_KEY> <COMMAND>

Commands:
  record  记录相关
  domain  域名相关
  help    Print this message or the help of the given subcommand(s)

Options:
      --secret-id <SECRET_ID>    [env: DNSPOD_SECRET_ID=]
      --secret-key <SECRET_KEY>  [env: DNSPOD_SECRET_KEY=]
  -h, --help                     Print help
  -V, --version                  Print version
```

目前只添加了关于域名和记录的部分 subcommand, 如果你想添加更多, 请看 [dnspod-lib](https://crates.io/crates/dnspod-lib/#%E8%87%AA%E5%AE%9A%E4%B9%89%E4%B8%80%E4%B8%AA%E8%AF%B7%E6%B1%82)   

通过 `define_action_list` 宏可以很方便地添加其它接口请求

