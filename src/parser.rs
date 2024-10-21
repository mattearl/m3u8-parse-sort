//! This file defines the structure and functionality for parsing and managing M3U8 master playlists.
//! It provides parsing utilities for various tags, including stream variants, media tracks, 
//! and I-frame streams, and allows serializing these structures back to a playlist format.
//! 
//! For more detailed documentation on the playlist format and the tags used, refer to:
//! https://datatracker.ietf.org/doc/html/rfc8216

use crate::errors::PlaylistError;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::opt,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, 
};
use nom::{error::Error as NomError, Err as NomErr};
use std::{
    fmt,
    io::{Result as IoResult, Write},
};

/// The Master Playlist defines the Variant Streams, Renditions, and
/// other global parameters of the presentation.
#[derive(Debug)]
pub struct MasterPlaylist {
    pub independent_segments: bool,
    pub variants: Vec<StreamVariant>,
    pub media: Vec<MediaTrack>,
    pub frames: Vec<IFrameStream>,
}

/// The EXT-X-STREAM-INF tag specifies a Variant Stream, which is a set
/// of Renditions that can be combined to play the presentation.  The
/// attributes of the tag provide information about the Variant Stream.
/// 
/// The URI line specifies a Media Playlist that carries a Rendition of
/// the Variant Stream.  
#[derive(Debug)]
pub struct StreamVariant {
    pub bandwidth: u32,
    pub average_bandwidth: Option<u32>,
    pub codecs: Option<String>,
    pub resolution: Option<(u32, u32)>,
    pub frame_rate: Option<f32>,
    pub video_range: Option<String>,
    pub audio: Option<String>,
    pub closed_captions: Option<String>,
    pub uri: String,
}

/// The EXT-X-MEDIA tag is used to relate Media Playlists that contain
/// alternative Renditions of the same content.  For example, three 
/// EXT-X-MEDIA tags can be used to identify audio-only Media Playlists
/// that contain English, French, and Spanish Renditions of the same
/// presentation.  Or, two EXT-X-MEDIA tags can be used to identify
/// video-only Media Playlists that show two different camera angles.
#[derive(Debug)]
pub struct MediaTrack {
    pub track_type: Option<String>,
    pub group_id: Option<String>,
    pub name: Option<String>,
    pub language: Option<String>,
    pub default: Option<String>,
    pub autoselect: Option<String>,
    pub channels: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug)]
pub struct IFrameStream {
    pub bandwidth: u32,
    pub codecs: Option<String>,
    pub resolution: Option<(u32, u32)>,
    pub video_range: Option<String>,
    pub uri: String,
}

impl fmt::Display for MediaTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(ref track_type) = self.track_type {
            parts.push(format!("TYPE={}", track_type));
        }
        if let Some(ref group_id) = self.group_id {
            parts.push(format!("GROUP-ID=\"{}\"", group_id));
        }
        if let Some(ref name) = self.name {
            parts.push(format!("NAME=\"{}\"", name));
        }
        if let Some(ref language) = self.language {
            parts.push(format!("LANGUAGE=\"{}\"", language));
        }
        if let Some(ref default) = self.default {
            parts.push(format!("DEFAULT={}", default));
        }
        if let Some(ref autoselect) = self.autoselect {
            parts.push(format!("AUTOSELECT={}", autoselect));
        }
        if let Some(ref channels) = self.channels {
            parts.push(format!("CHANNELS=\"{}\"", channels));
        }
        if let Some(ref uri) = self.uri {
            parts.push(format!("URI=\"{}\"", uri));
        }

        write!(f, "#EXT-X-MEDIA:{}", parts.join(","))
    }
}

impl fmt::Display for StreamVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        // Mandatory field
        parts.push(format!("BANDWIDTH={}", self.bandwidth));

        // Optional fields
        if let Some(average_bandwidth) = self.average_bandwidth {
            parts.push(format!("AVERAGE-BANDWIDTH={}", average_bandwidth));
        }
        if let Some(ref codecs) = self.codecs {
            parts.push(format!("CODECS=\"{}\"", codecs));
        }
        if let Some((width, height)) = self.resolution {
            parts.push(format!("RESOLUTION={}x{}", width, height));
        }
        if let Some(frame_rate) = self.frame_rate {
            parts.push(format!("FRAME-RATE={}", frame_rate));
        }
        if let Some(ref video_range) = self.video_range {
            parts.push(format!("VIDEO-RANGE={}", video_range));
        }
        if let Some(ref audio) = self.audio {
            parts.push(format!("AUDIO=\"{}\"", audio));
        }
        if let Some(ref closed_captions) = self.closed_captions {
            parts.push(format!("CLOSED-CAPTIONS={}", closed_captions));
        }

        // Write the #EXT-X-STREAM-INF line
        write!(f, "#EXT-X-STREAM-INF:{}\n{}", parts.join(","), self.uri)
    }
}

impl fmt::Display for IFrameStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        // Mandatory field: BANDWIDTH
        parts.push(format!("BANDWIDTH={}", self.bandwidth));

        // Optional fields
        if let Some(ref codecs) = self.codecs {
            parts.push(format!("CODECS=\"{}\"", codecs));
        }
        if let Some((width, height)) = self.resolution {
            parts.push(format!("RESOLUTION={}x{}", width, height));
        }
        if let Some(ref video_range) = self.video_range {
            parts.push(format!("VIDEO-RANGE={}", video_range));
        }

        // URI field (always present)
        parts.push(format!("URI=\"{}\"", self.uri));

        // Write the formatted string to the output
        write!(f, "#EXT-X-I-FRAME-STREAM-INF:{}", parts.join(","))
    }
}

impl MasterPlaylist {
    /// Writes the MasterPlaylist to any `Write` type (e.g., file, buffer)
    pub fn write_to<T: Write>(&self, w: &mut T) -> IoResult<()> {
        writeln!(w, "#EXTM3U")?;

        if self.independent_segments {
            writeln!(w, "#EXT-X-INDEPENDENT-SEGMENTS")?;
        }
        writeln!(w)?;

        // Write media tracks (EXT-X-MEDIA)
        for media in &self.media {
            writeln!(w, "{}", media)?; // Use the Display implementation of MediaTrack
        }
        writeln!(w)?;

        // Write stream variants (EXT-X-STREAM-INF)
        for variant in &self.variants {
            writeln!(w, "{}", variant)?; // Use the Display implementation of StreamVariant
        }
        writeln!(w)?;

        // Write I-frame streams (EXT-X-I-FRAME-STREAM-INF)
        for frame in &self.frames {
            writeln!(w, "{}", frame)?; // Use the Display implementation of IFrameStream
        }

        Ok(())
    }
}

/// Main function to parse the entire M3U8 playlist
pub fn parse_playlist(input: &str) -> Result<MasterPlaylist, PlaylistError> {
    let (_, playlist) = parse_master_playlist(input).map_err(|e| match e {
        NomErr::Incomplete(needed) => {
            PlaylistError::Incomplete(format!("Incomplete input, needed: {:?}", needed))
        }
        NomErr::Error(NomError { input, code }) | NomErr::Failure(NomError { input, code }) => {
            PlaylistError::ParseError(NomErr::Error(NomError {
                input: input.to_string(),
                code,
            }))
        }
    })?;
    Ok(playlist)
}

fn parse_master_playlist(input: &str) -> IResult<&str, MasterPlaylist> {
    let (input, _) = parse_extm3u(input)?; // Parse the #EXTM3U tag

    // Set independent_segments based on #EXT-X-INDEPENDENT-SEGMENTS presence
    let (mut input, independent_segments) = match opt(parse_ext_x_independent_segments)(input)? {
        (new_input, Some(_)) => (new_input, true),
        (new_input, None) => (new_input, false),
    };

    let mut variants = Vec::new();
    let mut media = Vec::new();
    let mut frames = Vec::new();

    // Loop through the input, parsing each tag dynamically
    while !input.is_empty() {
        if input.starts_with("#EXT-X-I-FRAME-STREAM-INF") {
            let (new_input, frame) = parse_iframe_stream(input)?;
            frames.push(frame);
            input = new_input;
        } else if input.starts_with("#EXT-X-STREAM-INF") {
            let (new_input, variant) = parse_stream_variant(input)?;
            variants.push(variant);
            input = new_input;
        } else if input.starts_with("#EXT-X-MEDIA") {
            let (new_input, track) = parse_media_track(input)?;
            media.push(track);
            input = new_input;
        } else {
            // Skip over any unrecognized or non-relevant tags or lines
            let (new_input, _) = not_line_ending(input)?;
            let (new_input, _) = line_ending(new_input)?;
            input = new_input;
        }
    }

    Ok((
        input,
        MasterPlaylist {
            independent_segments,
            variants,
            media,
            frames,
        },
    ))
}

fn parse_stream_variant(input: &str) -> IResult<&str, StreamVariant> {
    let (input, _) = tag("#EXT-X-STREAM-INF:")(input)?;

    // Parse until the end of the line, then handle key-value pairs
    let (input, key_value_section) = not_line_ending(input)?; // Capture the line without consuming the newline

    // Parse the key-value pairs from the line
    let (_, key_value_pairs) = separated_list1(
        tag(","),
        separated_pair(parse_key, tag("="), parse_quoted_or_unquoted_string),
    )(key_value_section)?;

    // Initialize the StreamVariant struct with default values
    let mut stream_variant = StreamVariant {
        bandwidth: 0,
        average_bandwidth: None,
        codecs: None,
        resolution: None,
        frame_rate: None,
        video_range: None,
        audio: None,
        closed_captions: None,
        uri: String::new(),
    };

    // Iterate over the key-value pairs and populate the struct
    for (key, value) in key_value_pairs {
        match key.as_str() {
            "BANDWIDTH" => stream_variant.bandwidth = value.parse().unwrap_or(0),
            "AVERAGE-BANDWIDTH" => {
                stream_variant.average_bandwidth = Some(value.parse().unwrap_or(0))
            }
            "CODECS" => stream_variant.codecs = Some(value),
            "RESOLUTION" => {
                let res_parts: Vec<&str> = value.split('x').collect();
                if res_parts.len() == 2 {
                    if let (Ok(width), Ok(height)) = (res_parts[0].parse(), res_parts[1].parse()) {
                        stream_variant.resolution = Some((width, height));
                    }
                }
            }
            "FRAME-RATE" => stream_variant.frame_rate = Some(value.parse().unwrap_or(0.0)),
            "VIDEO-RANGE" => stream_variant.video_range = Some(value),
            "AUDIO" => stream_variant.audio = Some(value),
            "CLOSED-CAPTIONS" => stream_variant.closed_captions = Some(value),
            _ => {}
        }
    }

    // Now parse the URI, which comes after the key-value section and a newline
    let (input, _) = line_ending(input)?; // Consume the newline
    let (input, uri) = parse_uri(input)?; // Parse the URI

    stream_variant.uri = uri;

    Ok((input, stream_variant))
}

fn parse_media_track(input: &str) -> IResult<&str, MediaTrack> {
    let (input, _) = tag("#EXT-X-MEDIA:")(input)?;

    // Split the input into key-value pairs by commas
    let (input, key_value_pairs) = separated_list1(
        tag(","),
        separated_pair(parse_key, tag("="), parse_quoted_or_unquoted_string),
    )(input)?;

    // Accumulate the key-value pairs into a MediaTrack struct
    let mut track = MediaTrack {
        track_type: None,
        group_id: None,
        name: None,
        language: None,
        default: None,
        autoselect: None,
        channels: None,
        uri: None,
    };

    for (key, value) in key_value_pairs {
        match key.as_str() {
            "TYPE" => track.track_type = Some(value),
            "GROUP-ID" => track.group_id = Some(value),
            "NAME" => track.name = Some(value),
            "LANGUAGE" => track.language = Some(value),
            "DEFAULT" => track.default = Some(value),
            "AUTOSELECT" => track.autoselect = Some(value),
            "CHANNELS" => track.channels = Some(value),
            "URI" => track.uri = Some(value),
            _ => {} // Ignore unknown fields
        }
    }

    Ok((input, track))
}

fn parse_iframe_stream(input: &str) -> IResult<&str, IFrameStream> {
    let (input, _) = tag("#EXT-X-I-FRAME-STREAM-INF:")(input)?;

    // Parse until the end of the line, then handle key-value pairs
    let (input, key_value_section) = not_line_ending(input)?; // Capture the line without consuming the newline

    // Parse the key-value pairs from the line
    let (_, key_value_pairs) = separated_list1(
        tag(","),
        separated_pair(parse_key, tag("="), parse_quoted_or_unquoted_string),
    )(key_value_section)?;

    // Initialize the IFrameStream struct with default values
    let mut iframe_stream = IFrameStream {
        bandwidth: 0,
        codecs: None,
        resolution: None,
        video_range: None,
        uri: String::new(),
    };

    // Iterate over the key-value pairs and populate the struct
    for (key, value) in key_value_pairs {
        match key.as_str() {
            "BANDWIDTH" => iframe_stream.bandwidth = value.parse().unwrap_or(0),
            "CODECS" => iframe_stream.codecs = Some(value),
            "RESOLUTION" => {
                let res_parts: Vec<&str> = value.split('x').collect();
                if res_parts.len() == 2 {
                    if let (Ok(width), Ok(height)) = (res_parts[0].parse(), res_parts[1].parse()) {
                        iframe_stream.resolution = Some((width, height));
                    }
                }
            }
            "VIDEO-RANGE" => iframe_stream.video_range = Some(value),
            "URI" => iframe_stream.uri = value,
            _ => {}
        }
    }

    Ok((input, iframe_stream))
}

fn parse_extm3u(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("#EXTM3U")(input)?;
    let (input, _) = line_ending(input)?; // Consume the newline after the tag
    Ok((input, ()))
}

fn parse_ext_x_independent_segments(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("#EXT-X-INDEPENDENT-SEGMENTS")(input)?;
    let (input, _) = line_ending(input)?; // Consume the newline after the tag
    Ok((input, ()))
}

fn parse_uri(input: &str) -> IResult<&str, String> {
    let (input, uri) = not_line_ending(input)?;
    Ok((input, uri.to_string()))
}

/// Helper function to parse the key part of a key-value pair
fn parse_key(input: &str) -> IResult<&str, String> {
    let (input, key) =
        nom::bytes::complete::take_while1(|c: char| c.is_alphanumeric() || c == '-')(input)?;
    Ok((input, key.to_string()))
}

/// Helper function to parse either quoted or unquoted strings
fn parse_quoted_or_unquoted_string(input: &str) -> IResult<&str, String> {
    if input.starts_with('"') {
        parse_quoted_string(input)
    } else {
        parse_unquoted_string(input)
    }
}

/// Parse quoted strings (surrounded by double quotes)
fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("\"")(input)?;
    let (input, value) = nom::bytes::complete::is_not("\"")(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, value.to_string()))
}

/// Parse unquoted strings (no quotes around them)
fn parse_unquoted_string(input: &str) -> IResult<&str, String> {
    // Parse any string until a comma or end of input
    let (input, value) = nom::bytes::complete::is_not(",")(input)?;
    Ok((input, value.trim().to_string())) // Trim any surrounding whitespace
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};
    use super::*;

    #[test]
    fn test_parse_media_track_round_trip() {
        let input = "#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac-128k\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"2\",URI=\"audio/unenc/aac_128k/vod.m3u8\"\n";
        round_trip_test(input, parse_media_track);
    }

    #[test]
    fn test_parse_stream_variant_round_trip() {
        let input = "#EXT-X-STREAM-INF:BANDWIDTH=2483789,AVERAGE-BANDWIDTH=1762745,CODECS=\"mp4a.40.2,hvc1.2.4.L90.90\",RESOLUTION=960x540,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/1650k/vod.m3u8";
        round_trip_test(input, parse_stream_variant);
    }

    #[test]
    fn test_parse_iframe_stream_round_trip() {
        let input = "#EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=222552,CODECS=\"hvc1.2.4.L93.90\",RESOLUTION=1280x720,VIDEO-RANGE=PQ,URI=\"hdr10/unenc/3300k/vod-iframe.m3u8\"";
        round_trip_test(input, parse_iframe_stream);
    }

    fn round_trip_test<T, F>(input: &str, parser: F)
    where
        T: std::fmt::Display + std::fmt::Debug,
        F: Fn(&str) -> IResult<&str, T>,
    {
        // Step 1: Parse the input string
        let result = parser(input);
        assert!(
            result.is_ok(),
            "Expected successful parsing but got error: {:?}",
            result
        );

        let (_, parsed_object) = result.unwrap();

        // Step 2: Serialize the parsed object back to a string using the Display trait
        let serialized_output = format!("{}", parsed_object);

        // Step 3: Ensure the serialized string matches the original input (without the newline)
        assert_eq!(
            serialized_output.trim(),
            input.trim(),
            "Expected serialized output to match the input, but got:\n{}",
            serialized_output
        );
    }

    fn get_test_files() -> Vec<&'static str> {
        vec!["parse_test.m3u8"]
    }

    #[test]
    fn test_multiple_playlists_round_trip() {
        for file_name in get_test_files() {
            // Step 1: Read each file
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("tests/data");
            file_path.push(file_name);

            let input = fs::read_to_string(file_path).expect("Failed to read test file");

            // Step 2: Parse the playlist
            let result = parse_playlist(&input);
            assert!(
                result.is_ok(),
                "Expected successful parsing of {} but got error: {:?}",
                file_name,
                result
            );

            let playlist = result.unwrap();

            // Step 3: Serialize the MasterPlaylist back to a string using write_to
            let mut serialized_output = Vec::new();
            playlist
                .write_to(&mut serialized_output)
                .expect("Failed to serialize playlist");

            // Convert serialized output to a string
            let serialized_str = String::from_utf8(serialized_output)
                .expect("Failed to convert serialized output to string");

            // Step 4: Ensure the serialized string matches the original input (ignoring differences like newlines)
            assert_eq!(
                serialized_str.trim(),
                input.trim(),
                "Expected serialized output to match the input for {}, but got:\n{}",
                file_name,
                serialized_str
            );
        }
    }
}
