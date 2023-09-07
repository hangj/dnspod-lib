//! 参数类型
//! https://cloud.tencent.com/document/api/1427/78480

#![allow(unused)]

use std::str::FromStr;

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::Utc;
use literal_enum::LiteralEnum;
use serde::Deserialize;
use serde::Serialize;

pub type String = std::string::String;
pub type Date = std::string::String; // DnsPodDate
pub type Timestamp = std::string::String; // DnsPodTimestamp
pub type Integer = u64;
pub type Boolean = bool;
pub type Float = f32;
pub type Double = f32;
pub type Binary = Vec<u8>;

#[derive(Debug, Default, Clone, LiteralEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum ContentType {
    #[default]
    #[lit = "application/json; charset=utf-8"]
    #[serde(rename = "application/json; charset=utf-8")]
    #[cfg_attr(feature = "clap", clap(name = "application/json; charset=utf-8"))]
    JSON,
}

#[derive(Debug, Default, Clone, LiteralEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Language {
    #[default]
    #[lit = "en-US"]
    #[serde(rename = "en-US")]
    #[cfg_attr(feature = "clap", clap(name = "en-US"))]
    EnUS,
    #[lit = "zh-CN"]
    #[serde(rename = "zh-CN")]
    #[cfg_attr(feature = "clap", clap(name = "zh-CN"))]
    ZhCN,
}

#[derive(Debug, Default, Clone, LiteralEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Version {
    #[default]
    #[lit = "2021-03-23"]
    #[serde(rename = "2021-03-23")]
    #[cfg_attr(feature = "clap", clap(name = "2021-03-23"))]
    Version2021_03_23,
}

#[derive(Debug, Clone, LiteralEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Region {}

#[derive(Debug, Default, Clone, LiteralEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum RecordType {
    /// A 记录是最常用类型，将域名指向一个 IPv4 地址，如 8.8.8.8
    #[default]
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    A,
    /// 将域名指向另一个域名地址，与其保持相同解析，如 https://www.dnspod.cn
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    CNAME,
    /// 用于邮件服务器，相关参数一般由邮件注册商提供
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    MX,
    /// 可填写附加文本信息，常用于域名验证
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    TXT,
    /// 将域名指向一个 IPv6 地址，如 ff06:0:0:0:0:0:0:c3
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    AAAA,
    /// 域名服务器记录，可将指定域名交由其他 DNS 服务商解析管理
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    NS,
    /// 用于指定域名的证书颁发机构（CA），减少证书颁发风险
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    CAA,
    /// 用于标识某台服务器使用了某个服务，常见于微软系统的目录管理。格式为「服务名字.协议类型」，如 _sip._tcp
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    SRV,
    /// HTTPS 服务绑定记录，有助于提升 HTTPS 安全性及性能
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    HTTPS,
    /// 新型服务绑定记录类型，允许服务指向多个客户端，并关联自定义参数值
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    SVCB,
    /// 用于指定发送邮件的服务器，是一种高效的反垃圾邮件解决方案
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    SPF,
    /// 将一个域名重定向至某个具体网页，且显示实际 URL 。仅支持 301 重定向，该记录要求双方域名均已完成备案。
    #[cfg_attr(feature = "clap", clap(name = "显性URL"))]
    显性URL,
    /// 将一个域名重定向至某个具体网页，但隐藏实际 URL 。仅支持 301 重定向，该记录要求双方域名均已完成备案。
    #[cfg_attr(feature = "clap", clap(name = "隐性URL"))]
    隐性URL,
}

#[derive(Debug, Clone, Default, LiteralEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum RecordLine {
    #[default]
    #[cfg_attr(feature = "clap", clap(name = "默认"))]
    默认,
}

/// 域名分组类型 ALL，默认为ALL  
/// 可取值为: MINE，SHARE，ISMARK，PAUSE，VIP，RECENT，SHARE_OUT，FREE
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Default, LiteralEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum DomainType {
    #[default]
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    ALL,
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    MINE,
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    SHARE,
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    ISMARK,
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    PAUSE,
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    VIP,
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    RECENT,
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    SHARE_OUT,
    #[cfg_attr(feature = "clap", clap(rename_all = "UPPER"))]
    FREE,
}

/// 注意: 服务器有时会返回 "0000-00-00", 会导致 date 解析出错
/// 所以直接用 String 会有更好的兼容性
#[derive(Debug, Clone)]
pub struct DnsPodDate {
    pub date: Option<NaiveDate>,
}

/// https://serde.rs/custom-date-format.html
impl Serialize for DnsPodDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(ref date) = self.date {
            let s = date.to_string();
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_str("0000-00-00")
        }
    }
}

impl<'de> Deserialize<'de> for DnsPodDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match NaiveDate::from_str(&s) {
            Ok(date) => Ok(Self { date: Some(date) }),
            Err(_) => Ok(Self { date: None }),
        }
    }
}

#[test]
fn test() {
    let s = "\"0000-00-00 00:00:00\"";
    assert!(serde_json::from_str::<'_, Timestamp>(s).is_err());

    let s = "\"0000-01-01 00:00:00\"";
    let t: Timestamp = serde_json::from_str(s).unwrap();
    let s = serde_json::to_string_pretty(&t).unwrap();

    println!("t: {:?}", t);
    println!("s: {:?}", s);

    let s = "\"2023-09-03\"";
    let d: Date = serde_json::from_str(s).unwrap();
    let s = serde_json::to_string_pretty(&d).unwrap();
    println!("d: {:?}", d);
    println!("s: {:?}", s);
}
