use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use std::{collections::HashMap, sync::Arc, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect::Policy,
    Client, Proxy, Response,
};

use crate::{
    query::QueryError,
    sherlock_target_manifest::{RequestMethod, TargetInfo},
};

static USER_AGENTS: Lazy<[&str; 8]> = Lazy::new(|| {
    [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:115.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.67",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_0_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.1 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_0_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12.0; rv:115.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0",
]
});

#[derive(Debug)]
pub struct RequestResult {
    pub username: Arc<String>,
    pub site: Arc<String>,
    pub info: Arc<TargetInfo>,
    pub url: String,
    pub url_probe: String,
    pub response: Result<Response, QueryError>,
    pub query_time: Duration,
}

pub async fn make_request(
    url: &str,
    headers: Option<HashMap<String, String>>,
    allow_redirects: bool,
    timeout: std::time::Duration,
    method: RequestMethod,
    request_payload: Option<String>,
    proxy: Option<&str>,
    user_agent: Option<String>,
) -> color_eyre::Result<Response> {
    let redirect_policy = match allow_redirects {
        true => Policy::limited(5),
        false => Policy::none(),
    };

    let headers_map = headers
        .unwrap_or_default()
        .into_iter()
        .map(|(key, value)| {
            let header_name = key.parse::<HeaderName>().unwrap();
            let header_value = value.parse::<HeaderValue>().unwrap();
            (header_name, header_value)
        })
        .collect::<HeaderMap>();

    let req_method = match method {
        RequestMethod::Get => reqwest::Method::GET,
        RequestMethod::Post => reqwest::Method::POST,
        RequestMethod::Put => reqwest::Method::PUT,
        RequestMethod::Head => reqwest::Method::HEAD,
    };

    let random_agent = USER_AGENTS
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned();

    let req_user_agent = user_agent.unwrap_or(random_agent.into());

    let mut builder = Client::builder()
        .default_headers(headers_map)
        .user_agent(req_user_agent)
        .timeout(timeout)
        .redirect(redirect_policy);

    if let Some(proxy) = proxy {
        builder = builder.proxy(Proxy::all(proxy)?);
    }

    let client = builder.build()?;

    let resp = client
        .request(req_method, url)
        .json(&request_payload)
        .send()
        .await?;

    Ok(resp)
}
