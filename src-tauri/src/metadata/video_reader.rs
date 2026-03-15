use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Seconds between Mac epoch (1904-01-01) and Unix epoch (1970-01-01)
const MAC_EPOCH_OFFSET: i64 = 2082844800;

pub fn read_video_datetime(path: &Path) -> Option<NaiveDateTime> {
    let mut file = File::open(path).ok()?;
    let file_size = file.seek(SeekFrom::End(0)).ok()?;
    file.seek(SeekFrom::Start(0)).ok()?;

    let (moov_offset, moov_size) = find_top_level_atom(&mut file, file_size, b"moov")?;

    // moov contains only metadata — cap read to 20 MB to be safe
    let read_size = moov_size.min(20 * 1024 * 1024) as usize;
    file.seek(SeekFrom::Start(moov_offset)).ok()?;
    let mut moov = vec![0u8; read_size];
    file.read_exact(&mut moov).ok()?;

    // Priority 1: com.apple.quicktime.creationdate (local time with offset — use as-is)
    if let Some(dt) = read_qt_creation_date(&moov) {
        return Some(dt);
    }

    // Priority 2: mvhd creation_time (Mac epoch → UTC → local)
    read_mvhd_time(&moov)
}

/// Scan top-level atoms in the file to find one by type.
/// Returns (data_offset, data_size) — offsets into the file, past the box header.
fn find_top_level_atom(file: &mut File, file_size: u64, atom_type: &[u8; 4]) -> Option<(u64, u64)> {
    let mut pos = 0u64;
    while pos + 8 <= file_size {
        file.seek(SeekFrom::Start(pos)).ok()?;
        let mut hdr = [0u8; 8];
        file.read_exact(&mut hdr).ok()?;

        let size32 = u32::from_be_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]) as u64;
        let btype = [hdr[4], hdr[5], hdr[6], hdr[7]];

        let (atom_size, hdr_size) = if size32 == 1 {
            // Extended 64-bit size
            if pos + 16 > file_size {
                break;
            }
            let mut ext = [0u8; 8];
            file.read_exact(&mut ext).ok()?;
            (u64::from_be_bytes(ext), 16u64)
        } else if size32 == 0 {
            // Atom extends to end of file
            (file_size - pos, 8u64)
        } else {
            (size32, 8u64)
        };

        if atom_size < hdr_size {
            break;
        }

        if &btype == atom_type {
            return Some((pos + hdr_size, atom_size - hdr_size));
        }

        pos = pos.saturating_add(atom_size);
    }
    None
}

/// Find a child atom within a byte slice (already past the parent's header).
/// Returns (data_start, data_size) as indices into `data`.
fn find_child_atom(data: &[u8], atom_type: &[u8; 4]) -> Option<(usize, usize)> {
    let mut pos = 0usize;
    while pos + 8 <= data.len() {
        let size32 = u32::from_be_bytes(data[pos..pos + 4].try_into().ok()?) as usize;
        let btype = &data[pos + 4..pos + 8];

        let (atom_size, hdr_size) = if size32 == 1 {
            if pos + 16 > data.len() {
                break;
            }
            let size64 = u64::from_be_bytes(data[pos + 8..pos + 16].try_into().ok()?) as usize;
            (size64, 16)
        } else if size32 == 0 {
            (data.len() - pos, 8)
        } else {
            (size32, 8)
        };

        if atom_size < hdr_size || pos + atom_size > data.len() {
            break;
        }

        if btype == atom_type {
            return Some((pos + hdr_size, atom_size - hdr_size));
        }

        pos += atom_size;
    }
    None
}

/// Read creation_time from mvhd (Mac epoch → UTC → system local time).
fn read_mvhd_time(moov: &[u8]) -> Option<NaiveDateTime> {
    let (start, size) = find_child_atom(moov, b"mvhd")?;
    let mvhd = moov.get(start..start + size)?;

    // mvhd is a FullBox: 1 byte version + 3 bytes flags
    let version = *mvhd.first()?;
    let mac_ts: u64 = if version == 1 {
        // 64-bit: version(1)+flags(3)+creation_time(8)
        if mvhd.len() < 12 {
            return None;
        }
        u64::from_be_bytes(mvhd[4..12].try_into().ok()?)
    } else {
        // 32-bit: version(1)+flags(3)+creation_time(4)
        if mvhd.len() < 8 {
            return None;
        }
        u32::from_be_bytes(mvhd[4..8].try_into().ok()?) as u64
    };

    let unix_ts = mac_ts as i64 - MAC_EPOCH_OFFSET;
    let dt_utc = Utc.timestamp_opt(unix_ts, 0).single()?;
    Some(dt_utc.with_timezone(&Local).naive_local())
}

/// Read com.apple.quicktime.creationdate from moov > meta > keys/ilst.
/// Returns the local time as stored (no UTC conversion).
fn read_qt_creation_date(moov: &[u8]) -> Option<NaiveDateTime> {
    let (meta_start, meta_size) = find_child_atom(moov, b"meta")?;
    let meta = moov.get(meta_start..meta_start + meta_size)?;

    // meta is a FullBox: skip 4 bytes (version + flags) before children
    if meta.len() < 4 {
        return None;
    }
    let meta_children = &meta[4..];

    let (keys_start, keys_size) = find_child_atom(meta_children, b"keys")?;
    let (ilst_start, ilst_size) = find_child_atom(meta_children, b"ilst")?;

    let keys_data = meta_children.get(keys_start..keys_start + keys_size)?;
    let ilst_data = meta_children.get(ilst_start..ilst_start + ilst_size)?;

    // keys is a FullBox: 4 bytes version+flags, then 4 bytes entry count
    if keys_data.len() < 8 {
        return None;
    }
    let entry_count = u32::from_be_bytes(keys_data[4..8].try_into().ok()?) as usize;

    // Scan key entries to find com.apple.quicktime.creationdate
    let mut key_pos = 8usize;
    let mut target_index: Option<u32> = None;

    for i in 0..entry_count {
        if key_pos + 8 > keys_data.len() {
            break;
        }
        let key_entry_size =
            u32::from_be_bytes(keys_data[key_pos..key_pos + 4].try_into().ok()?) as usize;
        if key_entry_size < 8 || key_pos + key_entry_size > keys_data.len() {
            break;
        }
        // key_namespace: keys_data[key_pos+4..key_pos+8]
        // key_value:     keys_data[key_pos+8..key_pos+key_entry_size]
        let key_value = &keys_data[key_pos + 8..key_pos + key_entry_size];
        if key_value == b"com.apple.quicktime.creationdate" {
            target_index = Some((i + 1) as u32); // 1-based index
        }
        key_pos += key_entry_size;
    }

    let target = target_index?;

    // Scan ilst items: each has a 4-byte size and 4-byte 1-based index
    let mut ilst_pos = 0usize;
    while ilst_pos + 8 <= ilst_data.len() {
        let item_size =
            u32::from_be_bytes(ilst_data[ilst_pos..ilst_pos + 4].try_into().ok()?) as usize;
        if item_size < 8 || ilst_pos + item_size > ilst_data.len() {
            break;
        }
        let item_index =
            u32::from_be_bytes(ilst_data[ilst_pos + 4..ilst_pos + 8].try_into().ok()?);

        if item_index == target {
            let item_children = ilst_data.get(ilst_pos + 8..ilst_pos + item_size)?;
            if let Some((data_start, data_size)) = find_child_atom(item_children, b"data") {
                let data_content = item_children.get(data_start..data_start + data_size)?;
                // data FullBox-like: 4 bytes type indicator + 4 bytes locale + value
                if data_content.len() > 8 {
                    let value_bytes = &data_content[8..];
                    if let Ok(s) = std::str::from_utf8(value_bytes) {
                        return parse_with_offset(s.trim());
                    }
                }
            }
            break;
        }

        ilst_pos += item_size;
    }

    None
}

/// Parse a datetime string with timezone offset (used as-is, no UTC conversion).
/// Handles "+02:00" (RFC 3339) and "+0200" (iPhone format).
fn parse_with_offset(s: &str) -> Option<NaiveDateTime> {
    if let Ok(dt) = DateTime::<FixedOffset>::parse_from_rfc3339(s) {
        return Some(dt.naive_local());
    }
    if let Ok(dt) = DateTime::<FixedOffset>::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%z") {
        return Some(dt.naive_local());
    }
    if let Ok(dt) = DateTime::<FixedOffset>::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%z") {
        return Some(dt.naive_local());
    }
    None
}
