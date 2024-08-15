#[derive(Debug)]
pub struct SiteInfo {
    name: String,
    url_home: String,
    url_username_format: String,
    username_claimed: String,
    username_unclaimed: String,
    information: &HashMap<String, String>,
    is_nsfw: bool,
}

impl fmt::Display for SiteInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.url_home)
    }
}
