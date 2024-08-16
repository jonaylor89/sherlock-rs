use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub enum QueryStatus {
    Claimed,   // username detected
    Available, // username not detected
    Unknown,   // error occured while trying to detect username
    Illegal,   // username not allowed for this site
    Waf,       // request blocked by waf (i.e. Cloudflare)
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
