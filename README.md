# HLS Playlist Parse and Sorter

## Overview

The **HLS Playlist Parse and Sorter** is a command-line application and library written in Rust. It provides functionality to fetch, parse, and sort M3U8 master playlists. The application allows you to sort streams, media tracks, and I-frame streams by multiple criteria, such as bandwidth, resolution, codecs, and more.

### Key Features

- Fetch playlists from URLs or local file paths.
- Parse M3U8 master playlists into a structured data format.
- Sort playlists by various attributes such as bandwidth, resolution, and codecs.
- Serialize sorted playlists back to M3U8 format.
- Async operations using the `tokio` runtime.
  
The underlying functionality is also exposed as a library, making it easy to integrate into other projects.

## CLI Usage

The **HLS Playlist Parse and Sorter** allows sorting HLS playlists directly from the command line. It supports sorting streams, media, and I-frame streams by primary and secondary attributes.

### CLI Help

```sh
Sort an HLS playlist from a URL or file

Usage: m3u8-parse-sort [OPTIONS] <PLAYLIST_LOCATION>

Arguments:
  <PLAYLIST_LOCATION>  The location of the playlist. Can be a file path or an HTTP URL.
                       Examples:
                        - /path/to/playlist.m3u8
                        - http://example.com/playlist.m3u8

Options:
  -s, --sort-stream-by <SORT_STREAM_BY>
          Sort the #EXT-X-STREAM-INF elements by primary and secondary attributes (format: primary,secondary) [possible values: bandwidth, average-bandwidth, codecs, resolution, frame-rate, video-range, audio, closed-captions, uri]
  -m, --sort-media-by <SORT_MEDIA_BY>
          Sort the #EXT-X-MEDIA elements by primary and secondary attributes (format: primary,secondary) [possible values: type, group-id, name, language, default, auto-select, channels, uri]
  -i, --sort-iframe-by <SORT_IFRAME_BY>
          Sort the #EXT-X-I-FRAME-STREAM-INF elements by primary and secondary attributes (format: primary,secondary) [possible values: bandwidth, codecs, resolution, video-range, uri]
  -h, --help
          Print help
```

## Examples

### Sorting a Playlist by Bandwidth and Resolution

To fetch a playlist from a URL and sort the streams by bandwidth and resolution:

```sh
m3u8-parse-sort http://example.com/playlist.m3u8 --sort-stream-by bandwidth,resolution
```

### Sorting a Local Playlist by Audio and Bandwidth

To sort a playlist from a local file by audio and bandwidth:

```sh
m3u8-parse-sort /path/to/playlist.m3u8 --sort-stream-by audio,bandwidth
```

### Sorting Media Tracks by Group ID and Channels

```sh
m3u8-parse-sort /path/to/playlist.m3u8 --sort-media-by group-id,channels
```

### Sorting I-frame Streams by Bandwidth and Resolution

```sh
m3u8-parse-sort /path/to/playlist.m3u8 --sort-iframe-by bandwidth,resolution
```

## Building the Project

To build the project, you will need to have Rust installed. You can follow the instructions [here](https://www.rust-lang.org/tools/install) to install Rust.

Once Rust is installed, you can build the project using `cargo`:

```sh
cargo build --release
```

This will create an optimized executable in the `target/release/` directory.

### Running Tests

To run the tests, use the following command:

```sh
cargo test
```

This will run all the unit tests for the library and CLI application.

## Library Usage

You can also use the HLS Playlist Sorter as a library in your own Rust project.

```rust
use m3u8_parse_sort::{
    fetch::fetch_playlist,
    sort::{get_sort_order, SortStreamBy}
};
use tokio;

#[tokio::main]
async fn main() {
    let playlist_url = "http://example.com/playlist.m3u8";
    match fetch_playlist(playlist_url).await {
        Ok(mut playlist) => {
            // Sort streams by bandwidth and resolution
            let sort_order = get_sort_order(&[SortStreamBy::Bandwidth, SortStreamBy::Resolution]);
            playlist.sort_stream(sort_order);

            println!("Playlist sorted successfully!");
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
```
