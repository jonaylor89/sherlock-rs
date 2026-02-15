use clap::Parser;
use color_eyre::Result;
use sherlock::{
    checker::{check_username, CheckOptions},
    get_data::{get_default_data, get_json_data},
    output::{save_results, SaveOptions},
    sherlock_target_manifest::{SherlockTargetManifest, TargetInfo},
    utils::create_username_variants,
};
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(name = "sherlock")]
#[command(author = "Johannes Naylor <jonaylor89@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Hunt down social media accounts by username", long_about = None)]
struct Cli {
    /// One or more usernames to check with social networks. Check similar usernames using {?} (replace to '_', '-', '.').
    #[clap(name = "usernames", required = true)]
    usernames: Vec<String>,

    /// Display extra debugging information and metrics.
    #[clap(short, long, alias = "debug")]
    verbose: bool,

    /// The output file to save the results to.
    #[clap(short, long = "output")]
    output_file: Option<String>,

    /// If using single username, the output of the result will be saved to this file.
    #[clap(short = 'f', long, alias = "output-folder")]
    output_folder: Option<String>,

    /// Create Comma-Separated Values (CSV) File.
    #[clap(short, long, alias = "csv")]
    csv: bool,

    /// Create the standard file for the modern Microsoft Excel spreadsheet (xlsx).
    #[clap(long)]
    xlsx: bool,

    /// Limit analysis to just the listed sites. Add multiple options to specify more than one site.
    #[clap(short, long)]
    site_list: Vec<String>,

    // Make requests over a proxy. e.g. socks5://127.0.0.1:1080
    #[clap(short, long, alias = "proxy")]
    proxy: Option<String>,

    /// Dump the HTTP request to stdout for targeted debugging.
    #[clap(short, long)]
    dump_response: bool,

    /// Load data from a JSON file or an online, valid, JSON file.
    #[clap(
        short,
        long = "json",
        conflicts_with = "local",
        default_value = "https://raw.githubusercontent.com/sherlock-project/sherlock/master/sherlock_project/resources/data.json"
    )]
    json_file: String,

    /// Time (in seconds) to wait for response to requests.
    #[clap(short, long, alias = "timeout", default_value_t = 60.0)]
    timeout: f64,

    /// Output sites where the username was not found.
    #[clap(long, alias = "print-all")]
    print_all: bool,

    /// Output sites where the username was found.
    #[clap(long, alias = "print-found", default_value_t = true)]
    print_found: bool,

    /// Browse to all results on default browser.
    #[clap(short, long, alias = "browse")]
    browse: bool,

    /// Use local data file instead of fetching the latest version online.
    #[clap(short, long)]
    local: bool,

    /// Include checking of NSFW sites from default list.
    #[clap(long, alias = "nsfw", default_value_t = true)]
    nsfw: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let json_str = match cli.local {
        true => get_default_data(),
        false => get_json_data(cli.json_file).await?,
    };

    // let json_data = include_str!("../resources/data.json");
    let deserializer = &mut serde_json::Deserializer::from_str(&json_str);
    let initial_data: SherlockTargetManifest = serde_path_to_error::deserialize(deserializer)
        .inspect_err(|err| {
            println!("[!!!] error path [{}]", err.path());
        })?;

    let targets = match cli.nsfw {
        true => initial_data.targets,
        false => initial_data
            .targets
            .into_iter()
            .filter(|(_, info)| !info.is_nsfw.unwrap_or(false))
            .collect(),
    };

    let filtered_targets = match cli.site_list.is_empty() {
        true => targets,
        false => targets
            .into_iter()
            .filter(|(site, _)| cli.site_list.contains(site))
            .collect(),
    };

    let arc_targets = filtered_targets
        .into_iter()
        .map(|(site, info)| (site, Arc::new(info)))
        .collect::<HashMap<String, Arc<TargetInfo>>>();
    let arc_targets = Arc::new(arc_targets);

    let username_variants = create_username_variants(&cli.usernames);

    let check_options = CheckOptions {
        timeout: Duration::from_secs_f64(cli.timeout),
        proxy: cli.proxy.map(Arc::from),
        print_all: cli.print_all,
        print_found: cli.print_found,
        dump_response: cli.dump_response,
        browse: cli.browse,
    };

    let save_options = SaveOptions {
        output_file: cli.output_file,
        output_folder: cli.output_folder,
        csv: cli.csv,
        xlsx: cli.xlsx,
        print_all: cli.print_all,
        print_found: cli.print_found,
    };

    for username in username_variants {
        let results = check_username(&username, Arc::clone(&arc_targets), &check_options).await?;
        save_results(&username, &results, &save_options)?;
    }

    Ok(())
}
