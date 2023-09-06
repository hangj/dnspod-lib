# dnspod-cli

A command line tool for dnspod

# Install

```console
cargo install dnspod-cli
```

# Usage

```console
dnspod-cli -h
dnspod-cli

Usage: dnspod-cli --secret-id <SECRET_ID> --secret-key <SECRET_KEY> <COMMAND>

Commands:
  describe-domain-list  获取域名列表 https://cloud.tencent.com/document/api/1427/56172
  create-record         添加记录 https://cloud.tencent.com/document/api/1427/56180
  delete-record         删除记录 https://cloud.tencent.com/document/api/1427/56176
  describe-record-list  获取域名的解析记录列表 https://cloud.tencent.com/document/api/1427/56166
  describe-record       获取记录信息 https://cloud.tencent.com/document/api/1427/56168
  modify-dynamic-dns    更新动态 DNS 记录 https://cloud.tencent.com/document/api/1427/56158
  modify-record         修改记录 https://cloud.tencent.com/document/api/1427/56157
  help                  Print this message or the help of the given subcommand(s)

Options:
      --secret-id <SECRET_ID>    [env: DNSPOD_SECRET_ID=]
      --secret-key <SECRET_KEY>  [env: DNSPOD_SECRET_KEY=]
  -h, --help                     Print help
  -V, --version                  Print version
```

