#![doc = include_str!("../README.md")]

use std::collections::HashMap;

use action::ExtractCommonParams;
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

pub mod prelude {
    pub use super::action::*;
    pub use super::data_types::*;
    pub use super::response::*;
    pub use super::ExtractHeaders;
}

pub trait ExtractHeaders: ExtractCommonParams {
    fn headers(
        &self,
        secret_id: &impl AsRef<str>,
        secret_key: &impl AsRef<str>,
    ) -> HashMap<String, String>;
}

impl<T: ExtractCommonParams> ExtractHeaders for T {
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
