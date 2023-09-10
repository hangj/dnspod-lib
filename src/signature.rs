//! 签名方法 v3
//! https://cloud.tencent.com/document/api/1427/56189
//!
//! CanonicalRequest =
//! HTTPRequestMethod + '\n' +
//! CanonicalURI + '\n' +
//! CanonicalQueryString + '\n' +
//! CanonicalHeaders + '\n' +
//! SignedHeaders + '\n' +
//! HashedRequestPayload
//!
//! HTTPRequestMethod        HTTP 请求方法（GET、POST ）。此示例取值为 POST。
//! CanonicalURI            URI 参数，API 3.0 固定为正斜杠（/）。
//! CanonicalQueryString    发起 HTTP 请求 URL 中的查询字符串，对于 POST 请求，固定为空字符串""，对于 GET 请求，则为 URL 中问号（?）后面的字符串内容，例如：Limit=10&Offset=0。
//!                         注意：CanonicalQueryString 需要参考 RFC3986 进行 URLEncode，字符集 UTF-8，推荐使用编程语言标准库，所有特殊字符均需编码，大写形式。
//!
//! ```txt
//! POST
//! /
//!
//! content-type:application/json; charset=utf-8
//! host:cvm.tencentcloudapi.com
//! x-tc-action:describeinstances
//!
//! content-type;host;x-tc-action
//! 35e9c5b0e3ae67532d3c9f17ead6c90222632e5b1ff7f6e89887f1398934f064
//! ```

use hmac_sha256::{Hash, HMAC};

use crate::consts::*;
use crate::{data_types::ContentType, header_params::HeaderParams, utils::encode_hex};

/// 1. 拼接规范请求串
/// ```javascript
/// CanonicalRequest =
///     HTTPRequestMethod + '\n' +
///     CanonicalURI + '\n' +
///     CanonicalQueryString + '\n' +
///     CanonicalHeaders + '\n' +
///     SignedHeaders + '\n' +
///     HashedRequestPayload
/// ```
fn canonical_request(body: &[u8], common_params: &HeaderParams, signed_headers: &str) -> String {
    let action: &'static str = common_params.action;
    let action = action.to_ascii_lowercase();
    let host = DNSPOD_DOMAIN_NAME;

    // HTTP 请求方法（GET、POST ）。此示例取值为 POST
    let http_request_method = "POST";

    // URI 参数，API 3.0 固定为正斜杠（/）
    let canonical_uri = "/";

    // 发起 HTTP 请求 URL 中的查询字符串，对于 POST 请求，固定为空字符串""，对于 GET 请求，则为 URL 中问号（?）后面的字符串内容，例如：Limit=10&Offset=0。
    // 注意：CanonicalQueryString 需要参考 RFC3986 进行 URLEncode，字符集 UTF-8，推荐使用编程语言标准库，所有特殊字符均需编码，大写形式。
    let canonical_query_string = "";

    let content_type: &'static str = ContentType::JSON.into();
    let content_type = content_type.to_ascii_lowercase();
    // 参与签名的头部信息，至少包含 host 和 content-type 两个头部，也可加入其他头部参与签名以提高自身请求的唯一性和安全性，此示例额外增加了接口名头部。
    // 拼接规则：
    // 头部 key 和 value 统一转成小写，并去掉首尾空格，按照 key:value\n 格式拼接；
    // 多个头部，按照头部 key（小写）的 ASCII 升序进行拼接。
    // 此示例计算结果是 content-type:application/json; charset=utf-8\nhost:cvm.tencentcloudapi.com\nx-tc-action:describeinstances\n。
    // 注意：content-type 必须和实际发送的相符合，有些编程语言网络库即使未指定也会自动添加 charset 值，如果签名时和发送时不一致，服务器会返回签名校验失败
    let canonical_headers =
        format!("content-type:{content_type}\nhost:{host}\nx-tc-action:{action}\n");

    // 参与签名的头部信息，说明此次请求有哪些头部参与了签名，和 CanonicalHeaders 包含的头部内容是一一对应的。content-type 和 host 为必选头部。
    // 拼接规则：
    // 头部 key 统一转成小写；
    // 多个头部 key（小写）按照 ASCII 升序进行拼接，并且以分号（;）分隔。
    // 此示例为 content-type;host;x-tc-action
    let signed_headers = signed_headers;

    // 请求正文（payload，即 body，此示例为 {"Limit": 1, "Filters": [{"Values": ["\u672a\u547d\u540d"], "Name": "instance-name"}]}）的哈希值，计算伪代码为 Lowercase(HexEncode(Hash.SHA256(RequestPayload)))，即对 HTTP 请求正文做 SHA256 哈希，然后十六进制编码，最后编码串转换成小写字母。对于 GET 请求，RequestPayload 固定为空字符串。此示例计算结果是 35e9c5b0e3ae67532d3c9f17ead6c90222632e5b1ff7f6e89887f1398934f064。
    let hashed_request_payload = encode_hex(&Hash::hash(body));

    format!(
        r#"{http_request_method}
{canonical_uri}
{canonical_query_string}
{canonical_headers}
{signed_headers}
{hashed_request_payload}"#
    )
}

/// 2. 拼接待签名字符串
/// ```javascript
/// StringToSign =
///     Algorithm + \n +
///     RequestTimestamp + \n +
///     CredentialScope + \n +
///     HashedCanonicalRequest
/// ```
fn string_to_sign(
    body: &[u8],
    common_params: &HeaderParams,
    credential_scope: &str,
    signed_headers: &str,
) -> String {
    // 签名算法，目前固定为 TC3-HMAC-SHA256。
    let algorithm = ALGORITHM;

    // 请求时间戳，即请求头部的公共参数 X-TC-Timestamp 取值，取当前时间 UNIX 时间戳，精确到秒。此示例取值为 1551113065
    let timestamp = common_params.datetime.timestamp();

    // 凭证范围，格式为 Date/service/tc3_request，包含日期、所请求的服务和终止字符串（tc3_request）。Date 为 UTC 标准时间的日期，取值需要和公共参数 X-TC-Timestamp 换算的 UTC 标准时间日期一致；service 为产品名，必须与调用的产品域名一致。此示例计算结果是 2019-02-25/cvm/tc3_request
    let credential_scope = credential_scope;

    let canonical_request = canonical_request(body, common_params, signed_headers);
    // 前述步骤拼接所得规范请求串的哈希值，计算伪代码为 Lowercase(HexEncode(Hash.SHA256(CanonicalRequest)))。此示例计算结果是 7019a55be8395899b900fb5564e4200d984910f34794a27cb3fb7d10ff6a1e84
    let hashed_canonical_request = encode_hex(&Hash::hash(canonical_request.as_bytes()));

    format!(
        r#"{algorithm}
{timestamp}
{credential_scope}
{hashed_canonical_request}"#
    )
}

/// 3. 计算签名
/// 1）计算派生签名密钥，伪代码如下：
///
/// ```javascript
/// SecretKey = "Gu5t9xGARNpq86cd98joQYCN3*******"
/// SecretDate = HMAC_SHA256("TC3" + SecretKey, Date) // Date 即 Credential 中的 Date 字段信息。此示例取值为 2019-02-25。
/// SecretService = HMAC_SHA256(SecretDate, Service) // Service 即 Credential 中的 Service 字段信息。此示例取值为 dnspod
/// SecretSigning = HMAC_SHA256(SecretService, "tc3_request")
/// ```
///
/// 2）计算签名，伪代码如下：
///
/// ```javascript
/// Signature = HexEncode(HMAC_SHA256(SecretSigning, StringToSign))
/// ```
///
fn calc_signature(
    body: &[u8],
    common_params: &HeaderParams,
    secret_key: &str,
    credential_scope: &str,
    signed_headers: &str,
) -> String {
    let date = common_params.datetime.date_naive().to_string();
    let secret_date = HMAC::mac(date, format!("TC3{secret_key}"));
    let secret_service = HMAC::mac(SERVICE, secret_date);
    let secret_signing = HMAC::mac(TERMINATOR, secret_service);

    let s = string_to_sign(body, common_params, credential_scope, signed_headers);
    encode_hex(&HMAC::mac(s, secret_signing))
}

/// 4. 拼接 Authorization
/// 按如下格式拼接 Authorization：
///
/// ```javascript
/// Authorization =
///     Algorithm + ' ' +
///     'Credential=' + SecretId + '/' + CredentialScope + ', ' +
///     'SignedHeaders=' + SignedHeaders + ', ' +
///     'Signature=' + Signature
/// ```
///
/// Algorithm    签名方法，固定为 TC3-HMAC-SHA256。
/// SecretId    密钥对中的 SecretId，即 AKIDz8krbsJ5yKBZQpn74WFkmLPx3*******。
/// CredentialScope    见上文，凭证范围。此示例计算结果是 2019-02-25/cvm/tc3_request。
/// SignedHeaders    见上文，参与签名的头部信息。此示例取值为 content-type;host;x-tc-action。
/// Signature    签名值。此示例计算结果是 be4f67d323c78ab9acb7395e43c0dbcf822a9cfac32fea2449a7bc7726b770a3。
pub fn calculate_authorization(
    body: &[u8],
    common_params: &HeaderParams,
    secret_id: &str,
    secret_key: &str,
) -> String {
    let date = common_params.datetime.date_naive().to_string();

    let algorithm = ALGORITHM;
    let credential_scope = format!("{date}/{SERVICE}/{TERMINATOR}");
    let signed_headers = "content-type;host;x-tc-action";
    let signature = calc_signature(
        body,
        common_params,
        secret_key,
        credential_scope.as_str(),
        signed_headers,
    );

    format!("{algorithm} Credential={secret_id}/{credential_scope}, SignedHeaders={signed_headers}, Signature={signature}")
}
