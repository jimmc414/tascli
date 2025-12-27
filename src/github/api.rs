use std::process::Command;

use serde_json::Value;

/// Parsed GitHub issue reference (owner/repo#number)
#[derive(Debug, Clone)]
pub struct IssueRef {
    pub owner: String,
    pub repo: String,
    pub number: u32,
}

impl IssueRef {
    /// Format as "owner/repo#number"
    pub fn to_string(&self) -> String {
        format!("{}/{}#{}", self.owner, self.repo, self.number)
    }
}

/// GitHub issue data from gh CLI
#[derive(Debug, Clone)]
pub struct GitHubIssue {
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub url: String,
}

/// Parse "owner/repo#42" format into components
pub fn parse_issue_ref(s: &str) -> Result<IssueRef, String> {
    // Find the '#' separator
    let hash_pos = s.rfind('#').ok_or_else(|| {
        format!(
            "Invalid issue reference '{}'. Use format: owner/repo#number",
            s
        )
    })?;

    // Parse the number after '#'
    let number_str = &s[hash_pos + 1..];
    let number: u32 = number_str.parse().map_err(|_| {
        format!(
            "Invalid issue number '{}'. Use format: owner/repo#number",
            number_str
        )
    })?;

    // Parse owner/repo before '#'
    let owner_repo = &s[..hash_pos];
    let slash_pos = owner_repo.find('/').ok_or_else(|| {
        format!(
            "Invalid issue reference '{}'. Use format: owner/repo#number",
            s
        )
    })?;

    let owner = &owner_repo[..slash_pos];
    let repo = &owner_repo[slash_pos + 1..];

    if owner.is_empty() || repo.is_empty() {
        return Err(format!(
            "Invalid issue reference '{}'. Use format: owner/repo#number",
            s
        ));
    }

    Ok(IssueRef {
        owner: owner.to_string(),
        repo: repo.to_string(),
        number,
    })
}

/// Check if gh CLI is available and authenticated
pub fn is_gh_available() -> bool {
    Command::new("gh")
        .args(["auth", "status"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Fetch issue details via gh CLI
pub fn get_issue(issue_ref: &IssueRef) -> Result<GitHubIssue, String> {
    let issue_arg = issue_ref.to_string();

    let output = Command::new("gh")
        .args(["issue", "view", &issue_arg, "--json", "title,body,state,url"])
        .output()
        .map_err(|e| format!("Failed to run gh: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("GitHub CLI error: {}", stderr.trim()));
    }

    // Parse JSON response
    let json: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse gh output: {}", e))?;

    Ok(GitHubIssue {
        title: json["title"].as_str().unwrap_or("").to_string(),
        body: json["body"].as_str().map(|s| s.to_string()),
        state: json["state"].as_str().unwrap_or("").to_string(),
        url: json["url"].as_str().unwrap_or("").to_string(),
    })
}

/// Close a GitHub issue via gh CLI
pub fn close_issue(issue_ref: &IssueRef) -> Result<(), String> {
    let issue_arg = issue_ref.to_string();

    let output = Command::new("gh")
        .args(["issue", "close", &issue_arg])
        .output()
        .map_err(|e| format!("Failed to run gh: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("GitHub CLI error: {}", stderr.trim()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_issue_ref_valid() {
        let r = parse_issue_ref("owner/repo#42").unwrap();
        assert_eq!(r.owner, "owner");
        assert_eq!(r.repo, "repo");
        assert_eq!(r.number, 42);
    }

    #[test]
    fn test_parse_issue_ref_with_dashes() {
        let r = parse_issue_ref("my-org/my-repo#123").unwrap();
        assert_eq!(r.owner, "my-org");
        assert_eq!(r.repo, "my-repo");
        assert_eq!(r.number, 123);
    }

    #[test]
    fn test_parse_issue_ref_with_underscores() {
        let r = parse_issue_ref("my_org/my_repo#999").unwrap();
        assert_eq!(r.owner, "my_org");
        assert_eq!(r.repo, "my_repo");
        assert_eq!(r.number, 999);
    }

    #[test]
    fn test_parse_issue_ref_large_number() {
        let r = parse_issue_ref("owner/repo#12345").unwrap();
        assert_eq!(r.number, 12345);
    }

    #[test]
    fn test_parse_issue_ref_invalid_no_hash() {
        assert!(parse_issue_ref("owner/repo").is_err());
    }

    #[test]
    fn test_parse_issue_ref_invalid_no_slash() {
        assert!(parse_issue_ref("owner#42").is_err());
    }

    #[test]
    fn test_parse_issue_ref_invalid_just_hash() {
        assert!(parse_issue_ref("#42").is_err());
    }

    #[test]
    fn test_parse_issue_ref_invalid_empty_parts() {
        assert!(parse_issue_ref("/repo#42").is_err());
        assert!(parse_issue_ref("owner/#42").is_err());
    }

    #[test]
    fn test_parse_issue_ref_invalid_not_a_number() {
        assert!(parse_issue_ref("owner/repo#abc").is_err());
    }

    #[test]
    fn test_issue_ref_to_string() {
        let r = IssueRef {
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            number: 42,
        };
        assert_eq!(r.to_string(), "owner/repo#42");
    }
}
