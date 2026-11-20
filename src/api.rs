use gloo_net::http::Request;

use crate::models::{CommitInfo, Repository};

pub async fn fetch_repositories(username: String) -> Result<Vec<Repository>, String> {
    // let github_api_access = std::env::var("GITHUB_API_ACCESS").expect("Failed to get GITHUB_API_ACCESS");
    let github_api_access = option_env!("GITHUB_API_ACCESS").ok_or_else(|| "GITHUB_API_ACCESS is not configured".to_string())?;

    if username.trim().is_empty() {
        return Ok(vec![]);
    }

    let url = format!(
        "https://api.github.com/users/{}/repos?per_page=100&sort=updated",
        username
    );

    let response = Request::get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", &format!("Bearer {}", github_api_access))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("GitHub API returned status: {}", response.status()));
    }

    response
        .json::<Vec<Repository>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_latest_commit(full_name: String) -> Result<CommitInfo, String> {
    let github_api_access = option_env!("GITHUB_API_ACCESS").ok_or_else(|| "GITHUB_API_ACCESS is not configured".to_string())?;

    let url = format!(
        "https://api.github.com/repos/{}/commits?per_page=1",
        full_name
    );

    let response = Request::get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", &format!("Bearer {}", github_api_access))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("GitHub API returned status: {}", response.status()));
    }

    let mut commits = response
        .json::<Vec<CommitInfo>>()
        .await
        .map_err(|e| e.to_string())?;

    commits.pop().ok_or_else(|| "No commits found".to_string())
}
