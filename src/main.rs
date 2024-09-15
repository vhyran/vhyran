use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Repo {
    language: Option<String>,
}

#[derive(Deserialize)]
struct LangStats {
    name: String,
    count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve the GitHub token from the environment
    let token = env::var("GITHUB_TOKEN")?;

    // Define the GitHub API URL for fetching repos
    let repos_url = format!("https://api.github.com/users/xarcgit/repos");

    // Create an HTTP client
    let client = Client::new();

    // Fetch repository data
    let repos_response = client
        .get(&repos_url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "Rust")
        .send()
        .await?
        .json::<Vec<Repo>>()
        .await?;

    // Collect languages from repos
    let mut language_counts = std::collections::HashMap::new();
    for repo in repos_response {
        if let Some(lang) = repo.language {
            *language_counts.entry(lang).or_insert(0) += 1;
        }
    }

    // Find most used languages
    let mut lang_stats: Vec<LangStats> = language_counts
        .into_iter()
        .map(|(name, count)| LangStats { name, count })
        .collect();
    lang_stats.sort_by(|a, b| b.count.cmp(&a.count)); // Sort by count descending

    let most_used_languages = if !lang_stats.is_empty() {
        lang_stats.into_iter().map(|s| s.name).collect::<Vec<_>>().join(", ")
    } else {
        "None".to_string()
    };

    // Define the README.md path
    let readme_path = Path::new("README.md");
    
    // Read README.md file
    let mut readme_content = fs::read_to_string(readme_path)?;

    // Update README.md content
    readme_content = readme_content
        .replace("Total Repos:              0", &format!("Total Repos:              {}", get_total_repos().await?))
        .replace("Total Commits:            0", &format!("Total Commits:            {}", get_total_commits().await?))
        .replace("Current Streak:           0", &format!("Current Streak:           {}", get_current_streak().await?))
        .replace("Total PRs:                0", &format!("Total PRs:                {}", get_total_prs().await?))
        .replace("Repos Contributed To:     0", &format!("Repos Contributed To:     {}", get_repos_contributed_to().await?))
        .replace("Open-Source Projects:     0", &format!("Open-Source Projects:     {}", get_open_source_projects().await?))
        .replace("Most Used Languages:      None", &format!("Most Used Languages:      {}", most_used_languages));

    // Write updated content back to README.md
    fs::write(readme_path, readme_content)?;

    println!("README.md updated successfully");
    Ok(())
}

// Define these functions to fetch respective stats
async fn get_total_repos() -> Result<u32, Box<dyn std::error::Error>> { /* Implement fetch */ Ok(0) }
async fn get_total_commits() -> Result<u32, Box<dyn std::error::Error>> { /* Implement fetch */ Ok(0) }
async fn get_current_streak() -> Result<u32, Box<dyn std::error::Error>> { /* Implement fetch */ Ok(0) }
async fn get_total_prs() -> Result<u32, Box<dyn std::error::Error>> { /* Implement fetch */ Ok(0) }
async fn get_repos_contributed_to() -> Result<u32, Box<dyn std::error::Error>> { /* Implement fetch */ Ok(0) }
async fn get_open_source_projects() -> Result<u32, Box<dyn std::error::Error>> { /* Implement fetch */ Ok(0) }
