use chrono::NaiveDateTime;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct ImageMetadata {
    pub datetime: NaiveDateTime,
    pub camera_model: Option<String>,
}

pub fn read_image_metadata(path: &Path) -> Option<ImageMetadata> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let exif = exif::Reader::new().read_from_container(&mut reader).ok()?;

    let datetime = exif
        .get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
        .and_then(|f| {
            if let exif::Value::Ascii(ref v) = f.value {
                v.first()
                    .and_then(|s| std::str::from_utf8(s).ok())
                    .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S").ok())
            } else {
                None
            }
        })?;

    let camera_model = exif
        .get_field(exif::Tag::Model, exif::In::PRIMARY)
        .and_then(|f| {
            if let exif::Value::Ascii(ref v) = f.value {
                v.first()
                    .and_then(|s| std::str::from_utf8(s).ok())
                    .map(|s| s.trim_matches('\0').trim().to_string())
            } else {
                None
            }
        })
        .filter(|s| !s.is_empty());

    Some(ImageMetadata {
        datetime,
        camera_model,
    })
}
