use serde::Serialize;

use crate::data_types::*;
use crate::utils::none_to_empty_string;
use crate::ExtractAction;

/// 获取域名的解析记录列表
/// https://cloud.tencent.com/document/api/1427/56166
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct DescribeRecordList {
    /// 要获取的解析记录所属的域名
    ///示例值：shenjianing.com
    Domain: String,
    /// 解析记录的主机头，如果传了此参数，则只会返回此主机头对应的解析记录
    /// 示例值：www
    #[serde(serialize_with = "none_to_empty_string")]
    Subdomain: Option<String>,

    /// 通过关键字搜索解析记录，当前支持搜索主机头和记录值
    /// 示例值：book
    #[serde(skip_serializing_if = "Option::is_none")]
    Keyword: Option<String>,
}

impl DescribeRecordList {
    pub fn new(domain: impl Into<String>, subdomain: Option<String>, keyword: Option<String>) -> Self {
        Self {
            Domain: domain.into(),
            Subdomain: subdomain,
            Keyword: keyword,
        }
    }
}

impl ExtractAction for DescribeRecordList {
    fn to_action() -> Action {
        Action::DescribeRecordList
    }
}
