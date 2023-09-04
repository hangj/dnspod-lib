use serde::Deserialize;
use serde::Serialize;

use crate::data_types::Action;
use crate::ExtractAction;

/// 获取记录信息
/// https://cloud.tencent.com/document/api/1427/56168
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DescribeRecord {
    /// 域名
    /// 示例值：dnspod.cn
    Domain: String,
    /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 [RecordId]
    /// 示例值：162
    RecordId: u64,
}

impl DescribeRecord {
    pub fn new(domain: impl Into<String>, record_id: u64) -> Self {
        Self {
            Domain: domain.into(),
            RecordId: record_id,
        }
    }
}

impl ExtractAction for DescribeRecord {
    fn to_action() -> Action {
        Action::DescribeRecord
    }
}
