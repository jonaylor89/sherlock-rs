use crate::query_result::{QueryResult, QueryStatus};
use color_eyre::Result;
use colored::Colorize;
use rust_xlsxwriter::Workbook;
use std::fs::File;
use std::io::Write;

pub fn save_results(
    username: &str,
    results: Vec<QueryResult>,
    output_file: Option<&String>,
    output_folder: Option<&String>,
    csv: bool,
    xlsx: bool,
    print_all: bool,
    print_found: bool,
) -> Result<()> {
    let total_hits = results
        .iter()
        .filter(|result| result.status == QueryStatus::Claimed)
        .count();

    println!("total of {}/{} hits", total_hits, results.len());

    if let Some(output_folder) = output_folder {
        // make sure the output folder exists
        std::fs::create_dir_all(&output_folder)?;
    }

    let output_file = match (output_file, output_folder) {
        (Some(output_file), _) => output_file.to_string(),
        (None, Some(output_folder)) => format!("{}/{}.txt", output_folder, username),
        (None, None) => format!("{}.txt", username),
    };

    let mut file = File::create(&output_file)?;
    for result in &results {
        if result.status == QueryStatus::Claimed {
            writeln!(file, "{}", result.site_url_user)?;
        }
    }

    writeln!(file, "Total Websites Username Detected On: {}", total_hits)?;

    if csv {
        write_csv(&username, &results, output_folder, print_all, print_found)?;
    }

    if xlsx {
        write_xlsx(&username, &results, output_folder, print_all, print_found)?;
    }

    // if args.xlsx:
    //     usernames = []
    //     names = []
    //     url_main = []
    //     url_user = []
    //     exists = []
    //     http_status = []
    //     response_time_s = []

    //     for site in results:
    //         if (
    //             args.print_found
    //             and not args.print_all
    //             and results[site]["status"].status != QueryStatus.CLAIMED
    //         ):
    //             continue

    //         if response_time_s is None:
    //             response_time_s.append("")
    //         else:
    //             response_time_s.append(results[site]["status"].query_time)
    //         usernames.append(username)
    //         names.append(site)
    //         url_main.append(results[site]["url_main"])
    //         url_user.append(results[site]["url_user"])
    //         exists.append(str(results[site]["status"].status))
    //         http_status.append(results[site]["http_status"])

    //     DataFrame = pd.DataFrame(
    //         {
    //             "username": usernames,
    //             "name": names,
    //             "url_main": url_main,
    //             "url_user": url_user,
    //             "exists": exists,
    //             "http_status": http_status,
    //             "response_time_s": response_time_s,
    //         }
    //     )
    //     DataFrame.to_excel(f"{username}.xlsx", sheet_name="sheet1", index=False)

    // save results to file
    Ok(())
}

pub fn write_xlsx(
    username: &str,
    results: &Vec<QueryResult>,
    output_folder: Option<&String>,
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

pub fn write_csv(
    username: &str,
    results: &Vec<QueryResult>,
    output_folder: Option<&String>,
    print_all: bool,
    print_found: bool,
) -> color_eyre::Result<()> {
    let csv_filename = match output_folder {
        None => format!("{}.csv", username),
        Some(folder) => format!("{}/{}.csv", folder, username),
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
            "{},{},{},{},{},{},{}",
            username,
            result.site_name,
            result.url_main,
            result.site_url_user,
            format!("{:?}", result.status),
            result.http_status.as_ref().unwrap_or(&0),
            response_time_s
        )?;
    }

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
