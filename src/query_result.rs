use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub enum QueryStatus {
    /// username detected
    Claimed,
    /// username not detected
    Available,
    /// error occured while trying to detect username
    Unknown,
    /// username not allowed for this site
    Illegal,
    /// request blocked by waf (i.e. Cloudflare)
    Waf,
}

#[derive(Debug)]
pub struct QueryResult {
    pub username: String,
    pub site_name: String,
    pub site_url_user: String,
    pub status: QueryStatus,
    pub query_time: Duration,
    pub context: Option<String>,
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.context {
            Some(context) => write!(f, "{:?} ({:?})", self.status, context),
            None => write!(f, "{:?}", self.status,),
        }
    }
}
