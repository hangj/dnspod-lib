#![doc = include_str!("../README.md")]

use std::collections::HashMap;

use chrono::Utc;
use data_types::*;
use header_params::HeaderParams;
use signature::calculate_authorization;

pub mod action;
pub mod consts;
pub mod data_types;
pub mod error_code;
pub mod header_params;
pub mod response;
pub mod signature;
mod utils;

// re-export serde and serde_json
// #[allow(unused_imports)]
// #[macro_use]
pub use serde;
pub use serde_json;

pub mod prelude {
    pub use super::action::*;
    pub use super::ExtractCommonParams;
}

pub trait ExtractCommonParams {
    fn action(&self) -> &'static str;
    fn body(&self) -> Vec<u8>;
    fn url(&self) -> &'static str;
    fn version(&self) -> Version;
    fn region(&self) -> Option<Region>;

    fn headers(
        &self,
        secret_id: &impl AsRef<str>,
        secret_key: &impl AsRef<str>,
    ) -> HashMap<String, String> {
        let datetime = Utc::now();
        // let timestamp = datetime.timestamp() as u64;
        // let date = datetime.date_naive().to_string();

        let mut hp = HeaderParams {
            action: self.action(),
            version: self.version(),
            region: self.region(),
            datetime,
            authorization: "".into(),
            language: Language::EnUS,
            content_type: ContentType::JSON,
        };

        let body = self.body();
        hp.authorization =
            calculate_authorization(&body, &hp, secret_id.as_ref(), secret_key.as_ref());

        hp.into()
    }
}


/// This is a helper trait for the macros to overloading the default implementation
#[allow(non_camel_case_types)]
pub trait DefaultMetaParams {
    #[inline] fn get_url(&self) -> &'static str { consts::DNSPOD_URL }
    #[inline] fn get_region(&self) -> Option<Region> { None }
    #[inline] fn get_version(&self) -> Version { Default::default() }
}