use crate::query_status::QueryStatus;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct QueryResult {
    username: String,
    site_name: String,
    site_url_user: String,
    status: QueryStatus,
    query_time: Duration,
    context: Option<String>,
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.context {
            Some(context) => write!(f, "{:?} ({:?})", self.status, context),
            None => write!(f, "{:?}", self.status,),
        }
    }
}
