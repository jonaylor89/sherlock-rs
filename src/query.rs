use fancy_regex::Regex;
use std::sync::Arc;
use std::time::Duration;
use std::{fmt, time::Instant};
use thiserror::Error;
use tokio::sync::mpsc::Sender;

use crate::requests::{make_request, RequestResult};
use crate::sherlock_target_manifest::{ErrorType, RequestMethod, TargetInfo};
use crate::utils::Interpolatable;

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Invalid username")]
    InvalidUsernameError,
    #[error("Request error")]
    RequestError,
}

#[derive(Debug, PartialEq, Eq)]
pub enum QueryStatus {
    /// username detected
    Claimed,
    /// username not detected
    Available,
    /// error occured while trying to detect username
    Unknown,
    /// username not allowed for this site
    Illegal,
    /// request blocked by waf (i.e. Cloudflare)
    Waf,
}

#[derive(Debug)]
pub struct QueryResult {
    pub username: Arc<String>,
    pub site_name: Arc<String>,
    pub info: Arc<TargetInfo>,
    pub site_url_user: String,
    pub status: QueryStatus,
    pub http_status: Option<u16>,
    pub query_time: Duration,
    pub context: Option<String>,
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.context {
            Some(context) => write!(f, "{:?} ({:?})", self.status, context),
            None => write!(f, "{:?}", self.status,),
        }
    }
}

pub fn add_result_to_channel(
    sender: Sender<RequestResult>,
    username: Arc<String>,
    site: Arc<String>,
    info: Arc<TargetInfo>,
    timeout: u64,
    proxy: Arc<Option<String>>,
) -> color_eyre::Result<()> {
    let encoded_username = &username.replace(" ", "%20");
    let profile_url = info.url.interpolate(encoded_username);
    let url_probe = match &info.url_probe {
        // There is a special URL for probing existence separate
        // from where the user profile normally can be found.
        Some(url_probe) => url_probe.interpolate(encoded_username),
        None => info.url.interpolate(encoded_username),
    };

    let request_body = info
        .request_payload
        .clone()
        .map(|payload| payload.to_string().interpolate(&username));

    tokio::spawn(async move {
        // use regex to make sure the url and username are valid for the site
        if let Some(regex) = &info.regex_check {
            let re = Regex::new(regex)?;
            let is_match = re.is_match(&username)?;
            if !is_match {
                // No need to do the check at the site: this username is not allowed.
                let request_result = RequestResult {
                    username: Arc::clone(&username),
                    site: Arc::clone(&site),
                    info,
                    url: profile_url,
                    url_probe,
                    response: Err(QueryError::InvalidUsernameError),
                    query_time: Duration::from_secs(0),
                };

                sender.send(request_result).await?;
                return Ok::<_, color_eyre::eyre::Report>(());
            }
        }

        let allow_redirects = !matches!(info.error_type, ErrorType::ResponseUrl { .. });

        let req_method = info.request_method.unwrap_or(match info.error_type {
            // In most cases when we are detecting by status code,
            // it is not necessary to get the entire body:  we can
            // detect fine with just the HEAD response.
            ErrorType::StatusCode { .. } => RequestMethod::Head,
            // Either this detect method needs the content associated
            // with the GET response, or this specific website will
            // not respond properly unless we request the whole page.
            _ => RequestMethod::Get,
        });

        let start = Instant::now();
        let resp = make_request(
            &url_probe,
            info.headers.clone(),
            allow_redirects,
            Duration::from_secs(timeout),
            req_method,
            request_body,
            proxy.as_deref(),
            None,
        )
        .await;
        let duration = start.elapsed();

        let request_result = RequestResult {
            username: Arc::clone(&username),
            site: Arc::clone(&site),
            info,
            url: profile_url.clone(),
            url_probe,
            response: resp.map_err(|_| QueryError::RequestError),
            query_time: duration,
        };

        // send to channel
        sender.send(request_result).await?;

        Ok(())
    });

    Ok(())
}
