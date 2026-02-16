use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use sherlock::checker::{check_username, CheckOptions};
use sherlock::query::QueryStatus;
use sherlock::sherlock_target_manifest::{ErrorCode, ErrorMsg, ErrorType, TargetInfo};
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

fn default_options() -> CheckOptions {
    CheckOptions {
        timeout: Duration::from_secs(10),
        proxy: None,
        print_all: false,
        print_found: false,
        dump_response: false,
        browse: false,
    }
}

fn make_target(mock_uri: &str, error_type: ErrorType) -> TargetInfo {
    TargetInfo {
        url: format!("{mock_uri}/users/{{}}"),
        url_main: mock_uri.to_string(),
        url_probe: None,
        username_claimed: "testuser".to_string(),
        regex_check: None,
        is_nsfw: None,
        headers: None,
        request_payload: None,
        __comment__: None,
        tags: None,
        request_method: None,
        error_type,
    }
}

fn site_data_from(name: &str, info: TargetInfo) -> Arc<HashMap<String, Arc<TargetInfo>>> {
    let mut map = HashMap::new();
    map.insert(name.to_string(), Arc::new(info));
    Arc::new(map)
}

fn init_color_eyre() {
    let _ = color_eyre::install();
}

#[tokio::test]
async fn test_status_code_claimed() {
    init_color_eyre();

    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let info = make_target(
        &mock_server.uri(),
        ErrorType::StatusCode { codes: None },
    );
    let site_data = site_data_from("TestSite", info);
    let results = check_username("testuser", site_data, &default_options())
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, QueryStatus::Claimed);
}

#[tokio::test]
async fn test_status_code_available() {
    init_color_eyre();

    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let info = make_target(
        &mock_server.uri(),
        ErrorType::StatusCode { codes: None },
    );
    let site_data = site_data_from("TestSite", info);
    let results = check_username("testuser", site_data, &default_options())
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, QueryStatus::Available);
}

#[tokio::test]
async fn test_status_code_with_explicit_error_codes() {
    init_color_eyre();

    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let info = make_target(
        &mock_server.uri(),
        ErrorType::StatusCode {
            codes: Some(ErrorCode::Multiple(vec![404, 410])),
        },
    );
    let site_data = site_data_from("TestSite", info);
    let results = check_username("testuser", site_data, &default_options())
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, QueryStatus::Available);
}

#[tokio::test]
async fn test_message_claimed() {
    init_color_eyre();

    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_string("Welcome to the profile page"))
        .mount(&mock_server)
        .await;

    let info = make_target(
        &mock_server.uri(),
        ErrorType::Message {
            msg: ErrorMsg::Single("User not found".into()),
        },
    );
    let site_data = site_data_from("TestSite", info);
    let results = check_username("testuser", site_data, &default_options())
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, QueryStatus::Claimed);
}

#[tokio::test]
async fn test_message_available() {
    init_color_eyre();

    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_string("User not found"))
        .mount(&mock_server)
        .await;

    let info = make_target(
        &mock_server.uri(),
        ErrorType::Message {
            msg: ErrorMsg::Single("User not found".into()),
        },
    );
    let site_data = site_data_from("TestSite", info);
    let results = check_username("testuser", site_data, &default_options())
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, QueryStatus::Available);
}

#[tokio::test]
async fn test_waf_detection() {
    init_color_eyre();

    // Must contain the exact WAF fingerprint from waf.rs:
    // r#"{return l.onPageView}}),Object.defineProperty(r,"perimeterxIdentifiers",{enumerable:"#
    let waf_body = r#"some prefix {return l.onPageView}}),Object.defineProperty(r,"perimeterxIdentifiers",{enumerable: some suffix"#;

    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_string(waf_body))
        .mount(&mock_server)
        .await;

    let info = make_target(
        &mock_server.uri(),
        ErrorType::Message {
            msg: ErrorMsg::Single("will not match".into()),
        },
    );

    let site_data = site_data_from("TestSite", info);
    let results = check_username("testuser", site_data, &default_options())
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, QueryStatus::Waf);
}

#[tokio::test]
async fn test_regex_check_illegal() {
    init_color_eyre();

    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let mut info = make_target(
        &mock_server.uri(),
        ErrorType::StatusCode { codes: None },
    );
    info.regex_check = Some("^[a-zA-Z0-9]+$".into());

    let site_data = site_data_from("TestSite", info);
    let results = check_username("user@name", site_data, &default_options())
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, QueryStatus::Illegal);
}

#[tokio::test]
async fn test_timeout_returns_unknown() {
    init_color_eyre();

    let mock_server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_string("ok").set_delay(Duration::from_secs(5)))
        .mount(&mock_server)
        .await;

    let info = make_target(
        &mock_server.uri(),
        ErrorType::StatusCode { codes: None },
    );
    let mut options = default_options();
    options.timeout = Duration::from_secs(1);

    let site_data = site_data_from("TestSite", info);
    let results = check_username("testuser", site_data, &options)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, QueryStatus::Unknown);
}
