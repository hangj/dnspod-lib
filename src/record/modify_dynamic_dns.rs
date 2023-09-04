use serde::Deserialize;
use serde::Serialize;

use crate::data_types::Action;
use crate::data_types::Integer;
use crate::data_types::RecordLine;
use crate::ExtractAction;

/// 更新动态 DNS 记录
/// https://cloud.tencent.com/document/api/1427/56158
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyDynamicDNS {
    /// 域名
    /// 示例值：dnspod.cn
    Domain: String,
    /// 主机记录，如 www，如果不传，默认为 @。
    /// 示例值：www
    SubDomain: String,
    /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 [RecordId]
    /// 示例值：162
    RecordId: u64,
    /// 记录线路，通过 API 记录线路获得，中文，比如：默认。
    /// 示例值：默认
    RecordLine: RecordLine,
    /// 记录值，如 IP : 200.200.200.200， CNAME : cname.dnspod.com.， MX : mail.dnspod.com.。
    /// 示例值：200.200.200.200
    Value: String,
    /// TTL值，如果不传，默认为域名的TTL值。
    /// 示例值：600
    Ttl: Integer,
}

impl ModifyDynamicDNS {
    pub fn new(
        domain: impl Into<String>,
        subdomain: Option<String>,
        record_id: u64,
        value: impl Into<String>,
    ) -> Self {
        let subdomain = match subdomain {
            Some(s) => s,
            None => "".into(),
        };

        Self {
            Domain: domain.into(),
            SubDomain: subdomain.into(),
            RecordId: record_id,
            RecordLine: RecordLine::default(),
            Value: value.into(),
            Ttl: 60,
        }
    }
}

impl ExtractAction for ModifyDynamicDNS {
    fn to_action() -> Action {
        Action::ModifyDynamicDNS
    }
}
