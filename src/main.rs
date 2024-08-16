use std::collections::HashMap;

use color_eyre::Result;
use sherlock_rs::{
    output::save_results,
    sherlock::check_username,
    sherlock_target_manifest::{RequestMethod, TargetInfo},
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // TODO: cmdline args
    //
    // TODO: import site_data from json
    //
    // TODO: for username in usernames:
    //          run sherlock
    //          output to txt, xlsx, etc

    let username = "jonaylor89";
    let mut site_data = HashMap::new();
    site_data.insert(
        "Playstore".into(),
        TargetInfo {
            error_type: sherlock_rs::sherlock_target_manifest::ErrorType::StatusCode,
            url: "https://play.google.com/store/apps/developer?id={}".into(),
            url_main: "https://play.google.com/store".into(),
            username_claimed: "Facebook".into(),
            url_probe: None,
            regex_check: None,
            is_nsfw: None,
            headers: None,
            request_payload: None,
            __comment__: None,
            tags: None,
            request_method: None,
            error_msg: None,
            error_code: None,
            error_url: None,
            response_url: None,
        },
    );
    site_data.insert(
        "SoundCloud".into(),
        TargetInfo {
            error_type: sherlock_rs::sherlock_target_manifest::ErrorType::StatusCode,
            url: "https://soundcloud.com/{}".into(),
            url_main: "https://soundcloud.com/".into(),
            username_claimed: "blue".into(),
            url_probe: None,
            regex_check: None,
            is_nsfw: None,
            headers: None,
            request_payload: None,
            __comment__: None,
            tags: None,
            request_method: None,
            error_msg: None,
            error_code: None,
            error_url: None,
            response_url: None,
        },
    );
    site_data.insert(
        "Snapchat".into(),
        TargetInfo {
            error_type: sherlock_rs::sherlock_target_manifest::ErrorType::StatusCode,
            regex_check: Some("^[a-z][a-z-_.]{3,15}".into()),
            request_method: Some(RequestMethod::Get),
            url: "https://www.snapchat.com/add/{}".into(),
            url_main: "https://www.snapchat.com".into(),
            username_claimed: "teamsnapchat".into(),
            url_probe: None,
            is_nsfw: None,
            headers: None,
            request_payload: None,
            __comment__: None,
            tags: None,
            error_msg: None,
            error_code: None,
            error_url: None,
            response_url: None,
        },
    );

    let results = check_username(username.into(), site_data).await?;

    save_results(results)?;

    Ok(())
}
