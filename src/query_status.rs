#[derive(Debug)]
pub enum QueryStatus {
    Claimed,   // username detected
    Available, // username not detected
    Unknown,   // error occured while trying to detect username
    Illegal,   // username not allowed for this site
    Waf,       // request blocked by waf (i.e. Cloudflare)
}
