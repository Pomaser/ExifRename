use chrono::{DateTime, NaiveDateTime};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
struct FfprobeOutput {
    format: FfprobeFormat,
}

#[derive(Deserialize)]
struct FfprobeFormat {
    tags: Option<FfprobeTags>,
}

#[derive(Deserialize)]
struct FfprobeTags {
    creation_time: Option<String>,
    #[serde(rename = "com.apple.quicktime.creationdate")]
    apple_creation_date: Option<String>,
}

pub fn read_video_datetime(path: &Path) -> Option<NaiveDateTime> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_entries",
            "format_tags=creation_time,com.apple.quicktime.creationdate",
            path.to_str()?,
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let parsed: FfprobeOutput = serde_json::from_slice(&output.stdout).ok()?;
    let tags = parsed.format.tags?;

    // Try Apple-specific tag first (more accurate for iPhone videos)
    let time_str = tags.apple_creation_date.or(tags.creation_time)?;

    // Try RFC3339 parse
    if let Ok(dt) = DateTime::parse_from_rfc3339(&time_str) {
        return Some(dt.naive_local());
    }

    // Fallback: ffprobe sometimes outputs "YYYY-MM-DDTHH:MM:SS.000000Z"
    if let Ok(dt) = DateTime::parse_from_rfc3339(time_str.trim_end_matches('Z').trim()) {
        return Some(dt.naive_local());
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
