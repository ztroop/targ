use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    time::{Duration, SystemTime},
};

use chrono::offset::Utc;
use chrono::DateTime;
use flate2::bufread::GzDecoder;
use tar::Archive;

use crate::structs::FileOrDir;

/// Read the contents of a tar file and return a vector of rows.
pub fn read_tar_contents<P: AsRef<Path>>(
    tar_path: P,
    show_indicator: bool,
) -> io::Result<Vec<FileOrDir>> {
    let file = File::open(tar_path.as_ref())?;
    let buf_reader = BufReader::new(file);

    let tar: Box<dyn Read> = if tar_path.as_ref().extension().and_then(|s| s.to_str()) == Some("gz")
    {
        Box::new(GzDecoder::new(buf_reader))
    } else {
        Box::new(buf_reader)
    };

    let mut archive = Archive::new(tar);
    let mut entries = Vec::new();

    for entry in archive.entries()? {
        let entry = entry?;

        // Skip indicator files if they are not requested
        if !show_indicator
            && entry
                .path()?
                .file_name()
                .and_then(|s| s.to_str())
                .map_or(false, |s| s.starts_with("._"))
        {
            continue;
        }

        let path = entry.path()?.to_path_buf();
        let size = entry.header().size()?;

        let path_str = path.to_str().unwrap().to_string();
        let modified_time = entry
            .header()
            .mtime()
            .map(|t| SystemTime::UNIX_EPOCH + Duration::new(t, 0))
            .unwrap();
        // Convert the modified time to a human readable format that is ISO 8601 compliant
        let modified = DateTime::<Utc>::from(modified_time);

        match entry.header().entry_type() {
            tar::EntryType::Directory => {
                entries.push(FileOrDir::Dir {
                    path: path_str,
                    expanded: false,
                });
            }
            _ => {
                entries.push(FileOrDir::File {
                    path: path_str,
                    size: size,
                    modified,
                });
            }
        }
    }

    Ok(entries)
}
