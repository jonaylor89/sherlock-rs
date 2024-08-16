use clap::Parser;
use color_eyre::Result;
use sherlock_rs::{
    default_data::get_default_data, output::save_results, sherlock::check_username,
    sherlock_target_manifest::SherlockTargetManifest,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The username to check
    username: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let username = cli.username;

    let json_data = get_default_data();
    // let initial_data = serde_json::from_str::<SherlockTargetManifest>(&initial_data_str)?;
    // let json_data = include_str!("../resources/data.json");
    let deserializer = &mut serde_json::Deserializer::from_str(&json_data);
    let initial_data: SherlockTargetManifest = serde_path_to_error::deserialize(deserializer)
        .map_err(|err| {
            println!("[!!!] error path [{}]", err.path().to_string());
            err
        })?;

    // TODO: for username in usernames:
    //          run sherlock
    //          output to txt, xlsx, etc

    let results = check_username(username.into(), initial_data.targets).await?;

    save_results(results)?;

    Ok(())
}
