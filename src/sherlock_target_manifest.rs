use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SherlockTargetManifest {
    #[serde(rename = "$schema")]
    pub schema: String,

    #[serde(flatten)]
    pub targets: HashMap<String, TargetInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub url: String,
    pub url_main: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_probe: Option<String>,
    pub username_claimed: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payload: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub __comment__: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<RequestMethod>,
    pub error_type: ErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<ErrorMsg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tags {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum RequestMethod {
    Get,
    Post,
    Head,
    Put,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    Message,
    ResponseUrl,
    StatusCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorMsg {
    Single(String),
    Multiple(Vec<String>),
}

impl ErrorMsg {
    pub fn is_in(&self, text: &str) -> bool {
        match self {
            ErrorMsg::Single(msg) => text.contains(msg),
            ErrorMsg::Multiple(msgs) => msgs.iter().any(|msg| text.contains(msg)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorCode {
    Single(u16),
    Multiple(Vec<u16>),
}

impl ErrorCode {
    pub fn contains(&self, code: &u16) -> bool {
        match self {
            ErrorCode::Single(c) => c == code,
            ErrorCode::Multiple(codes) => codes.iter().any(|c| c == code),
        }
    }
}
