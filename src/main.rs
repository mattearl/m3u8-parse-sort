use anyhow::Result;
use clap::Parser;
use m3u8_parse_sort::{
    fetch::fetch_playlist,
    sort::{get_sort_order, SortIFrameBy, SortMediaBy, SortStreamBy},
};
use std::io::stdout;
use tracing::{error, info};

#[derive(Parser)]
#[command(
    name = "HLS Playlist Sorter",
    about = "Sort an HLS playlist from a URL or file"
)]
pub struct Cli {
    #[arg(
        help = "The location of the playlist. Can be a file path or an HTTP URL.\nExamples:\n - /path/to/playlist.m3u8\n - http://example.com/playlist.m3u8"
    )]
    pub playlist_location: String,

    #[arg(
        short = 's',
        long,
        value_enum,
        value_delimiter = ',',
        help = "Sort the #EXT-X-STREAM-INF elements by primary and secondary attributes (format: primary,secondary)"
    )]
    pub sort_stream_by: Vec<SortStreamBy>,

    #[arg(
        short = 'm',
        long,
        value_enum,
        value_delimiter = ',',
        help = "Sort the #EXT-X-MEDIA elements by primary and secondary attributes (format: primary,secondary)"
    )]
    pub sort_media_by: Vec<SortMediaBy>,

    #[arg(
        short = 'i',
        long,
        value_enum,
        value_delimiter = ',',
        help = "Sort the #EXT-X-I-FRAME-STREAM-INF elements by primary and secondary attributes (format: primary,secondary)"
    )]
    pub sort_iframe_by: Vec<SortIFrameBy>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Cli::parse();

    info!("Fetching playlist from {}", &args.playlist_location);

    match fetch_playlist(&args.playlist_location).await {
        Ok(mut playlist) => {
            info!("Successfully fetched and parsed playlist.");

            // Sort the playlist based on the selected sorting criteria
            playlist.sort_stream(get_sort_order(&args.sort_stream_by));
            playlist.sort_media(get_sort_order(&args.sort_media_by));
            playlist.sort_iframe(get_sort_order(&args.sort_iframe_by));

            // Write the sorted playlist to stdout
            let stdout = stdout();
            let mut handle = stdout.lock();

            playlist.write_to(&mut handle)?;

            info!("Playlist successfully written to output.");
        }
        Err(err) => {
            error!("Failed to fetch or parse playlist: {:?}", err);
            return Err(err.into());
        }
    }

    Ok(())
}
