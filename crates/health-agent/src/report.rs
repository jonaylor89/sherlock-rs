use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sherlock::query::QueryStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthReport {
    pub timestamp: DateTime<Utc>,
    pub total_sites: usize,
    pub healthy: usize,
    pub results: Vec<SiteHealthResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteHealthResult {
    pub site: String,
    pub claimed_check: Option<CheckResult>,
    pub unclaimed_check: Option<CheckResult>,
    pub verdict: SiteVerdict,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckResult {
    pub username: String,
    pub expected: QueryStatus,
    pub actual: QueryStatus,
    pub http_status: Option<u16>,
    pub query_time_ms: u64,
    pub context: Option<String>,
}

impl CheckResult {
    pub fn passed(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiteVerdict {
    /// Both checks passed — site detection is working correctly
    Healthy,
    /// Known-claimed user was not detected — we're missing real users
    FalseNegative,
    /// Known-unclaimed user was detected as claimed — everything looks claimed
    FalsePositive,
    /// Both checks returned the opposite of expected — detection logic is inverted
    Inverted,
    /// Blocked by WAF/bot detection
    WafBlocked,
    /// Site is unreachable or errored out
    Unreachable,
    /// Username was rejected as illegal for both checks (broken regexCheck in data.json)
    Illegal,
    /// Could not determine (e.g., missing check results)
    Unknown,
}

impl SiteVerdict {
    pub fn from_checks(claimed: Option<&CheckResult>, unclaimed: Option<&CheckResult>) -> Self {
        match (claimed, unclaimed) {
            (Some(c), Some(u)) => {
                // Check for WAF on either
                if c.actual == QueryStatus::Waf || u.actual == QueryStatus::Waf {
                    return SiteVerdict::WafBlocked;
                }

                // Check for unreachable on either
                if c.actual == QueryStatus::Unknown || u.actual == QueryStatus::Unknown {
                    return SiteVerdict::Unreachable;
                }

                // Check for illegal username on both
                if c.actual == QueryStatus::Illegal && u.actual == QueryStatus::Illegal {
                    return SiteVerdict::Illegal;
                }

                // If the unclaimed username was rejected as illegal by the site's
                // regex, that's fine — the site correctly wouldn't match it.
                // Only the claimed check matters in that case.
                let unclaimed_ok = u.passed() || u.actual == QueryStatus::Illegal;

                match (c.passed(), unclaimed_ok) {
                    (true, true) => SiteVerdict::Healthy,
                    (false, true) => SiteVerdict::FalseNegative,
                    (true, false) => SiteVerdict::FalsePositive,
                    (false, false) => SiteVerdict::Inverted,
                }
            }
            _ => SiteVerdict::Unknown,
        }
    }
}
