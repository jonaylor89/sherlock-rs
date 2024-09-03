use std::{fs::File, io::Read, path::Path};

use color_eyre::eyre;

pub async fn get_json_data(json_file: String) -> color_eyre::Result<String> {
    // Ensure that the specified data file has the correct extension.
    if !json_file.to_lowercase().ends_with(".json") {
        return Err(eyre::eyre!(
            "Incorrect JSON file extension for data file '{}'.",
            json_file
        ));
    }

    let json_str = match json_file.to_lowercase().starts_with("http") {
        true => {
            // Reference is to a URL.
            let response = reqwest::get(&json_file).await?;

            if !response.status().is_success() {
                return Err(eyre::eyre!(
                    "Bad response while accessing data file URL '{}'.",
                    &json_file
                ));
            }

            let site_data: String = response.text().await.map_err(|error| {
                eyre::eyre!(
                    "Problem parsing JSON contents at '{}': {}.",
                    json_file,
                    error
                )
            })?;

            site_data
        }
        false => {
            // Reference is to a file.
            let path = Path::new(&json_file);

            let mut file = File::open(path).map_err(|_| {
                eyre::eyre!(
                    "Problem while attempting to access data file '{}'.",
                    json_file
                )
            })?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            contents
        }
    };

    Ok(json_str)
}

/// the default sites to check for sherlock locally
/// includes >400 websites and their error messages
pub fn get_default_data() -> String {
    include_str!("data.json").to_string()
}
