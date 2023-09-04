//! https://cloud.tencent.com/document/api/1427/56188

use std::collections::HashMap;

use crate::data_types::*;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;

use crate::utils::datetime_to_timestamp_string;

/// Headers
#[derive(Debug, Serialize)]
pub struct HeaderParams {
    /// X-TC-Action
    #[serde(rename = "X-TC-Action")]
    pub action: Action,
    /// X-TC-Version
    #[serde(rename = "X-TC-Version")]
    pub version: Version,
    /// X-TC-Region
    #[serde(rename = "X-TC-Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(
        rename = "X-TC-Timestamp",
        serialize_with = "datetime_to_timestamp_string"
    )]
    pub datetime: DateTime<Utc>,
    /// Authorization
    #[serde(rename = "Authorization")]
    pub authorization: String,
    /// X-TC-Language
    /// en-US/zh-CN
    #[serde(rename = "Language")]
    pub language: Language,
    #[serde(rename = "Content-Type")]
    pub content_type: ContentType,
}

impl From<HeaderParams> for HashMap<String, String> {
    fn from(value: HeaderParams) -> Self {
        let value = serde_json::to_value(&value)
            .expect("error: HeaderParams to HashMap failed (serde_json::to_value)");
        serde_json::from_value(value)
            .expect("error: HeaderParams to HashMap failed (serde_json::from_value)")
    }
}

#[test]
fn test() {
    let cp = HeaderParams {
        action: Action::CreateRecord,
        version: Version::Version2021_03_23,
        region: None,
        datetime: Utc::now(),
        authorization: "authorization".into(),
        language: Language::EnUS,
        content_type: ContentType::JSON,
    };

    let value = serde_json::to_value(&cp).unwrap();
    println!("value: {:?}", value);
    let m: HashMap<String, String> = serde_json::from_value(value).unwrap();

    println!("m: {:#?}", m);
}
