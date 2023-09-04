#![doc = include_str!("../README.md")]

use std::collections::HashMap;

use chrono::Utc;
use data_types::*;
use header_params::HeaderParams;
use serde::Serialize;
use signature::calculate_authorization;

pub mod consts;
pub mod data_types;
pub mod domain;
pub mod error_code;
pub mod header_params;
pub mod record;
pub mod response;
pub mod signature;
mod utils;

pub mod prelude {
    pub use super::ExtractHeaders;
    pub use super::ExtractAction;
    pub use super::ExtractBody;
    pub use super::response::*;
    pub use super::data_types::*;
    pub use super::domain::*;
    pub use super::record::*;
}

pub trait ExtractAction {
    #[inline]
    fn url_() -> &'static str {
        consts::DNSPOD_URL
    }

    fn url(&self) -> &'static str {
        Self::url_()
    }

    fn to_action() -> Action;

    #[inline]
    fn version() -> Version {
        Version::Version2021_03_23
    }

    #[inline]
    fn region() -> Option<String> {
        None
    }
}

pub(crate) trait ToHeaderParams {
    fn to_header_params(body: &[u8], secret_id: &str, secret_key: &str) -> HeaderParams;
}

impl<T: ExtractAction> ToHeaderParams for T {
    fn to_header_params(body: &[u8], secret_id: &str, secret_key: &str) -> HeaderParams {
        let datetime = Utc::now();
        // let timestamp = datetime.timestamp() as u64;
        // let date = datetime.date_naive().to_string();

        let mut hp = HeaderParams {
            action: Self::to_action(),
            version: Self::version(),
            region: Self::region(),
            datetime,
            authorization: "".into(),
            language: Language::EnUS,
            content_type: ContentType::JSON,
        };

        hp.authorization = calculate_authorization(&body, &hp, secret_id, secret_key);

        hp
    }
}

pub trait ExtractBody: ExtractAction {
    fn body(&self) -> Vec<u8>;
}

impl<T: ExtractAction + Serialize> ExtractBody for T {
    fn body(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait ExtractHeaders: ExtractBody {
    fn headers(
        &self,
        secret_id: &impl AsRef<str>,
        secret_key: &impl AsRef<str>,
    ) -> HashMap<String, String>;
}

impl<T: ExtractAction + ExtractBody + ToHeaderParams> ExtractHeaders for T {
    fn headers(
        &self,
        secret_id: &impl AsRef<str>,
        secret_key: &impl AsRef<str>,
    ) -> HashMap<String, String> {
        let body = self.body();
        let body = body.as_slice();

        Self::to_header_params(body, secret_id.as_ref(), secret_key.as_ref()).into()
    }
}
