use crate::query_result::{QueryResult, QueryStatus};
use color_eyre::Result;
use colored::Colorize;

pub fn save_results(results: Vec<QueryResult>) -> Result<()> {
    println!("total of {} results", results.len());

    // save results to file
    Ok(())
}

pub fn print_result(result: &QueryResult) {
    let response_time_text = format!("[{}ms]", result.query_time.as_millis());
    match result.status {
        QueryStatus::Claimed => {
            println!(
                "{}{}{} {} {}: {}",
                "[".white(),
                "+".green(),
                "]".white(),
                response_time_text.white(),
                result.site_name.green(),
                result.site_url_user.green(),
            );
        }
        QueryStatus::Available => {
            println!(
                "{}{}{} {} {}: {}",
                "[".white(),
                "-".red(),
                "]".white(),
                response_time_text.white(),
                result.site_name.green(),
                "Not Found!".yellow(),
            );
        }
        QueryStatus::Unknown => {
            println!(
                "{}{}{} {} {}: {:?}",
                "[".white(),
                "-".red(),
                "]".white(),
                response_time_text.white(),
                result.site_name.green(),
                result.context,
            );
        }
        QueryStatus::Illegal => {
            println!(
                "{}{}{} {} {}: {}",
                "[".white(),
                "-".red(),
                "]".white(),
                response_time_text.white(),
                result.site_name.green(),
                "Illegal Username Foramt For This Site!".yellow(),
            );
        }
        QueryStatus::Waf => {
            println!(
                "{}{}{} {} {} {}",
                "[".white(),
                "-".red(),
                "]".white(),
                result.site_name.green(),
                "Blocked by boy detection".red(),
                "(proxy may help)".yellow(),
            );
        }
    };
}
