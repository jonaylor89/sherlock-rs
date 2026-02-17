use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use rand::seq::SliceRandom;
use sherlock::{
    checker::{check_username, CheckOptions},
    get_data::{get_default_data, get_json_data},
    query::QueryStatus,
    sherlock_target_manifest::{SherlockTargetManifest, TargetInfo},
};

mod fix;
mod report;

use report::{CheckResult, HealthReport, SiteHealthResult, SiteVerdict};

#[derive(Parser)]
#[command(name = "sherlock-health-agent")]
#[command(about = "Automated site health monitor for sherlock")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Probe sites and generate a health report
    Check {
        /// Number of sites to randomly sample (0 = all)
        #[clap(short, long, default_value_t = 0)]
        sample: usize,

        /// Delay in milliseconds between batches of requests
        #[clap(long, default_value_t = 500)]
        batch_delay_ms: u64,

        /// Timeout in seconds for each request
        #[clap(short, long, default_value_t = 30.0)]
        timeout: f64,

        /// Output file path for the JSON health report
        #[clap(short, long, default_value = "health_report.json")]
        output: PathBuf,

        /// Check a specific site only
        #[clap(long)]
        site: Option<String>,

        /// Use local data.json instead of fetching from GitHub
        #[clap(short, long)]
        local: bool,

        /// JSON file or URL to load site data from
        #[clap(
            short,
            long = "json",
            conflicts_with = "local",
            default_value = "https://raw.githubusercontent.com/sherlock-project/sherlock/master/sherlock_project/resources/data.json"
        )]
        json_file: String,
    },

    /// Apply fixes to data.json based on a health report
    Fix {
        /// Path to the health report JSON
        #[clap(short, long, default_value = "health_report.json")]
        report: PathBuf,

        /// Path to the data.json to fix
        #[clap(short, long)]
        data: PathBuf,

        /// Show what would change without writing
        #[clap(long)]
        dry_run: bool,
    },
}

/// A username that is extremely unlikely to be claimed on any site.
const UNCLAIMED_USERNAME: &str = "xq7z_k9m2_sherlock_health_test_unclaimed_99";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Check {
            sample,
            batch_delay_ms,
            timeout,
            output,
            site,
            local,
            json_file,
        } => {
            run_check(
                sample,
                batch_delay_ms,
                timeout,
                output,
                site,
                local,
                json_file,
            )
            .await
        }
        Commands::Fix {
            report,
            data,
            dry_run,
        } => run_fix(report, data, dry_run),
    }
}

async fn run_check(
    sample: usize,
    batch_delay_ms: u64,
    timeout: f64,
    output: PathBuf,
    site: Option<String>,
    local: bool,
    json_file: String,
) -> Result<()> {
    let json_str = match local {
        true => get_default_data(),
        false => get_json_data(json_file).await?,
    };

    let deserializer = &mut serde_json::Deserializer::from_str(&json_str);
    let manifest: SherlockTargetManifest = serde_path_to_error::deserialize(deserializer)
        .inspect_err(|err| {
            eprintln!("[!!!] error path [{}]", err.path());
        })?;

    let mut targets: HashMap<String, Arc<TargetInfo>> = manifest
        .targets
        .into_iter()
        .map(|(site, info)| (site, Arc::new(info)))
        .collect();

    if let Some(ref site_name) = site {
        targets.retain(|name, _| name == site_name);
        if targets.is_empty() {
            return Err(color_eyre::eyre::eyre!(
                "Site '{}' not found in data",
                site_name
            ));
        }
    }

    let site_names: Vec<String> = if sample > 0 && sample < targets.len() {
        let mut names: Vec<String> = targets.keys().cloned().collect();
        names.shuffle(&mut rand::thread_rng());
        names.truncate(sample);
        targets.retain(|name, _| names.contains(name));
        names
    } else {
        targets.keys().cloned().collect()
    };

    eprintln!(
        "[*] Checking {} sites (timeout: {}s, delay: {}ms)",
        targets.len(),
        timeout,
        batch_delay_ms,
    );

    // Create a shared HTTP client for all requests
    let client = Arc::new(
        reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()?,
    );

    let check_options = CheckOptions {
        timeout: Duration::from_secs_f64(timeout),
        proxy: None,
        client,
        print_all: false,
        print_found: false,
        dump_response: false,
        browse: false,
    };

    let mut site_results: Vec<SiteHealthResult> = Vec::with_capacity(targets.len());
    let batch_delay = Duration::from_millis(batch_delay_ms);

    for site_name in &site_names {
        let info = match targets.get(site_name) {
            Some(info) => info,
            None => continue,
        };

        let single_site: HashMap<String, Arc<TargetInfo>> = [(site_name.clone(), Arc::clone(info))]
            .into_iter()
            .collect();
        let single_site = Arc::new(single_site);

        let claimed_username = &info.username_claimed;
        let claimed_results =
            check_username(claimed_username, Arc::clone(&single_site), &check_options).await?;
        let claimed_result = claimed_results.into_iter().next();

        let unclaimed_results =
            check_username(UNCLAIMED_USERNAME, Arc::clone(&single_site), &check_options).await?;
        let unclaimed_result = unclaimed_results.into_iter().next();

        let claimed_check = claimed_result.as_ref().map(|r| CheckResult {
            username: claimed_username.clone(),
            expected: QueryStatus::Claimed,
            actual: r.status.clone(),
            http_status: r.http_status,
            query_time_ms: r.query_time.as_millis() as u64,
            context: r.context.clone(),
        });

        let unclaimed_check = unclaimed_result.as_ref().map(|r| CheckResult {
            username: UNCLAIMED_USERNAME.to_string(),
            expected: QueryStatus::Available,
            actual: r.status.clone(),
            http_status: r.http_status,
            query_time_ms: r.query_time.as_millis() as u64,
            context: r.context.clone(),
        });

        let verdict = SiteVerdict::from_checks(claimed_check.as_ref(), unclaimed_check.as_ref());

        let symbol = match verdict {
            SiteVerdict::Healthy => "+",
            _ => "-",
        };
        eprintln!("[{symbol}] {site_name}: {verdict:?}");

        site_results.push(SiteHealthResult {
            site: site_name.clone(),
            claimed_check,
            unclaimed_check,
            verdict,
        });

        tokio::time::sleep(batch_delay).await;
    }

    let healthy_count = site_results
        .iter()
        .filter(|r| r.verdict == SiteVerdict::Healthy)
        .count();

    eprintln!(
        "\n[*] Results: {}/{} healthy",
        healthy_count,
        site_results.len()
    );

    let report = HealthReport {
        timestamp: Utc::now(),
        total_sites: site_results.len(),
        healthy: healthy_count,
        results: site_results,
    };

    let json = serde_json::to_string_pretty(&report)?;
    std::fs::write(&output, &json)?;
    eprintln!("[*] Report written to {}", output.display());

    Ok(())
}

fn run_fix(report_path: PathBuf, data_path: PathBuf, dry_run: bool) -> Result<()> {
    let report_json = std::fs::read_to_string(&report_path)?;
    let report: HealthReport = serde_json::from_str(&report_json)?;

    if dry_run {
        eprintln!("[*] Dry run — no changes will be written");
    }

    eprintln!(
        "[*] Processing report from {} ({} sites)",
        report.timestamp, report.total_sites
    );

    let summary = fix::apply_fixes(&report, &data_path, dry_run)?;

    if !summary.removed.is_empty() {
        eprintln!("\n  Removed {} sites (unreachable):", summary.removed.len());
        for site in &summary.removed {
            eprintln!("    - {site}");
        }
    }

    if !summary.regex_fixed.is_empty() {
        eprintln!(
            "\n  Fixed {} regexes (illegal username):",
            summary.regex_fixed.len()
        );
        for (site, old, new) in &summary.regex_fixed {
            eprintln!("    - {site}");
            eprintln!("        {old}");
            eprintln!("      → {new}");
        }
    }

    if !summary.skipped.is_empty() {
        eprintln!(
            "\n  Skipped {} sites (need manual investigation):",
            summary.skipped.len()
        );
        for (site, verdict, reason) in &summary.skipped {
            eprintln!("    - {site} [{verdict:?}]: {reason}");
        }
    }

    let total_fixes = summary.removed.len() + summary.regex_fixed.len();
    if total_fixes > 0 && !dry_run {
        eprintln!(
            "\n[*] Applied {total_fixes} fixes to {}",
            data_path.display()
        );
    } else if total_fixes > 0 {
        eprintln!("\n[*] Would apply {total_fixes} fixes (dry run)");
    } else {
        eprintln!("\n[*] No automatic fixes to apply");
    }

    Ok(())
}
