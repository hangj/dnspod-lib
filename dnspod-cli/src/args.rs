use clap::Parser;
use dnspod_lib::action;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
    #[arg(long, env = "DNSPOD_SECRET_ID")]
    pub secret_id: String,
    #[arg(long, env = "DNSPOD_SECRET_KEY")]
    pub secret_key: String,
}


macro_rules! impl_enums {
    (
        $(
            $(#[$meta: meta])*
            $vis: vis enum $name: ident {
                $(
                    $(#[$field_meta: meta])*
                    $field: ident ($ty: ty),
                )*
            }
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis enum $name {
                $(
                    $(#[$field_meta])*
                    $field($ty),
                )*
            }

            impl dnspod_lib::ExtractCommonParams for $name {
                #[inline] fn action(&self) -> &'static str {
                    match self {
                        $( $name::$field(v) => v.action(), )*
                    }
                }
                #[inline] fn body(&self) -> Vec<u8> {
                    match self {
                        $( $name::$field(v) => v.body(), )*
                    }
                }
                #[inline] fn url(&self) -> &'static str {
                    match self {
                        $( $name::$field(v) => v.url(), )*
                    }
                }
                #[inline] fn version(&self) -> dnspod_lib::data_types::Version {
                    match self {
                        $( $name::$field(v) => v.version(), )*
                    }
                }
                #[inline] fn region(&self) -> Option<dnspod_lib::data_types::Region> {
                    match self {
                        $( $name::$field(v) => v.region(), )*
                    }
                }
            }
        )*
    };
}

dnspod_lib::custom_meta_struct! {
    (
        impl_enums,
        #[derive(Debug, Clone, clap::Subcommand)]
    ),

    pub enum Action {
        /// 记录相关
        #[clap(subcommand)]
        Record(Record),
        /// 域名相关
        #[clap(subcommand)]
        Domain(Domain),
    }

    pub enum Record {
        Describe(action::DescribeRecord),
        List(action::DescribeRecordList),
        Create(action::CreateRecord),
        Delete(action::DeleteRecord),
        Modify(action::ModifyRecord),
        ModifyDDNS(action::ModifyDynamicDNS),
        DescribeType(action::DescribeRecordType),
        DescribeLineList(action::DescribeRecordLineList),
    }

    pub enum Domain {
        List(action::DescribeDomainList),
        Create(action::CreateDomain),
        Delete(action::DeleteDomain),
    }
}



impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
