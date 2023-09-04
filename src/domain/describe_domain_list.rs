use serde::Deserialize;
use serde::Serialize;

use crate::data_types::*;
use crate::ExtractAction;

/// 获取域名列表
/// https://cloud.tencent.com/document/api/1427/56172
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DescribeDomainList {
    /// Type	否	String	域名分组类型，默认为ALL。可取值为ALL，MINE，SHARE，ISMARK，PAUSE，VIP，RECENT，SHARE_OUT，FREE。
    /// 示例值：ALL
    Type: DomainType,
    /// Offset	否	Integer	记录开始的偏移, 第一条记录为 0, 依次类推。默认值为0。
    /// 示例值：0
    Offset: Integer,
    /// Limit	否	Integer	要获取的域名数量, 比如获取20个, 则为20。默认值为3000。
    /// 示例值：20
    Limit: Integer,
    /// GroupId	否	Integer	分组ID, 第一个组为 0, 获取指定分组的域名
    /// 示例值：1
    GroupId: Integer,
    /// Keyword	否	String	根据关键字搜索域名
    /// 示例值：qq
    Keyword: String,
}

impl Default for DescribeDomainList {
    fn default() -> Self {
        Self {
            Type: DomainType::ALL,
            Offset: 0,
            Limit: 3000,
            GroupId: 0,
            Keyword: "".into(),
        }
    }
}

impl ExtractAction for DescribeDomainList {
    fn to_action() -> Action {
        Action::DescribeDomainList
    }
}
