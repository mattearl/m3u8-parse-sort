//! This module provides sorting functionalities for M3U8 Master Playlists,
//! allowing sorting of streams, media tracks, and I-frame streams by
//! various criteria. The sorting is done by primary and secondary sorting
//! criteria defined by enum variants.

use crate::parser::MasterPlaylist;
use crate::parser::{IFrameStream, MediaTrack, StreamVariant};

/// Specifies sorting criteria for stream variants in a playlist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug, Default)]
pub enum SortStreamBy {
    #[default]
    Bandwidth,
    AverageBandwidth,
    Codecs,
    Resolution,
    FrameRate,
    VideoRange,
    Audio,
    ClosedCaptions,
    Uri,
}

/// Specifies sorting criteria for media tracks in a playlist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug, Default)]
pub enum SortMediaBy {
    Type,
    #[default]
    GroupId,
    Name,
    Language,
    Default,
    AutoSelect,
    Channels,
    Uri,
}

/// Specifies sorting criteria for I-frame streams in a playlist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug, Default)]
pub enum SortIFrameBy {
    #[default]
    Bandwidth,
    Codecs,
    Resolution,
    VideoRange,
    Uri,
}

impl MasterPlaylist {
    /// Sorts the stream variants within the playlist using primary and
    /// secondary sorting criteria.
    pub fn sort_stream(&mut self, sort_by: (SortStreamBy, SortStreamBy)) {
        self.variants.sort_by(|a, b| {
            let primary_cmp = Self::compare_stream(a, b, sort_by.0);
            if primary_cmp == std::cmp::Ordering::Equal {
                Self::compare_stream(a, b, sort_by.1)
            } else {
                primary_cmp
            }
        });
    }

    /// Sorts the media tracks within the playlist using primary and
    /// secondary sorting criteria.
    pub fn sort_media(&mut self, sort_by: (SortMediaBy, SortMediaBy)) {
        self.media.sort_by(|a, b| {
            let primary_cmp = Self::compare_media(a, b, sort_by.0);
            if primary_cmp == std::cmp::Ordering::Equal {
                Self::compare_media(a, b, sort_by.1)
            } else {
                primary_cmp
            }
        });
    }

    /// Sorts the I-frame streams within the playlist using primary and
    /// secondary sorting criteria.
    pub fn sort_iframe(&mut self, sort_by: (SortIFrameBy, SortIFrameBy)) {
        self.frames.sort_by(|a, b| {
            let primary_cmp = Self::compare_iframe(a, b, sort_by.0);
            if primary_cmp == std::cmp::Ordering::Equal {
                Self::compare_iframe(a, b, sort_by.1)
            } else {
                primary_cmp
            }
        });
    }

    /// Comparison logic for stream variants.
    fn compare_stream(
        a: &StreamVariant,
        b: &StreamVariant,
        sort_by: SortStreamBy,
    ) -> std::cmp::Ordering {
        match sort_by {
            SortStreamBy::Bandwidth => a.bandwidth.cmp(&b.bandwidth),
            SortStreamBy::AverageBandwidth => a.average_bandwidth.cmp(&b.average_bandwidth),
            SortStreamBy::Resolution => a.resolution.cmp(&b.resolution),
            SortStreamBy::Uri => a.uri.cmp(&b.uri),
            SortStreamBy::Codecs => a.codecs.cmp(&b.codecs),
            SortStreamBy::VideoRange => a.video_range.cmp(&b.video_range),
            SortStreamBy::Audio => a.audio.cmp(&b.audio),
            SortStreamBy::ClosedCaptions => a.closed_captions.cmp(&b.closed_captions),
            SortStreamBy::FrameRate => a
                .frame_rate
                .partial_cmp(&b.frame_rate)
                .unwrap_or(std::cmp::Ordering::Equal),
        }
    }

    /// Comparison logic for media tracks.
    fn compare_media(a: &MediaTrack, b: &MediaTrack, sort_by: SortMediaBy) -> std::cmp::Ordering {
        match sort_by {
            SortMediaBy::Type => a.track_type.cmp(&b.track_type),
            SortMediaBy::GroupId => a.group_id.cmp(&b.group_id),
            SortMediaBy::Name => a.name.cmp(&b.name),
            SortMediaBy::Language => a.language.cmp(&b.language),
            SortMediaBy::Default => a.default.cmp(&b.default),
            SortMediaBy::AutoSelect => a.autoselect.cmp(&b.autoselect),
            SortMediaBy::Channels => a.channels.cmp(&b.channels),
            SortMediaBy::Uri => a.uri.cmp(&b.uri),
        }
    }

    /// Comparison logic for I-frame streams.
    fn compare_iframe(
        a: &IFrameStream,
        b: &IFrameStream,
        sort_by: SortIFrameBy,
    ) -> std::cmp::Ordering {
        match sort_by {
            SortIFrameBy::Bandwidth => a.bandwidth.cmp(&b.bandwidth),
            SortIFrameBy::Resolution => a.resolution.cmp(&b.resolution),
            SortIFrameBy::Uri => a.uri.cmp(&b.uri),
            SortIFrameBy::Codecs => a.codecs.cmp(&b.codecs),
            SortIFrameBy::VideoRange => a.video_range.cmp(&b.video_range),
        }
    }
}

/// Helper function to get the primary and secondary sorting order.
///
/// # Parameters
/// - `sort_order`: A slice of sorting criteria.
///
/// # Returns
/// A tuple with the primary and secondary sorting criteria. Defaults are
/// used if not provided.
pub fn get_sort_order<T: Clone + Default>(sort_order: &[T]) -> (T, T) {
    let primary = sort_order.first().cloned().unwrap_or_default();
    let secondary = sort_order.get(1).cloned().unwrap_or_default();
    (primary, secondary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_playlist;
    use std::{fs, path::PathBuf};

    #[test]
    fn test_sorted_playlist_by_audio_then_bandwidth() {
        test_sort_playlist(
            "master_unenc_hdr10_all.m3u8",
            "expected_sorted_by_audio_then_bandwidth.m3u8",
            (SortStreamBy::Audio, SortStreamBy::Bandwidth),
            (SortMediaBy::GroupId, SortMediaBy::Channels),
            (SortIFrameBy::Bandwidth, SortIFrameBy::Resolution),
        );
    }

    #[test]
    fn test_sorted_playlist_by_resolution_then_average_bandwidth() {
        test_sort_playlist(
            "master_unenc_hdr10_all.m3u8",
            "expected_sorted_by_resolution_then_average_bandwidth.m3u8",
            (SortStreamBy::Resolution, SortStreamBy::AverageBandwidth),
            (SortMediaBy::Channels, SortMediaBy::GroupId),
            (SortIFrameBy::Resolution, SortIFrameBy::Bandwidth),
        );
    }

    #[test]
    fn test_sorted_chaos_playlist() {
        test_sort_playlist(
            "chaos_parse_test.m3u8",
            "expected_chaos_parse_test.m3u8",
            (SortStreamBy::Bandwidth, SortStreamBy::Bandwidth),
            (SortMediaBy::GroupId, SortMediaBy::GroupId),
            (SortIFrameBy::Bandwidth, SortIFrameBy::Bandwidth),
        );
    }

    fn test_sort_playlist(
        input_file: &str,
        expected_file: &str,
        stream_sort_by: (SortStreamBy, SortStreamBy),
        media_sort_by: (SortMediaBy, SortMediaBy),
        iframe_sort_by: (SortIFrameBy, SortIFrameBy),
    ) {
        // Step 1: Read input file
        let mut input_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_file_path.push("tests/data");
        input_file_path.push(input_file);

        let input = fs::read_to_string(&input_file_path).expect("Failed to read test input file");

        // Step 2: Parse the playlist
        let result = parse_playlist(&input);
        assert!(
            result.is_ok(),
            "Expected successful parsing of {} but got error: {:?}",
            input_file,
            result
        );
        let mut playlist = result.unwrap();

        // Step 3: Apply sorting for streams, media, and iframes
        playlist.sort_stream(stream_sort_by);
        playlist.sort_media(media_sort_by);
        playlist.sort_iframe(iframe_sort_by);

        // Step 4: Serialize the sorted playlist back to a string using write_to
        let mut serialized_output = Vec::new();
        playlist
            .write_to(&mut serialized_output)
            .expect("Failed to serialize playlist");

        let serialized_str = String::from_utf8(serialized_output)
            .expect("Failed to convert serialized output to string");

        // Step 5: Read expected output file
        let mut expected_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        expected_file_path.push("tests/data");
        expected_file_path.push(expected_file);

        let expected_output =
            fs::read_to_string(&expected_file_path).expect("Failed to read expected output file");

        // Step 6: Compare sorted output with expected output (ignoring newlines and whitespace)
        assert_eq!(
            serialized_str.trim(),
            expected_output.trim(),
            "Expected sorted output to match the expected output for {}, but got:\n{}",
            input_file,
            serialized_str
        );
    }
}
