// As WAFs advance and evolve, they will occasionally block Sherlock and
// lead to false positives and negatives. Fingerprints should be added
// here to filter results that fail to bypass WAFs. Fingerprints should
// be highly targetted. Comment at the end of each fingerprint to
// indicate target and date fingerprinted.
const WAFHIT_MSGS: [&str; 2] = [
    r#".loading-spinner{visibility:hidden}body.no-js .challenge-running{display:none}body.dark{background-color:#222;color:#d9d9d9}body.dark a{color:#fff}body.dark a:hover{color:#ee730a;text-decoration:underline}body.dark .lds-ring div{border-color:#999 transparent transparent}body.dark .font-red{color:#b20f03}body.dark"#,
    // 2024-04-09 PerimeterX / Human Security
    r#"{return l.onPageView}}),Object.defineProperty(r,"perimeterxIdentifiers",{enumerable:"#,
];

pub fn waf_hit(resp_text: &str) -> bool {
    WAFHIT_MSGS.iter().any(|msg| resp_text.contains(msg))
}
