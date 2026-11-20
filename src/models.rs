
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u64,
    pub language: Option<String>,
    pub fork: bool,
}



#[derive(Debug, Clone, Deserialize)]
pub struct CommitInfo {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitDetails {
    pub message: String,
    pub author: Option<CommitAuthor>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub date: String,
}
