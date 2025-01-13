# Octopull Discord Bot

<p align="center">
  <img src="https://github.com/user-attachments/assets/2fe68f78-417e-476d-b326-2d717f5e8cce" alt="Octopull-Logo" width="245">
</p>



<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
  <img src="https://img.shields.io/badge/Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white">
  <img src="https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white">
</p>
<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.72.0-black?style=for-the-badge&logo=rust&logoColor=white">
  <img src="https://img.shields.io/badge/Serenity-0.10.11-5865F2?style=for-the-badge&logo=discord&logoColor=white">
  <img src="https://img.shields.io/badge/Serde-1.0.215-darkgreen?style=for-the-badge">
  <img src="https://img.shields.io/badge/Reqwest-0.11.14-purple?style=for-the-badge">
  <img src="https://img.shields.io/badge/Concurrency-Tokio-darkred?style=for-the-badge">
</p>

---

## 🚀 Overview

**Octopull** is a lightweight and efficient Discord bot written in **Rust**. Designed to integrate seamlessly with Discord servers, it provides a simple and robust way to monitor GitHub repositories in real-time, empowering teams to stay updated on the latest changes in their projects.

### 🌟 Features

- **📦 Repository Tracking**: Users can set and query the status of GitHub repositories.
- **🔔 Real-Time Notifications**: Automated alerts for new commits every 60 seconds.
- **💬 User Commands**:
  - `!setrepo <url>`: Define the repository to track.
  - `!github`: Display general information about the current repository.
  - `!commit`: Fetch and display the latest commit details.
- **🔄 Concurrent Monitoring**: Efficient asynchronous operations using Tokio for periodic checks without blocking user commands.

---

## 🛠️ Technologies & Libraries

### Languages
![Rust](https://img.shields.io/badge/Rust-1.72.0-black?style=for-the-badge&logo=rust&logoColor=white)

### Libraries
- **[Serenity](https://docs.rs/serenity/latest/serenity/)**: A framework for building Discord bots.
- **[Reqwest](https://docs.rs/reqwest/latest/reqwest/)**: For handling HTTP requests to GitHub's REST API.
- **[Serde](https://serde.rs/)**: Serialization and deserialization of JSON responses.
- **[Tokio](https://tokio.rs/)**: An asynchronous runtime for efficient concurrency.

### APIs
- **[GitHub REST API](https://docs.github.com/en/rest)**: Fetch repository and commit data.
- **[Discord API](https://discord.com/developers/docs/intro)**: Power the bot's interaction with servers.

---

## 🧪 How It Works

1. Users configure the repository URL via the `!setrepo` command.
2. The bot queries GitHub's API to fetch repository or commit information.
3. A background task checks the repository for updates every 60 seconds.
4. Notifications about new commits are automatically sent to a specified Discord channel.

---

## 📖 Setup Guide

### Prerequisites

- **Rust** (Latest stable version): [Install Rust](https://www.rust-lang.org/tools/install)
- A **Discord Developer Account**: Create a bot token from the [Discord Developer Portal](https://discord.com/developers/applications).
- A **GitHub Token**: Generate a personal access token with the `repo` scope from [GitHub](https://github.com/settings/tokens).

### Installation Steps

1. **Clone the Repository**
   ```sh
   git clone https://github.com/yourusername/octopull.git
   cd octopull
   ```

2. **Set Up Environment Variables**
   Create a `.env` file based on the provided `env.example`:
   ```env
   DISCORD_TOKEN=your_discord_bot_token_here
   DISCORD_CHANNEL_ID=your_discord_channel_id_here
   GITHUB_TOKEN=your_github_api_token_here
   REPO_URL=https://api.github.com/repos/your_user/your_repo
   ```

   - **DISCORD_TOKEN**: Obtain your bot token from the [Discord Developer Portal](https://discord.com/developers/applications).
   - **DISCORD_CHANNEL_ID**: Enable Developer Mode in Discord, right-click a channel, and copy its ID.
   - **GITHUB_TOKEN**: Generate from [GitHub Settings](https://github.com/settings/tokens).

3. **Build and Run the Bot**
   ```sh
   cargo build --release
   cargo run
   ```

4. **Invite the Bot to Your Server**
   Use the OAuth2 URL from the Discord Developer Portal to invite your bot:
   ```
   https://discord.com/api/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=BOT_PERMISSIONS&scope=bot
   ```

---

## 🧠 Commands

- **`!ping`**: Responds with "Pong!" to verify the bot is online.
- **`!setrepo <url>`**: Set the repository URL for monitoring.
- **`!github`**: Fetch general information about the current repository.
- **`!commit`**: Display the latest commit's details.

---

## ⏱️ Concurrency & Efficiency

The bot utilizes **Tokio** to:

- Manage asynchronous tasks, ensuring the bot remains responsive to user commands while running background checks.
- Periodically check the GitHub API every 60 seconds to detect new commits, without blocking other bot operations.

---

## 📜 License

[![License](https://img.shields.io/badge/License-BSD%203%20-darkred)](LICENSE)

This project is licensed under the BSD-3 Clause License. See the LICENSE file for details.

---

## 🔮 Contributing

Contributions are welcome! Follow these steps:

1. **Fork the Repository**
   ```sh
   git checkout -b feature/your-feature
   ```

2. **Implement Your Changes**

3. **Commit and Push**
   ```sh
   git push origin feature/your-feature
   ```

4. **Open a Pull Request**

---


### ⚗️ Future Enhancements

- **Webhook Integration**: Add support for GitHub webhooks for instant notifications.
- **Customizable Intervals**: Allow users to define the monitoring frequency.
- **Advanced Analytics**: Display repository statistics and trends over time.
