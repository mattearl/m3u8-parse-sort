//! # Playlist Parse And Sort Library
//!
//! This library provides functionality to fetch, parse, and sort M3U8 master playlists.
//! It includes support for fetching playlists from both HTTP/HTTPS URLs and local file paths,
//! parsing them into a `MasterPlaylist` structure, and sorting stream variants, media tracks,
//! and I-frame streams by various criteria.
//!
//! The library supports asynchronous operations using the `tokio` runtime and includes error
//! handling using custom error types.
//!
//! ## Features
//!
//! - Fetch playlists from URLs or local file paths.
//! - Parse M3U8 master playlists into structured data (`MasterPlaylist`).
//! - Sort streams, media tracks, and I-frame streams by multiple criteria such as bandwidth, resolution, and codecs.
//! - Serialize sorted playlists back into M3U8 format.
//!
//! ## Examples
//!
//! ### Fetching a Playlist
//!
//! ```rust
//! use m3u8_parse_sort::parser::MasterPlaylist;
//! use m3u8_parse_sort::errors::PlaylistError;
//! use m3u8_parse_sort::fetch::fetch_playlist;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), PlaylistError> {
//!     let location = "tests/data/master_unenc_hdr10_all.m3u8";
//!     let playlist = fetch_playlist(location).await?;
//!     println!("Fetched playlist with {} streams", playlist.variants.len());
//!     Ok(())
//! }
//! ```
//!
//! In this example, a playlist is fetched from the specified URL and parsed into a `MasterPlaylist` structure.
//!
//! ### Sorting a Playlist by Bandwidth and Resolution
//!
//! ```rust
//! use m3u8_parse_sort::parser::MasterPlaylist;
//! use m3u8_parse_sort::sort::{SortStreamBy, get_sort_order};
//!
//! fn sort_playlist_by_bandwidth_and_resolution(mut playlist: MasterPlaylist) {
//!     let sort_order = (SortStreamBy::Bandwidth, SortStreamBy::Resolution);
//!     playlist.sort_stream(sort_order);
//!     println!("Sorted playlist by bandwidth and resolution");
//! }
//! ```
//!
//! This example demonstrates how to sort streams in a playlist first by bandwidth, then by resolution.
//!
//! ### Fetching, Sorting, and Saving a Playlist
//!
//! ```rust
//! use m3u8_parse_sort::fetch::fetch_playlist;
//! use m3u8_parse_sort::sort::{get_sort_order, SortStreamBy};
//! use m3u8_parse_sort::errors::PlaylistError;
//! use std::fs::File;
//! use std::io::Write;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), PlaylistError> {
//!     // Fetch playlist
//!     let location = "tests/data/master_unenc_hdr10_all.m3u8";
//!     let mut playlist = fetch_playlist(location).await?;
//!
//!     // Sort playlist by bandwidth and codecs
//!     let sort_order = (SortStreamBy::Bandwidth, SortStreamBy::Codecs);
//!     playlist.sort_stream(sort_order);
//!
//!     // Save sorted playlist to a file
//!     let mut file = File::create("sorted_playlist.m3u8")?;
//!     playlist.write_to(&mut file)?;
//!
//!     println!("Playlist sorted and saved to sorted_playlist.m3u8");
//!     Ok(())
//! }
//! ```
//!
//! This example fetches a playlist, sorts it by bandwidth and codecs, and then saves the sorted playlist to a file.
//!
//! ## Modules
//!
//! - `fetch: Provides functionality for fetching and parsing playlists from URLs or local files.
//! - `sort`: Sorting functionalities for M3U8 master playlists by various criteria.
//! - `parser`: Defines the structures and functions used for parsing M3U8 master playlists.
//! - `errors`: Defines custom error types used throughout the library.

pub mod errors;
pub mod fetch;
pub mod parser;
pub mod sort;
