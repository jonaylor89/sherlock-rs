use crate::{
    interpolate::Interpolatable,
    output::print_result,
    query_result::{QueryResult, QueryStatus},
    sherlock_target_manifest::{ErrorType, RequestMethod, TargetInfo},
};
use color_eyre::eyre;
use fancy_regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect::Policy,
    Client, Proxy, Response,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use thiserror::Error;
use tokio::sync::mpsc::{channel, Sender};

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Invalid username")]
    InvalidUsernameError,
    #[error("Request error")]
    RequestError,
}

#[derive(Debug)]
pub struct RequestResult {
    pub username: String,
    pub site: String,
    pub info: TargetInfo,
    pub url: String,
    pub response: Result<Response, QueryError>,
    pub query_time: Duration,
}

pub async fn check_username(
    username: &String,
    site_data: HashMap<String, TargetInfo>,
    timeout: u64,
    proxy: Option<&String>,
) -> color_eyre::Result<Vec<QueryResult>> {
    let num_of_sites = site_data.keys().len();
    if num_of_sites == 0 {
        return Err(eyre::eyre!("No sites to check"));
    }

    let (tx, mut rx) = channel::<RequestResult>(num_of_sites);

    // ping sites for username matches
    for (site, info) in site_data.into_iter() {
        add_result_to_channel(
            username.clone(),
            site,
            info,
            timeout,
            proxy.map(|s| s.clone()),
            tx.clone(),
        )?;
    }

    drop(tx);

    // save to output data struct
    let mut results = vec![];
    while let Some(result) = rx.recv().await {
        let site = result.site;
        let info = result.info;
        let url = result.url;
        let username = result.username;

        let query_result: QueryResult = match result.response {
            Err(e) => match e {
                QueryError::InvalidUsernameError => QueryResult {
                    username: username.clone(),
                    site_name: site,
                    url_main: info.url_main,
                    site_url_user: url,
                    status: QueryStatus::Illegal,
                    http_status: None,
                    query_time: result.query_time,
                    context: Some(e.to_string()),
                },
                QueryError::RequestError => QueryResult {
                    username: username.clone(),
                    site_name: site,
                    url_main: info.url_main,
                    site_url_user: url,
                    status: QueryStatus::Unknown,
                    http_status: None,
                    query_time: result.query_time,
                    context: Some(e.to_string()),
                },
            },
            Ok(response) => {
                // As WAFs advance and evolve, they will occasionally block Sherlock and
                // lead to false positives and negatives. Fingerprints should be added
                // here to filter results that fail to bypass WAFs. Fingerprints should
                // be highly targetted. Comment at the end of each fingerprint to
                // indicate target and date fingerprinted.
                let wafhit_msgs = [
                    r#".loading-spinner{visibility:hidden}body.no-js .challenge-running{display:none}body.dark{background-color:#222;color:#d9d9d9}body.dark a{color:#fff}body.dark a:hover{color:#ee730a;text-decoration:underline}body.dark .lds-ring div{border-color:#999 transparent transparent}body.dark .font-red{color:#b20f03}body.dark"#,
                    // 2024-04-09 PerimeterX / Human Security
                    r#"{return l.onPageView}}),Object.defineProperty(r,"perimeterxIdentifiers",{enumerable:"#,
                ];

                let status_code = response.status().as_u16();
                let resp_text = response.text().await?;
                let wfthit = wafhit_msgs.iter().any(|msg| resp_text.contains(msg));

                let error_type = info.error_type;
                let status = match (wfthit, error_type) {
                    (true, _) => QueryStatus::Waf,
                    (false, ErrorType::Message) => {
                        let error_flag = info.error_msg.iter().any(|msg| msg.is_in(&resp_text));
                        if error_flag {
                            QueryStatus::Available
                        } else {
                            QueryStatus::Claimed
                        }
                    }
                    (false, ErrorType::StatusCode) => {
                        let error_codes = info.error_code;
                        let mut status = QueryStatus::Claimed;

                        if let Some(error_codes) = error_codes {
                            if error_codes.contains(&status_code) {
                                status = QueryStatus::Available;
                            }
                        } else {
                            if &status_code > &399 || &status_code < &200 {
                                status = QueryStatus::Available;
                            }
                        }

                        status
                    }
                    (false, ErrorType::ResponseUrl) => {
                        if (200..300).contains(&status_code) {
                            QueryStatus::Claimed
                        } else {
                            QueryStatus::Available
                        }
                    }
                };

                QueryResult {
                    username: username.clone(),
                    site_name: site,
                    url_main: info.url_main,
                    site_url_user: url,
                    status,
                    http_status: Some(status_code),
                    query_time: result.query_time,
                    context: None,
                }
            }
        };

        print_result(&query_result);
        results.push(query_result);
    }

    Ok(results)
}

pub fn add_result_to_channel(
    username: String,
    site: String,
    info: TargetInfo,
    timeout: u64,
    proxy: Option<String>,
    sender: Sender<RequestResult>,
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
                    username: username.clone(),
                    site,
                    info,
                    url: profile_url,
                    response: Err(QueryError::InvalidUsernameError),
                    query_time: Duration::from_secs(0),
                };

                sender.send(request_result).await?;
                return Ok::<_, color_eyre::eyre::Report>(());
            }
        }

        let allow_redirects = !matches!(info.error_type, ErrorType::ResponseUrl);

        let req_method = info.request_method.unwrap_or(match info.error_type {
            // In most cases when we are detecting by status code,
            // it is not necessary to get the entire body:  we can
            // detect fine with just the HEAD response.
            ErrorType::StatusCode => RequestMethod::Head,
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
            username: username.clone(),
            site,
            info,
            url: profile_url.clone(),
            response: resp.map_err(|_| QueryError::RequestError),
            query_time: duration,
        };

        // send to channel
        sender.send(request_result).await?;

        Ok(())
    });

    Ok(())
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

    let req_user_agent = user_agent
        .unwrap_or("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0".into());

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
