//! 定义各个 Action
//! 
//! 添加具体 Action 时可以通过 `@[url = consts::DNSPOD_URL]` 覆盖掉默认公共参数。可以覆盖的还有 region 和 version
//! 
//! Example:
//! 
//! ```rust
//! /// 获取域名列表
//! /// https://cloud.tencent.com/document/api/1427/56172
//! @[url = consts::DNSPOD_URL]
//! @[version = Version::Version2021_03_23]
//! pub struct DescribeDomainList {
//!     /// 域名分组类型，默认为ALL
//!     #[cfg_attr(feature = "clap", arg(long, value_enum, default_value_t=Default::default()))]
//!     pub Type: DomainType,
//!     /// 记录开始的偏移, 第一条记录为 0, 依次类推。默认值为0。
//!     /// 示例值：0
//!     #[cfg_attr(feature = "clap", arg(long, default_value_t=0))]
//!     pub Offset: Integer,
//!     /// 要获取的域名数量, 比如获取20个, 则为20。默认值为3000。
//!     /// 示例值：20
//!     #[cfg_attr(feature = "clap", arg(long, default_value_t=3000))]
//!     pub Limit: Integer,
//!     /// 分组ID, 第一个组为 0, 获取指定分组的域名
//!     /// 示例值：1
//!     #[cfg_attr(feature = "clap", arg(long, default_value_t=0))]
//!     pub GroupId: Integer,
//!     /// 根据关键字搜索域名
//!     /// 示例值：qq
//!     #[cfg_attr(feature = "clap", arg(long, default_value=""))]
//!     pub Keyword: Option<String>,
//! }
//! ```

#![allow(non_snake_case)]


use crate::utils::none_to_empty_string;

use crate::data_types::*;
use crate::consts;

/// for `#[serde(crate = "dnspod_lib::serde")]`
mod dnspod_lib {
    pub use crate::serde;
}

#[macro_export]
macro_rules! define_action_list {
    () => {};

    // 手动实现 Action 且不需要有 enum 的情况, 面向外部的接口
    (
        $(
            $(#[$meta: meta])*
            $(@[$param_meta: ident = $param_expr: expr])*
            $vis: vis struct $name: ident $tt: tt
        )*
    ) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, $crate::serde::Serialize, $crate::serde::Deserialize)]
            #[serde(crate = "dnspod_lib::serde")]
            $vis struct $name $tt

            const _: () = {
                use $crate::DefaultMetaParams;

                impl DefaultMetaParams for $name {
                    $(
                        define_action_list!($param_meta, $param_expr);
                    )*
                }

                impl $crate::ExtractCommonParams for $name {
                    #[inline] fn action(&self) -> &'static str { stringify!($name) }
                    #[inline] fn body(&self) -> Vec<u8> { $crate::serde_json::to_vec(self).unwrap() }
                    #[inline] fn url(&self) -> &'static str { self.get_url() }
                    #[inline] fn version(&self) -> $crate::data_types::Version { self.get_version() }
                    #[inline] fn region(&self) -> Option<$crate::data_types::Region> { self.get_region() }
                }
            };
        )*
    };

    // 手动实现 Action 带有 enum 的情况, 面向外部的接口
    (
        $action_enum: ident,
        $(
            $(#[$meta: meta])*
            $(@[$param_meta: ident = $param_expr: expr])*
            $vis: vis struct $name: ident $tt: tt
        )*
    ) => {
        define_action_list!(
            enum $action_enum {},
            $(
                $(#[$meta])*
                $(@[$param_meta = $param_expr])*
                $vis struct $name $tt
            )*
        );
    };

    // 真正干活的接口
    (
        $(#[$enum_meta: meta])*
        enum $action_enum: ident {},
        $(
            $(#[$meta: meta])*
            $(@[$param_meta: ident = $param_expr: expr])*
            $vis: vis struct $name: ident $tt: tt
        )*
    ) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, $crate::serde::Serialize, $crate::serde::Deserialize)]
            #[serde(crate = "dnspod_lib::serde")]
            $vis struct $name $tt

            impl From<$name> for $action_enum {
                fn from(v: $name) -> Self {
                    Self::$name(v)
                }
            }

            const _: () = {
                use $crate::DefaultMetaParams;

                impl DefaultMetaParams for $name {
                    $(
                        define_action_list!($param_meta, $param_expr);
                    )*
                }

                impl $crate::ExtractCommonParams for $name {
                    #[inline] fn action(&self) -> &'static str { stringify!($name) }
                    #[inline] fn body(&self) -> Vec<u8> { $crate::serde_json::to_vec(self).unwrap() }
                    #[inline] fn url(&self) -> &'static str { self.get_url() }
                    #[inline] fn version(&self) -> $crate::data_types::Version { self.get_version() }
                    #[inline] fn region(&self) -> Option<$crate::data_types::Region> { self.get_region() }
                }
            };
        )*

        #[derive(Debug, Clone)]
        $(#[$enum_meta])*
        pub enum $action_enum {
            $($name($name),)*
        }

        impl $crate::ExtractCommonParams for $action_enum {
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

    // 本 lib 内部使用的接口, 注入一些 meta 数据
    (
        crate,
        $action_enum: ident,
        $(
            $(#[$meta: meta])*
            $(@[$param_meta: ident = $param_expr: expr])*
            $vis: vis struct $name: ident $tt: tt
        )*
    ) => {
        define_action_list!(
            #[cfg_attr(feature = "clap", derive(clap::Subcommand))]
            enum $action_enum {},
            $(
                $(#[$meta])*
                #[cfg_attr(feature = "clap", derive(clap::Parser))]
                $(@[$param_meta = $param_expr])*
                $vis struct $name $tt
            )*
        );
    };

    (url, $expr: expr) => {
        #[inline] fn get_url(&self) -> &'static str { $expr }
    };
    (version, $expr: expr) => {
        #[inline] fn get_version(&self) -> $crate::data_types::Version { $expr }
    };
    (region, $expr: expr) => {
        #[inline] fn get_region(&self) -> Option<$crate::data_types::Region> { Some($expr) }
    };
    ($($tt: tt)*) => {
        compile_error!("This macro only accepts `url` `region` `version`");
    };
}


define_action_list! {
    crate,
    Action,

    /// 获取域名列表
    /// https://cloud.tencent.com/document/api/1427/56172
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
    /// https://cloud.tencent.com/document/api/1427/56180
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
    /// https://cloud.tencent.com/document/api/1427/56176
    pub struct DeleteRecord {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 [RecordId]
        /// 示例值：162
        #[cfg_attr(feature = "clap", arg(long))]
        pub RecordId: u64,
    }

    /// 获取域名的解析记录列表
    /// https://cloud.tencent.com/document/api/1427/56166
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
    /// https://cloud.tencent.com/document/api/1427/56168
    pub struct DescribeRecord {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 [RecordId]
        /// 示例值：162
        #[cfg_attr(feature = "clap", arg(long))]
        pub RecordId: u64,
    }

    /// 更新动态 DNS 记录
    /// https://cloud.tencent.com/document/api/1427/56158
    pub struct ModifyDynamicDNS {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 主机记录，如 www，如果不传，默认为 @。
        /// 示例值：www
        #[cfg_attr(feature = "clap", arg(long, default_value="@"))]
        pub SubDomain: String,
        /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 [RecordId]
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
    /// https://cloud.tencent.com/document/api/1427/56157
    pub struct ModifyRecord {
        /// 域名
        /// 示例值：dnspod.cn
        #[cfg_attr(feature = "clap", arg(long))]
        pub Domain: String,
        /// 主机记录，如 www，如果不传，默认为 @。
        /// 示例值：www
        #[cfg_attr(feature = "clap", arg(long, default_value="@"))]
        pub SubDomain: String,
        /// 记录 ID 。可以通过接口 [DescribeRecordList] 查到所有的解析记录列表以及对应的 [RecordId]
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
