//! This module provides asynchronous functions to fetch and parse a playlist from a URL or a local file.
//! It supports fetching content from HTTP/HTTPS locations as well as reading playlists from local file paths.
//! The fetched content is parsed into a `MasterPlaylist` using a custom parser.

use crate::parser::MasterPlaylist;
use crate::{errors::PlaylistError, parser::parse_playlist};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::{error, info};

/// Async function to fetch and parse the playlist using the custom parser
pub async fn fetch_playlist(location: &str) -> Result<MasterPlaylist, PlaylistError> {
    info!("Fetching playlist from {}", location);

    let content = fetch_content(location).await?;
    let playlist = parse_playlist(&content)?;

    Ok(playlist)
}

/// Async helper function to fetch content from a URL or local file
async fn fetch_content(location: &str) -> Result<String, PlaylistError> {
    if location.starts_with("http://") || location.starts_with("https://") {
        info!("Fetching from URL: {}", location);
        let content = reqwest::get(location).await?.text().await?;
        Ok(content)
    } else if Path::new(location).exists() {
        info!("Reading from local file: {}", location);
        let mut file = File::open(location).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        Ok(content)
    } else {
        error!("Invalid location: {}", location);
        Err(PlaylistError::InvalidLocation)
    }
}
