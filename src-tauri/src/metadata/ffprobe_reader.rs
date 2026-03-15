use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
struct FfprobeOutput {
    #[serde(default)]
    format: FfprobeFormat,
    #[serde(default)]
    streams: Vec<FfprobeStream>,
}

#[derive(Deserialize, Default)]
struct FfprobeFormat {
    #[serde(default)]
    tags: FfprobeTags,
}

#[derive(Deserialize)]
struct FfprobeStream {
    #[serde(default)]
    tags: FfprobeTags,
}

#[derive(Deserialize, Default)]
struct FfprobeTags {
    creation_time: Option<String>,
    #[serde(rename = "com.apple.quicktime.creationdate")]
    apple_creation_date: Option<String>,
}

/// Parse "2025-09-06T13:19:12+0200" — local time with offset.
/// Returns the displayed local time directly (no conversion).
fn parse_with_offset(s: &str) -> Option<NaiveDateTime> {
    // RFC3339: "+02:00"
    if let Ok(dt) = DateTime::<FixedOffset>::parse_from_rfc3339(s) {
        return Some(dt.naive_local());
    }
    // iPhone format: "+0200" (no colon)
    if let Ok(dt) = DateTime::<FixedOffset>::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%z") {
        return Some(dt.naive_local());
    }
    if let Ok(dt) = DateTime::<FixedOffset>::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%z") {
        return Some(dt.naive_local());
    }
    None
}

/// Parse "2025-09-06T11:19:12.000000Z" — UTC, convert to local system time.
fn parse_utc_to_local(s: &str) -> Option<NaiveDateTime> {
    let dt = DateTime::<FixedOffset>::parse_from_rfc3339(s).ok()?;
    Some(dt.with_timezone(&Local).naive_local())
}

pub fn read_video_datetime(path: &Path) -> Option<NaiveDateTime> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            path.to_str()?,
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let parsed: FfprobeOutput = serde_json::from_slice(&output.stdout).ok()?;

    // Priority 1: com.apple.quicktime.creationdate (local time + offset, use directly)
    if let Some(ref s) = parsed.format.tags.apple_creation_date {
        if let Some(dt) = parse_with_offset(s) {
            return Some(dt);
        }
    }

    // Priority 2: creation_time from format.tags (UTC → local)
    if let Some(ref s) = parsed.format.tags.creation_time {
        if let Some(dt) = parse_utc_to_local(s) {
            return Some(dt);
        }
    }

    // Priority 3: creation_time from streams[0].tags (UTC → local)
    if let Some(stream) = parsed.streams.first() {
        if let Some(ref s) = stream.tags.creation_time {
            if let Some(dt) = parse_utc_to_local(s) {
                return Some(dt);
            }
        }
    }

    None
}

pub fn ffprobe_available() -> bool {
    Command::new("ffprobe")
        .args(["-version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
