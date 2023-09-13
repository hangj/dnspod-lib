//! 定义各个 Action 请求
//! 
//! 添加具体 Action 时可以通过 `@[url = consts::DNSPOD_URL]` 覆盖掉默认公共参数。可以覆盖的还有 region 和 version
//! 

#![allow(non_snake_case)]


use crate::utils::none_to_empty_string;

use crate::data_types::*;
use crate::consts;

/// for `#[serde(crate = "dnspod_lib::serde")]`
mod dnspod_lib {
    pub use crate::data_types;
    pub use crate::serde;
    pub use crate::consts;
}

#[macro_export]
macro_rules! overloading_common_params {
    (url = $expr: expr) => {
        #[inline] fn url(&self) -> &'static str { $expr }
    };
    (version = $expr: expr) => {
        #[inline] fn version(&self) -> $crate::data_types::Version { $expr }
    };
    (region = $expr: expr) => {
        #[inline] fn region(&self) -> Option<$crate::data_types::Region> { Some($expr) }
    };
    ($($tt: tt)*) => {
        compile_error!("This macro only accepts `url` `region` `version`");
    };
}

#[macro_export]
macro_rules! impl_define_action_list {
    (
        $(
            $(#[$meta: meta])*
            $(@[$($my_meta: tt)*])*
            $vis: vis struct $name: ident $body: tt
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis struct $name $body

            const _: () = {
                use $crate::ExtractCommonParams;
                use $crate::serde_json;

                impl ExtractCommonParams for $name {
                    #[inline] fn action(&self) -> &'static str { stringify!($name) }
                    #[inline] fn body(&self) -> Vec<u8> { serde_json::to_vec(self).unwrap() }
                    $(
                        $crate::overloading_common_params! { $($my_meta)* }
                    )*
                }
            };
        )*
    };

    (
        $(#[$enum_meta: meta])*
        $enum_vis: vis enum $enum_name: ident {}
        ,
        $(
            $(#[$meta: meta])*
            $(@[$($my_meta: tt)*])*
            $vis: vis struct $name: ident $body: tt
        )*
    ) => {
        $crate::impl_define_action_list! {
            $(
                $(#[$meta])*
                $(@[$($my_meta)*])*
                $vis struct $name $body
            )*
        }

        $(
            impl From<$name> for $enum_name {
                #[inline] fn from(v: $name) -> Self { Self::$name(v) }
            }
        )*

        $(#[$enum_meta])*
        $enum_vis enum $enum_name {
            $($name($name),)*
        }

        impl $crate::ExtractCommonParams for $enum_name {
            #[inline]
            fn action(&self) -> &'static str {
                #[allow(unreachable_patterns)]
                match self {
                    $(Self::$name(v) => v.action(),)*
                    _ => Default::default(),
                }
            }
            #[inline]
            fn body(&self) -> Vec<u8> {
                #[allow(unreachable_patterns)]
                match self {
                    $(Self::$name(v) => v.body(), )*
                    _ => Default::default(),
                }
            }
            #[inline]
            fn url(&self) -> &'static str {
                #[allow(unreachable_patterns)]
                match self {
                    $(Self::$name(v) => v.url(), )*
                    _ => Default::default(),
                }
            }
            #[inline]
            fn version(&self) -> $crate::data_types::Version {
                #[allow(unreachable_patterns)]
                match self {
                    $(Self::$name(v) => v.version(), )*
                    _ => Default::default(),
                }
            }
            #[inline]
            fn region(&self) -> Option<$crate::data_types::Region> {
                #[allow(unreachable_patterns)]
                match self {
                    $(Self::$name(v) => v.region(), )*
                    _ => Default::default(),
                }
            }
        }
    };
}

/// 供外部 crate 调用
#[macro_export]
macro_rules! define_action_list {
    (
        $($tt: tt)*
    ) => {
        $crate::custom_meta_struct! {
            (
                // callback macro
                $crate::impl_define_action_list,
                // common metas
                #[derive(Debug, Clone, $crate::serde::Serialize, $crate::serde::Deserialize)]
                #[serde(crate = "dnspod_lib::serde")]
            ),
            $($tt)*
        }
    };
}

macro_rules! define_action_list_private {
    (
        $($tt: tt)*
    ) => {
        $crate::impl_define_action_list! {
            #[derive(Debug, Clone)]
            #[cfg_attr(feature = "clap", derive(clap::Subcommand))]
            #[allow(non_snake_case)]
            pub enum Action {}
            ,
            $($tt)*
        }
    };
}


crate::custom_meta_struct! {
    (
        define_action_list_private, // callback macro

        // 公共 meta attribute, 赋给每个 struct 
        #[cfg_attr(feature = "clap", derive(clap::Parser))]
        #[derive(Debug, Clone, crate::serde::Serialize, crate::serde::Deserialize)]
        #[serde(crate = "dnspod_lib::serde")]
    ),

    /// 获取域名列表
    /// <https://cloud.tencent.com/document/api/1427/56172>
    @[url = consts::DNSPOD_URL]
    @[version = Version::Version2021_03_23]
    pub struct DescribeDomainList {
        /// 域名分组类型，默认为ALL
        #[cfg_attr(feature = "clap", arg(long, value_enum, default_value_t=Default::default()))]
        pub Type: DomainType,
        /// 记录开始的偏移, 第一条记录为 0, 依次类推。默认值为0。
        /// 示例值：0
        #[cfg_attr(feature = "clap", arg(long, default_value_t=0))]
        pub Offset: Integer,
        /// 要获取的域名数量, 比如获取20个, 则为20。默认值为3000。
        /// 示例值：20
        #[cfg_attr(feature = "clap", arg(long, default_value_t=3000))]
        pub Limit: Integer,
        /// 分组ID, 第一个组为 0, 获取指定分组的域名
        /// 示例值：1
        #[cfg_attr(feature = "clap", arg(long, default_value_t=0))]
        pub GroupId: Integer,
        /// 根据关键字搜索域名
        /// 示例值：qq
        #[cfg_attr(feature = "clap", arg(long, default_value=""))]
        pub Keyword: Option<String>,
    }

    /// 添加记录
    /// <https://cloud.tencent.com/document/api/1427/56180>
    pub struct CreateRecord {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 主机记录，如 www，如果不传，默认为 @。
        /// 示例值：www
        #[cfg_attr(feature = "clap", arg(long, default_value = "@"))]
        pub SubDomain: String,
        /// 记录类型，通过 API 记录类型获得，大写英文，比如：A 。
        /// 示例值：A
        #[cfg_attr(feature = "clap", arg(long, value_enum, rename_all = "UPPER"))]
        pub RecordType: RecordType,
        /// 记录线路，通过 API 记录线路获得，中文，比如：默认。
        /// 示例值：默认
        #[cfg_attr(feature = "clap", arg(long, value_enum, default_value_t=Default::default()))]
        pub RecordLine: RecordLine,
        /// 记录值，如 IP : 200.200.200.200， CNAME : cname.dnspod.com.， MX : mail.dnspod.com.。
        /// 示例值：200.200.200.200
        #[cfg_attr(feature = "clap", arg(long))]
        pub Value: String,
    }

    /// 删除记录
    /// <https://cloud.tencent.com/document/api/1427/56176>
    pub struct DeleteRecord {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 RecordId
        /// 示例值：162
        #[cfg_attr(feature = "clap", arg(long))]
        pub RecordId: u64,
    }

    /// 获取域名的解析记录列表
    /// <https://cloud.tencent.com/document/api/1427/56166>
    pub struct DescribeRecordList {
        /// 要获取的解析记录所属的域名
        /// 示例值：example.com
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 解析记录的主机头，如果传了此参数，则只会返回此主机头对应的解析记录
        /// 示例值：www
        #[serde(serialize_with = "none_to_empty_string")]
        #[cfg_attr(feature = "clap", arg(long, default_value=""))]
        pub Subdomain: Option<String>,
        /// 通过关键字搜索解析记录，当前支持搜索主机头和记录值
        /// 示例值：book
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "clap", arg(long, default_value=""))]
        pub Keyword: Option<String>,
    }

    /// 获取记录信息
    /// <https://cloud.tencent.com/document/api/1427/56168>
    pub struct DescribeRecord {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 RecordId
        /// 示例值：162
        #[cfg_attr(feature = "clap", arg(long))]
        pub RecordId: u64,
    }

    /// 获取等级允许的记录类型 <https://cloud.tencent.com/document/api/1427/56165>
    pub struct DescribeRecordType {
        /// 域名等级。
        /// 
        /// + 旧套餐：D_FREE、D_PLUS、D_EXTRA、D_EXPERT、D_ULTRA 分别对应免费套餐、个人豪华、企业1、企业2、企业3。
        /// 
        /// + 新套餐：DP_FREE、DP_PLUS、DP_EXTRA、DP_EXPERT、DP_ULTRA 分别对应新免费、个人专业版、企业创业版、企业标准版、企业旗舰版。
        /// 
        /// 示例值：DP_Plus
        #[cfg_attr(feature = "clap", arg(long))]
        pub DomainGrade: DomainGrade,
    }
    /// 获取等级允许的线路 <https://cloud.tencent.com/document/api/1427/56167>
    pub struct DescribeRecordLineList {
        /// 域名。
        /// 示例值：dnspod.cn
        pub Domain: String,
        #[cfg_attr(feature = "clap", arg(long))]
        /// 域名等级
        pub DomainGrade: DomainGrade,
    }

    /// 更新动态 DNS 记录
    /// <https://cloud.tencent.com/document/api/1427/56158>
    pub struct ModifyDynamicDNS {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 主机记录，如 www，如果不传，默认为 @。
        /// 示例值：www
        #[cfg_attr(feature = "clap", arg(long, default_value="@"))]
        pub SubDomain: String,
        /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 RecordId
        /// 示例值：162
        #[cfg_attr(feature = "clap", arg(long))]
        pub RecordId: u64,
        /// 记录线路，通过 API 记录线路获得，中文，比如：默认。
        /// 示例值：默认
        #[cfg_attr(feature = "clap", arg(value_enum, default_value_t=Default::default()))]
        pub RecordLine: RecordLine,
        /// 记录值，如 IP : 200.200.200.200， CNAME : cname.dnspod.com.， MX : mail.dnspod.com.。
        /// 示例值：200.200.200.200
        #[cfg_attr(feature = "clap", arg(long))]
        pub Value: String,
        /// TTL值，如果不传，默认为域名的TTL值。
        /// 示例值：600
        #[cfg_attr(feature = "clap", arg(long, default_value_t=600))]
        pub Ttl: Integer,
    }

    /// 修改记录
    /// <https://cloud.tencent.com/document/api/1427/56157>
    pub struct ModifyRecord {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 主机记录，如 www，如果不传，默认为 @。
        /// 示例值：www
        #[cfg_attr(feature = "clap", arg(long, default_value="@"))]
        pub SubDomain: String,
        /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 RecordId
        /// 示例值：162
        #[cfg_attr(feature = "clap", arg(long))]
        pub RecordId: u64,
        /// 记录类型，通过 API 记录类型获得，大写英文，比如：A 。
        /// 示例值：A
        #[cfg_attr(feature = "clap", arg(long))]
        pub RecordType: RecordType,
        /// 记录线路，通过 API 记录线路获得，中文，比如：默认。
        /// 示例值：默认
        #[cfg_attr(feature = "clap", arg(long, value_enum, default_value_t=Default::default()))]
        pub RecordLine: RecordLine,
        /// 记录值，如 IP : 200.200.200.200， CNAME : cname.dnspod.com.， MX : mail.dnspod.com.。
        /// 示例值：200.200.200.200
        #[cfg_attr(feature = "clap", arg(long))]
        pub Value: String,
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    super::super::define_action_list! {}

    crate::define_action_list! {
        /// fuckme 
        @[url = "https://hangj.cnblogs.com"]
        struct A;
        /// hey
        struct B;
    }
    
    crate::define_action_list! {
        /// 获取域名信息
        /// <https://cloud.tencent.com/document/api/1427/56173>
        @[url = "https://example.com"] // 公共参数可以重载 url, version, region
        pub struct DescribeDomain {
            /// 域名分组类型，默认为ALL
            #[serde(rename = "Domain")]
            pub domain: String,
        }
    
        @[version = dnspod_lib::data_types::Version::Version2021_03_23]
        #[allow(non_snake_case)]
        pub struct CustomAction {
            /// 域名分组类型，默认为ALL
            pub Domain: String,
        }
    }    
}