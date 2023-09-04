use serde::Deserialize;
use serde::Serialize;

use crate::data_types::*;
use crate::ExtractAction;

/// 添加记录
/// https://cloud.tencent.com/document/api/1427/56180
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRecord {
    /// 域名
    /// 示例值：dnspod.cn
    Domain: String,
    /// 主机记录，如 www，如果不传，默认为 @。
    /// 示例值：www
    SubDomain: String,
    /// 记录类型，通过 API 记录类型获得，大写英文，比如：A 。
    /// 示例值：A
    RecordType: RecordType,
    /// 记录线路，通过 API 记录线路获得，中文，比如：默认。
    /// 示例值：默认
    RecordLine: RecordLine,
    /// 记录值，如 IP : 200.200.200.200， CNAME : cname.dnspod.com.， MX : mail.dnspod.com.。
    /// 示例值：200.200.200.200
    Value: String,
}

impl CreateRecord {
    /// subdomain 主机记录，如 www，如果不传，默认为 @。
    pub fn new(
        domain: impl Into<String>,
        subdomain: Option<String>,
        value: impl Into<String>,
        record_type: RecordType,
    ) -> Self {
        let subdomain = if subdomain.is_none() {
            "".into()
        } else {
            subdomain.unwrap()
        };

        Self {
            Domain: domain.into(),
            SubDomain: subdomain,
            RecordType: record_type,
            RecordLine: RecordLine::默认,
            Value: value.into(),
        }
    }

    pub fn record_line(mut self, record_line: RecordLine) -> Self {
        self.RecordLine = record_line;

        self
    }
}

impl ExtractAction for CreateRecord {
    fn to_action() -> Action {
        Action::CreateRecord
    }
}
