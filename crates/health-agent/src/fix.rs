use std::path::Path;

use color_eyre::Result;
use serde_json::{Map, Value};

use crate::report::{HealthReport, SiteVerdict};

#[derive(Debug)]
pub struct FixSummary {
    pub removed: Vec<String>,
    pub regex_fixed: Vec<(String, String, String)>, // (site, old_regex, new_regex)
    pub skipped: Vec<(String, SiteVerdict, String)>, // (site, verdict, reason)
}

/// Read a health report and apply deterministic fixes to data.json.
///
/// Returns a summary of what was changed.
pub fn apply_fixes(
    report: &HealthReport,
    data_json_path: &Path,
    dry_run: bool,
) -> Result<FixSummary> {
    let raw = std::fs::read_to_string(data_json_path)?;
    let mut data: Map<String, Value> = serde_json::from_str(&raw)?;

    let mut summary = FixSummary {
        removed: Vec::new(),
        regex_fixed: Vec::new(),
        skipped: Vec::new(),
    };

    for result in &report.results {
        match result.verdict {
            SiteVerdict::Healthy => {}

            SiteVerdict::Unreachable => {
                if data.remove(&result.site).is_some() {
                    summary.removed.push(result.site.clone());
                }
            }

            SiteVerdict::Illegal => {
                if let Some(fixed) = try_fix_regex(&data, &result.site) {
                    summary
                        .regex_fixed
                        .push((result.site.clone(), fixed.old, fixed.new.clone()));
                    if let Some(entry) = data.get_mut(&result.site) {
                        entry["regexCheck"] = Value::String(fixed.new);
                    }
                } else {
                    summary.skipped.push((
                        result.site.clone(),
                        result.verdict.clone(),
                        "Could not auto-fix regex".into(),
                    ));
                }
            }

            SiteVerdict::FalsePositive => {
                // FalsePositive means every username looks "claimed" — the site
                // is actively producing wrong results. Remove it.
                if data.remove(&result.site).is_some() {
                    summary.removed.push(result.site.clone());
                }
            }

            ref v @ (SiteVerdict::FalseNegative
            | SiteVerdict::Inverted
            | SiteVerdict::WafBlocked) => {
                let reason = match v {
                    SiteVerdict::FalseNegative => {
                        "Needs investigation — site may be blocking requests or URL changed"
                    }
                    SiteVerdict::Inverted => "Needs investigation — detection logic is inverted",
                    SiteVerdict::WafBlocked => "Blocked by WAF — may need new fingerprint or proxy",
                    _ => unreachable!(),
                };
                summary
                    .skipped
                    .push((result.site.clone(), v.clone(), reason.into()));
            }

            SiteVerdict::Unknown => {
                summary.skipped.push((
                    result.site.clone(),
                    result.verdict.clone(),
                    "Insufficient data".into(),
                ));
            }
        }
    }

    if !dry_run && (!summary.removed.is_empty() || !summary.regex_fixed.is_empty()) {
        let updated = serde_json::to_string_pretty(&data)?;
        let updated = escape_non_ascii(&updated);
        std::fs::write(data_json_path, updated)?;
    }

    Ok(summary)
}

/// Re-encode non-ASCII characters as `\uXXXX` escape sequences to match
/// the upstream data.json format and avoid noisy diffs.
fn escape_non_ascii(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    let mut in_string = false;
    let mut prev_backslash = false;

    for ch in json.chars() {
        if in_string && !prev_backslash && ch == '"' {
            in_string = false;
            out.push(ch);
        } else if !in_string && ch == '"' {
            in_string = true;
            out.push(ch);
        } else if in_string && !ch.is_ascii() {
            for unit in ch.encode_utf16(&mut [0; 2]) {
                out.push_str(&format!("\\u{unit:04x}"));
            }
        } else {
            out.push(ch);
        }
        prev_backslash = in_string && ch == '\\' && !prev_backslash;
    }
    out
}

struct RegexFix {
    old: String,
    new: String,
}

/// Attempt to fix common regex issues:
/// - Missing quantifier: `^[charset]$` → `^[charset]+$`
/// - Min length too long for username_claimed
fn try_fix_regex(data: &Map<String, Value>, site: &str) -> Option<RegexFix> {
    let entry = data.get(site)?;
    let regex = entry.get("regexCheck")?.as_str()?;
    let claimed = entry.get("username_claimed")?.as_str()?;

    // Case 1: Single-char class like `^[a-zA-Z0-9@_-]$` missing `+`
    // Matches: ^[...]$ where there's no quantifier after the bracket
    if regex.starts_with("^[") && regex.ends_with("]$") && !regex.contains("]{") {
        let new_regex = format!("{}+$", &regex[..regex.len() - 1]);
        // Verify the fix would work for the claimed username
        if fancy_regex::Regex::new(&new_regex)
            .ok()?
            .is_match(claimed)
            .unwrap_or(false)
        {
            return Some(RegexFix {
                old: regex.to_string(),
                new: new_regex,
            });
        }
    }

    // Case 2: Min length in `{min,max}` is too large for the claimed username
    // e.g., `^[a-z0-9]{4,40}$` but claimed username is "bob" (3 chars)
    if let Some(brace_start) = regex.rfind('{') {
        if let Some(brace_end) = regex[brace_start..].find('}') {
            let quantifier = &regex[brace_start + 1..brace_start + brace_end];
            if let Some((min_str, max_str)) = quantifier.split_once(',') {
                if let Ok(min) = min_str.trim().parse::<usize>() {
                    let new_min = claimed.len().min(min);
                    if new_min < min {
                        let new_regex = format!(
                            "{}{{{},{}}}{}",
                            &regex[..brace_start],
                            new_min,
                            max_str.trim(),
                            &regex[brace_start + brace_end + 1..]
                        );
                        if fancy_regex::Regex::new(&new_regex)
                            .ok()?
                            .is_match(claimed)
                            .unwrap_or(false)
                        {
                            return Some(RegexFix {
                                old: regex.to_string(),
                                new: new_regex,
                            });
                        }
                    }
                }
            }
        }
    }

    None
}
