use serde::Deserialize;
use serde::Serialize;

use crate::data_types::Action;
use crate::ExtractAction;

/// 删除记录
/// https://cloud.tencent.com/document/api/1427/56176
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRecord {
    /// 域名
    /// 示例值：dnspod.cn
    Domain: String,
    /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 [RecordId]
    /// 示例值：162
    RecordId: u64,
}

impl DeleteRecord {
    pub fn new(domain: impl Into<String>, record_id: u64) -> Self {
        Self {
            Domain: domain.into(),
            RecordId: record_id,
        }
    }
}

impl ExtractAction for DeleteRecord {
    fn to_action() -> Action {
        Action::DeleteRecord
    }
}
