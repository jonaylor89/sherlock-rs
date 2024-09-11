use crate::query::{QueryResult, QueryStatus};
use color_eyre::Result;
use colored::Colorize;

#[cfg(feature = "xlsx")]
use rust_xlsxwriter::Workbook;

use std::fs::File;
use std::io::Write;

/// Options for saving results
pub struct SaveOptions {
    pub output_file: Option<String>,
    pub output_folder: Option<String>,
    pub csv: bool,
    pub xlsx: bool,
    pub print_all: bool,
    pub print_found: bool,
}

/// Save the results to a file.
///
/// # Arguments
/// * `username` - The username to save the results for.
/// * `results` - The results to save.
/// * `output_file` - The output file to save the results to.
/// * `output_folder` - The output folder to save the results to.
/// * `csv` - Save the results to a CSV file.
/// * `xlsx` - Save the results to an XLSX file.
/// * `print_all` - Print all results.
/// * `print_found` - Print only found results.
///
/// # Returns
/// A Result containing the success or failure of the operation.
pub fn save_results(
    username: &str,
    results: &[QueryResult],
    options: &SaveOptions,
) -> Result<()> {
    let total_hits = results
        .iter()
        .filter(|result| result.status == QueryStatus::Claimed)
        .count();

    println!("total of {}/{} hits", total_hits, results.len());

    if let Some(ref output_folder) = options.output_folder {
        // make sure the output folder exists
        std::fs::create_dir_all(output_folder)?;
    }

    let output_file = match (&options.output_file, &options.output_folder) {
        (Some(output_file), _) => output_file.to_string(),
        (None, Some(output_folder)) => format!("{output_folder}/{username}.txt"),
        (None, None) => format!("{username}.txt"),
    };

    let mut file = File::create(&output_file)?;
    for result in results {
        if result.status == QueryStatus::Claimed {
            writeln!(file, "{}", result.site_url_user)?;
        }
    }

    writeln!(file, "Total Websites Username Detected On: {total_hits}")?;

    if options.csv {
        write_csv(
            username,
            &results,
            options.output_folder.as_deref(),
            options.print_all,
            options.print_found,
        )?;
    }

    if options.xlsx {
        #[cfg(feature = "xlsx")]
        write_xlsx(&username, &results, output_folder, print_all, print_found)?;

        #[cfg(not(feature = "xlsx"))]
        eprintln!("Error: xlsx support is not enabled");
    }

    Ok(())
}

/// Write the results to a Excel File.
/// # Arguments
/// * `username` - The username to save the results for.
/// * `results` - The results to save.
/// * `output_folder` - The output folder to save the results to.
/// * `print_all` - Print all results.
/// * `print_found` - Print only found results.
///
/// # Returns
/// A Result containing the success or failure of the operation.
#[cfg(feature = "xlsx")]
pub fn write_xlsx(
    username: &str,
    results: &Vec<QueryResult>,
    output_folder: Option<&str>,
    print_all: bool,
    print_found: bool,
) -> color_eyre::Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Write the header
    worksheet.write_string(0, 0, "username")?;
    worksheet.write_string(0, 1, "name")?;
    worksheet.write_string(0, 2, "url_main")?;
    worksheet.write_string(0, 3, "url_user")?;
    worksheet.write_string(0, 4, "exists")?;
    worksheet.write_string(0, 5, "http_status")?;
    worksheet.write_string(0, 6, "response_time_s")?;

    let mut row = 1;
    for result in results {
        if print_found && !print_all && result.status != QueryStatus::Claimed {
            continue;
        }

        let response_time_s = result.query_time.as_secs();

        worksheet.write_string(row, 0, username)?;
        worksheet.write_string(row, 1, &result.site_name)?;
        worksheet.write_string(row, 2, &result.url_main)?;
        worksheet.write_string(row, 3, &result.site_url_user)?;
        worksheet.write_string(row, 4, &format!("{:?}", result.status))?;
        worksheet.write_number(row, 5, result.http_status.unwrap_or(0) as f64)?;
        worksheet.write_number(row, 6, response_time_s as f64)?;

        row += 1;
    }

    let xlsx_filename = match output_folder {
        None => format!("{}.xlsx", username),
        Some(folder) => format!("{}/{}.xlsx", folder, username),
    };

    workbook.save(xlsx_filename)?;

    Ok(())
}

/// Write the results to a CSV File.
/// # Arguments
/// * `username` - The username to save the results for.
/// * `results` - The results to save.
/// * `output_folder` - The output folder to save the results to.
/// * `print_all` - Print all results.
/// * `print_found` - Print only found results.
///
/// # Returns
/// A Result containing the success or failure of the operation.
pub fn write_csv(
    username: &str,
    results: &[QueryResult],
    output_folder: Option<&str>,
    print_all: bool,
    print_found: bool,
) -> color_eyre::Result<()> {
    let csv_filename = match output_folder {
        None => format!("{username}.csv"),
        Some(folder) => format!("{folder}/{username}.csv"),
    };

    let mut csv_report = File::create(csv_filename)?;

    // Write the CSV header
    writeln!(
        csv_report,
        "username,name,url_main,url_user,exists,http_status,response_time_s"
    )?;

    // Iterate over the results and write each row
    for result in results {
        if print_found && !print_all && result.status != QueryStatus::Claimed {
            continue;
        }

        let response_time_s = result.query_time.as_secs();

        writeln!(
            csv_report,
            "{},{},{},{},{:?},{},{}",
            username,
            result.site_name,
            result.info.url_main,
            result.site_url_user,
            result.status,
            result.http_status.as_ref().unwrap_or(&0),
            response_time_s
        )?;
    }

    Ok(())
}

/// Print the results to the console.
///
/// # Arguments
/// * `results` - The results from checking the username.
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
                result.site_url_user,
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
                "{}{}{} {} {}: {}",
                "[".white(),
                "-".red(),
                "]".white(),
                response_time_text.white(),
                result.site_name.green(),
                result
                    .context
                    .as_ref()
                    .unwrap_or(&String::from("no context"))
                    .yellow(),
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
                "Blocked by bot detection".red(),
                "(proxy may help)".yellow(),
            );
        }
    };
}
