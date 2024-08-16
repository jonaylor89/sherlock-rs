use color_eyre::Result;
use sherlock_rs::{
    default_data::get_default_data, output::save_results, sherlock::check_username,
    sherlock_target_manifest::SherlockTargetManifest,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // TODO: cmdline args
    //
    // TODO: import site_data from json
    let json_data = include_str!("../resources/data.json");
    let initial_data = serde_json::from_str::<SherlockTargetManifest>(json_data)?;

    // let initial_data_str = get_default_data();
    // let initial_data = serde_json::from_str::<SherlockTargetManifest>(&initial_data_str)?;

    // TODO: for username in usernames:
    //          run sherlock
    //          output to txt, xlsx, etc

    let username = "jonaylor89";
    let results = check_username(username.into(), initial_data.targets).await?;

    save_results(results)?;

    Ok(())
}
