use serde::Deserialize;
use serde::Serialize;

use crate::data_types::Action;
use crate::data_types::RecordLine;
use crate::data_types::RecordType;
use crate::ExtractAction;

/// 修改记录
/// https://cloud.tencent.com/document/api/1427/56157
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyRecord {
    /// 域名
    /// 示例值：dnspod.cn
    Domain: String,
    /// 主机记录，如 www，如果不传，默认为 @。
    /// 示例值：www
    SubDomain: String,
    /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 [RecordId]
    /// 示例值：162
    RecordId: u64,
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

impl ModifyRecord {
    /// subdomain 主机记录，如 www，如果不传，默认为 @。
    pub fn new(
        domain: impl Into<String>,
        record_id: u64,
        subdomain: Option<String>,
        value: impl Into<String>,
        record_type: RecordType,
    ) -> Self {
        let subdomain = match subdomain {
            Some(s) => s,
            None => "".into(),
        };

        Self {
            Domain: domain.into(),
            SubDomain: subdomain,
            RecordId: record_id,
            RecordType: record_type,
            RecordLine: RecordLine::默认,
            Value: value.into(),
        }
    }
}

impl ExtractAction for ModifyRecord {
    fn to_action() -> Action {
        Action::ModifyRecord
    }
}
