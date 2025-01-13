use serenity::async_trait;
use serenity::http::Http;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use tokio::time::{interval, Duration};

#[group]
#[commands(ping)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
struct GitHubRepo {
    name: String,
    owner: Owner,
    description: Option<String>,
    stargazers_count: u64,
}

#[derive(Deserialize, Serialize)]
struct Owner {
    login: String,
}

#[derive(Deserialize, Serialize)]
struct GitHubCommit {
    sha: String,
    commit: CommitDetails,
}

#[derive(Deserialize, Serialize)]
struct CommitDetails {
    author: CommitAuthor,
    message: String,
}

#[derive(Deserialize, Serialize)]
struct CommitAuthor {
    name: String,
    date: String,
}

struct Handler {
    repo_url: Arc<Mutex<String>>,
    last_commit_sha: Arc<Mutex<Option<String>>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!setrepo ") {
            let new_url = msg.content["!setrepo ".len()..].trim().to_string();
            let mut repo_url = self.repo_url.lock().await;
            *repo_url = new_url.clone();

            msg.channel_id
                .say(&ctx.http, format!("Repository URL set to: {}", new_url))
                .await
                .ok();
        }

        if msg.content == "!github" {
            let repo_url = self.repo_url.lock().await.clone();
            let github_token = env::var("GITHUB_TOKEN").expect("GitHub token not found");

            match fetch_repo_info(&repo_url, &github_token).await {
                Ok(response) => {
                    msg.channel_id.say(&ctx.http, response).await.ok();
                }
                Err(_) => {
                    msg.channel_id
                        .say(&ctx.http, "Failed to fetch repository info.")
                        .await
                        .ok();
                }
            };
        }

        if msg.content == "!commit" {
            let repo_url = self.repo_url.lock().await.clone();
            let github_token = env::var("GITHUB_TOKEN").expect("GitHub token not found");

            match fetch_latest_commit(&repo_url, &github_token).await {
                Ok((Some(commit_sha), author, date, message)) => {
                    let response = format!(
                        "Latest commit by {} at {}:\n{}",
                        author, date, message
                    );
                    msg.channel_id.say(&ctx.http, response).await.ok();
                }
                Ok(_) => {
                    msg.channel_id.say(&ctx.http, "No commits found.").await.ok();
                }
                Err(_) => {
                    msg.channel_id
                        .say(&ctx.http, "Failed to fetch commits.")
                        .await
                        .ok();
                }
            };
        }
    }
}

async fn monitor_commits(
    http: Arc<Http>,
    repo_url: Arc<Mutex<String>>,
    last_commit_sha: Arc<Mutex<Option<String>>>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    let github_token = env::var("GITHUB_TOKEN").expect("GitHub token not found");
    let channel_id: u64 = env::var("DISCORD_CHANNEL_ID")
        .expect("Channel ID not found")
        .parse()
        .expect("Invalid Channel ID");

    let channel_id = ChannelId(channel_id);

    loop {
        interval.tick().await;

        let repo_url = repo_url.lock().await.clone();
        match fetch_latest_commit(&repo_url, &github_token).await {
            Ok((Some(commit_sha), author, date, message)) => {
                let mut last_sha = last_commit_sha.lock().await;

                if Some(commit_sha.clone()) != *last_sha {
                    *last_sha = Some(commit_sha.clone());

                    let formatted_message = format!(
                        "New commit by {} at {}:\n{}",
                        author, date, message
                    );

                    if let Err(e) = channel_id.say(&http, formatted_message).await {
                        eprintln!("Failed to send commit notification: {:?}", e);
                    }
                }
            }
            Ok(_) => eprintln!("No new commits found."),
            Err(e) => eprintln!("Failed to fetch latest commit: {:?}", e),
        }
    }
}

async fn fetch_repo_info(repo_url: &str, github_token: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res: GitHubRepo = client
        .get(repo_url)
        .header("User-Agent", "octopull")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?
        .json()
        .await?;

    Ok(format!(
        "Repo: {}\nDescription: {}",
        res.name,
        res.description.unwrap_or_else(|| "No description".to_string())
    ))
}

async fn fetch_latest_commit(
    repo_url: &str,
    github_token: &str,
) -> Result<(Option<String>, String, String, String), reqwest::Error> {
    let commit_url = format!("{}/commits", repo_url);
    let client = reqwest::Client::new();
    let res: Vec<GitHubCommit> = client
        .get(&commit_url)
        .header("User-Agent", "octopull")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?
        .json()
        .await?;

    if let Some(commit) = res.first() {
        Ok((
            Some(commit.sha.clone()),
            commit.commit.author.name.clone(),
            commit.commit.author.date.clone(),
            commit.commit.message.clone(),
        ))
    } else {
        Ok((None, "No author".to_string(), "No date".to_string(), "No message".to_string()))
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Token not found");
    let initial_repo_url = env::var("REPO_URL").unwrap_or_else(|_| "https://api.github.com/repos/owner/repo_name".to_string());

    let repo_url = Arc::new(Mutex::new(initial_repo_url));
    let last_commit_sha = Arc::new(Mutex::new(None));

    let handler = Handler {
        repo_url: Arc::clone(&repo_url),
        last_commit_sha: Arc::clone(&last_commit_sha),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = serenity::Client::builder(&token, GatewayIntents::all())
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let http = client.cache_and_http.http.clone();
    tokio::spawn(monitor_commits(http, Arc::clone(&repo_url), Arc::clone(&last_commit_sha)));

    if let Err(e) = client.start().await {
        eprintln!("Error starting the bot: {:?}", e);
    }
}
