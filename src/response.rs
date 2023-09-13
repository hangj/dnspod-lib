//! 返回结果 <https://cloud.tencent.com/document/api/1427/56191>
//! 参数类型 <https://cloud.tencent.com/document/api/1427/78480>
//!
//! 目前腾讯云 API 3.0 输入参数和输出参数支持如下几种数据格式：
//!
//! String: 字符串。
//! Integer：整型，上限为无符号64位整数。SDK 3.0 不同编程语言支持的类型有所差异，建议以所使用编程语言的最大整型定义，例如 Golang 的 uint64。
//! Boolean：布尔型。
//! Float：浮点型。
//! Double：双精度浮点型。
//! Date：字符串，日期格式。例如：2022-01-01。
//! Timestamp：字符串，时间格式。例如：2022-01-01 00:00:00。
//! Timestamp ISO8601：ISO 8601 是由国际标准化组织（International Organization for Standardization，ISO）发布的关于日期和时间格式的国际标准，对应国标 《GB/T 7408-2005数据元和交换格式信息交换日期和时间表示法》。建议以所使用编程语言的标准库进行格式解析。例如：2022-01-01T00:00:00+08:00。
//! Binary：二进制内容，需要以特定协议请求和解析。
#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::data_types::*;
use crate::error_code::ErrorCode;

crate::custom_meta_struct! {
    (
        // 公共 meta attribute, 赋给每个 struct 
        #[derive(Debug, Clone, crate::serde::Serialize, crate::serde::Deserialize)]
    ),

    /// 返回结果
    /// <https://cloud.tencent.com/document/api/1427/56191>
    pub struct Response {
        pub Response: InnerResponse,
    }

    pub struct InnerResponse {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub Error: Option<Error>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub TotalCount: Option<Integer>,
        /// 唯一请求 ID，每次请求都会返回。定位问题时需要提供该次请求的 RequestId
        pub RequestId: String,
        /// 记录ID
        /// 示例值：162
        #[serde(skip_serializing_if = "Option::is_none")]
        pub RecordId: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub RecordCountInfo: Option<RecordCountInfo>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub RecordList: Option<Vec<RecordListItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub RecordInfo: Option<RecordInfo>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub DomainCountInfo: Option<DomainCountInfo>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub DomainList: Option<Vec<DomainListItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub LineList: Option<Vec<LineInfo>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub LineGroupList: Option<Vec<LineGroupInfo>>,
        /// 域名信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub DomainInfo: Option<DomainCreateInfo>,
    }

    pub struct Error {
        #[serde(flatten)]
        pub code: ErrorCode,
        pub Message: String,
    }

    pub struct RecordCountInfo {
        pub SubdomainCount: Integer,
        pub TotalCount: Integer,
        pub ListCount: Integer,
    }

    pub struct RecordListItem {
        /// 记录Id
        /// 示例值：1
        pub RecordId: Integer,
        /// String    记录值
        /// 示例值：1.1.1.1
        pub Value: String,
        /// Status    String    记录状态，启用：ENABLE，暂停：DISABLE
        /// 示例值：ENABLE
        pub Status: String,
        /// UpdatedOn    Timestamp    更新时间
        /// 示例值：2021-03-28 11:27:09
        pub UpdatedOn: Timestamp,
        /// Name    String    主机名
        /// 示例值：www
        pub Name: String,
        /// Line    String    记录线路
        /// 示例值：默认
        pub Line: String,
        /// LineId    String    线路Id
        /// 示例值：0
        pub LineId: String,
        /// Type    String    记录类型
        /// 示例值：A
        pub Type: String,
        /// Weight    Integer    记录权重，用于负载均衡记录
        /// 注意：此字段可能返回 null，表示取不到有效值。
        /// 示例值：20
        pub Weight: Option<Integer>,
        /// MonitorStatus    String    记录监控状态，正常：OK，告警：WARN，宕机：DOWN，未设置监控或监控暂停则为空
        /// 示例值：OK
        pub MonitorStatus: String,
        /// Remark    String    记录备注说明
        /// 示例值：用于api
        pub Remark: String,
        /// TTL    Integer    记录缓存时间
        /// 示例值：600
        pub TTL: Integer,
        /// MX    Integer    MX值，只有MX记录有
        /// 注意：此字段可能返回 null，表示取不到有效值。
        /// 示例值：10
        pub MX: Option<Integer>,
        /// DefaultNS    Boolean    是否是默认的ns记录
        /// 示例值：true
        pub DefaultNS: Option<Boolean>,
    }

    pub struct RecordInfo {
        /// Id    Integer    记录 ID 。
        /// 示例值：158
        pub Id: Integer,

        /// SubDomain    String    子域名(主机记录)。
        /// 示例值：www
        pub SubDomain: String,

        /// RecordType    String    记录类型, 详见 [DescribeRecordType](super::action::DescribeRecordType) 接口。
        /// 示例值：A
        pub RecordType: String,

        /// RecordLine    String    解析记录的线路，详见 [DescribeRecordLineList](super::action::DescribeRecordLineList) 接口。
        /// 示例值：百度
        pub RecordLine: String,

        /// RecordLineId    String    解析记录的线路 ID ，详见 [DescribeRecordLineList](super::action::DescribeRecordLineList) 接口。
        /// 示例值：90=0
        pub RecordLineId: String,

        /// Value    String    记录值。
        /// 示例值：129.23.32.32
        pub Value: String,

        /// Weight    Integer    记录权重值。
        /// 注意：此字段可能返回 null，表示取不到有效值。
        /// 示例值：10
        pub Weight: Option<Integer>,

        /// MX    Integer    记录的 MX 记录值，非 MX 记录类型，默认为 0。
        /// 示例值：20
        pub MX: Integer,

        /// TTL    Integer    记录的 TTL 值。
        /// 示例值：600
        pub TTL: Integer,

        /// Enabled    Integer    记录状态。0表示禁用，1表示启用。
        /// 示例值：1
        pub Enabled: Integer,

        /// MonitorStatus    String    该记录的 D 监控状态。
        /// "Ok" : 服务器正常。
        /// "Warn" : 该记录有报警, 服务器返回 4XX。
        /// "Down" : 服务器宕机。
        /// "" : 该记录未开启 D 监控。
        /// 示例值：Ok
        pub MonitorStatus: String,

        /// Remark    String    记录的备注。
        /// 注意：此字段可能返回 null，表示取不到有效值。
        /// 示例值：这是解析记录的备注
        pub Remark: Option<String>,

        /// UpdatedOn    Timestamp    记录最后更新时间。
        /// 示例值：2021-03-31 11:38:02
        pub UpdatedOn: Timestamp,

        /// DomainId    Integer    域名 ID 。
        /// 示例值：62
        pub DomainId: Integer,
    }

    pub struct DomainCountInfo {
        /// DomainTotal    Integer    符合条件的域名数量
        /// 示例值：1
        pub DomainTotal: Integer,
        /// AllTotal    Integer    用户可以查看的所有域名数量
        /// 示例值：1
        pub AllTotal: Integer,
        /// MineTotal    Integer    用户账号添加的域名数量
        /// 示例值：1
        pub MineTotal: Integer,
        /// ShareTotal    Integer    共享给用户的域名数量
        /// 示例值：1
        pub ShareTotal: Integer,
        /// VipTotal    Integer    付费域名数量
        /// 示例值：1
        pub VipTotal: Integer,
        /// PauseTotal    Integer    暂停的域名数量
        /// 示例值：1
        pub PauseTotal: Integer,
        /// ErrorTotal    Integer    dns设置错误的域名数量
        /// 示例值：1
        pub ErrorTotal: Integer,
        /// LockTotal    Integer    锁定的域名数量
        /// 示例值：1
        pub LockTotal: Integer,
        /// SpamTotal    Integer    封禁的域名数量
        /// 示例值：1
        pub SpamTotal: Integer,
        /// VipExpire    Integer    30天内即将到期的域名数量
        /// 示例值：1
        pub VipExpire: Integer,
        /// ShareOutTotal    Integer    分享给其它人的域名数量
        /// 示例值：1
        pub ShareOutTotal: Integer,
        /// GroupTotal    Integer    指定分组内的域名数量
        /// 示例值：1
        pub GroupTotal: Integer,
    }

    pub struct DomainListItem {
        /// DomainId    Integer    系统分配给域名的唯一标识
        /// 示例值：12
        pub DomainId: Integer,
        /// Name    String    域名的原始格式
        /// 示例值：qq.com
        pub Name: String,
        /// Status    String    域名的状态，正常：ENABLE，暂停：PAUSE，封禁：SPAM
        /// 示例值：ENABLE
        pub Status: String,
        /// TTL    Integer    域名默认的解析记录默认TTL值
        /// 示例值：600
        pub TTL: Integer,
        /// CNAMESpeedup    String    是否开启CNAME加速，开启：ENABLE，未开启：DISABLE
        /// 示例值：DISABLE
        pub CNAMESpeedup: String,
        /// DNSStatus    String    DNS 设置状态，错误：DNSERROR，正常：空字符串
        /// 示例值：DNSERROR
        pub DNSStatus: String,
        /// Grade    String    域名的套餐等级代码
        /// 示例值：DP_FREE
        pub Grade: String,
        /// GroupId    Integer    域名所属的分组Id
        /// 示例值：1
        pub GroupId: Integer,
        /// SearchEnginePush    String    是否开启搜索引擎推送优化，是：YES，否：NO
        /// 示例值：NO
        pub SearchEnginePush: String,
        /// Remark    String    域名备注说明
        /// 示例值：重要域名
        pub Remark: String,
        /// Punycode    String    经过punycode编码后的域名格式
        /// 示例值：xn--a9.com
        pub Punycode: String,
        /// EffectiveDNS    Array of String    系统为域名分配的有效DNS
        /// 示例值：["f1g1ns1.dnspod.net","f1g1ns2.dnspod.net"]
        pub EffectiveDNS: Vec<String>,
        /// GradeLevel    Integer    域名套餐等级对应的序号
        /// 示例值：5
        pub GradeLevel: Integer,
        /// GradeTitle    String    套餐名称
        /// 示例值：免费版
        pub GradeTitle: String,
        /// IsVip    String    是否是付费套餐
        /// 示例值：YES
        pub IsVip: String,
        /// VipStartAt    Timestamp    付费套餐开通时间
        /// 示例值：2021-04-07 13:34:20
        pub VipStartAt: Timestamp,
        /// VipEndAt    Timestamp    付费套餐到期时间
        /// 示例值：2022-04-07 13:34:20
        pub VipEndAt: Timestamp,
        /// VipAutoRenew    String    域名是否开通VIP自动续费，是：YES，否：NO，默认：DEFAULT
        /// 示例值：YES
        pub VipAutoRenew: String,
        /// RecordCount    Integer    域名下的记录数量
        /// 示例值：20
        pub RecordCount: Integer,
        /// CreatedOn    Timestamp    域名添加时间
        /// 示例值：2020-05-21 16:08:29
        pub CreatedOn: Timestamp,
        /// UpdatedOn    Timestamp    域名更新时间
        /// 示例值：2021-04-01 18:09:58
        pub UpdatedOn: Timestamp,
        /// Owner    String    域名所属账号
        /// 示例值：abc@tencent.com
        pub Owner: String,
        /// TagList    Array of TagItem    域名关联的标签列表
        /// 注意：此字段可能返回 null，表示取不到有效值。
        pub TagList: Option<Vec<TagItem>>,
    }

    pub struct TagItem {
        /// 标签键
        /// 示例值：key1
        pub TagKey: String,
        /// 标签值
        /// 注意：此字段可能返回 null，表示取不到有效值。
        /// 示例值：value1
        pub TagValue: Option<String>,
    }

    /// 解析线路信息
    ///被如下接口引用：[DescribeRecordLineList](super::action::DescribeRecordLineList)
    pub struct LineInfo {
        /// 线路名称 示例值：电信
        Name: String,
        /// 线路ID 示例值：10=0
        LineId: String,
    }

    /// 线路分组信息
    /// 被如下接口引用：[DescribeRecordLineList](super::action::DescribeRecordLineList)
    pub struct LineGroupInfo {
        /// 线路分组ID 示例值：15=0
        LineId: String,	
        /// 线路分组名称 示例值：华北
        Name: String,
        /// 分组类型 示例值：system
        Type: String,	
        /// 线路分组包含的线路列表
        LineList: Vec<String>,
    }

    /// 域名信息（创建域名时返回） 被如下接口引用：[CreateDomain](super::action::CreateDomain)
    pub struct DomainCreateInfo {
        /// 域名ID
        Id: Integer,
        /// 域名
        Domain: String,
        /// 域名的punycode 示例值：dnspod.cn
        Punycode: String,
        /// 域名的NS列表 示例值：["source.dnspod.net","low.dnspod.net"]
        GradeNsList: Vec<String>,
    }
}

