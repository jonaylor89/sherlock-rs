use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SherlockTargetManifest {
    #[serde(rename = "$schema")]
    pub schema: String,

    #[serde(flatten)]
    pub targets: HashMap<String, TargetInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TargetInfo {
    pub url: String,
    #[serde(rename = "urlMain")]
    pub url_main: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlProbe", default)]
    pub url_probe: Option<String>,
    pub username_claimed: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "regexCheck",
        default
    )]
    pub regex_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isNSFW", default)]
    pub is_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub request_payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub __comment__: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Tags>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub request_method: Option<RequestMethod>,

    #[serde(flatten)]
    pub error_type: ErrorType,
    // The json schema says there is a `response_url` field, but it is not present
    // in any of the targets in the official repository
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "errorType", rename_all = "snake_case")]
pub enum ErrorType {
    Message {
        #[serde(rename = "errorMsg")]
        msg: ErrorMsg,
    },
    ResponseUrl {
        #[serde(rename = "errorUrl")]
        url: String,
    },
    StatusCode {
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "Option::is_none", default)]
        codes: Option<ErrorCode>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ErrorMsg {
    Single(String),
    Multiple(Vec<String>),
}

impl ErrorMsg {
    #[must_use]
    pub fn is_in(&self, text: &str) -> bool {
        match self {
            ErrorMsg::Single(msg) => text.contains(msg),
            ErrorMsg::Multiple(msgs) => msgs.iter().any(|msg| text.contains(msg)),
        }
    }
}

impl fmt::Debug for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorMsg::Single(c) => write!(f, "{c}"),
            ErrorMsg::Multiple(codes) => codes.iter().fold(Ok(()), |_, c| write!(f, "{c}, ")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ErrorCode {
    Single(u16),
    Multiple(Vec<u16>),
}

impl ErrorCode {
    #[must_use]
    pub fn contains(&self, code: &u16) -> bool {
        match self {
            ErrorCode::Single(c) => c == code,
            ErrorCode::Multiple(codes) => codes.iter().any(|c| c == code),
        }
    }
}
