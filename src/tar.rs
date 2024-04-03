use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    time::{Duration, SystemTime},
};

use flate2::bufread::GzDecoder;
use ratatui::widgets::{Cell, Row};
use tar::Archive;

pub fn read_tar_contents<P: AsRef<Path>>(tar_path: P) -> io::Result<Vec<Row<'static>>> {
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
        let path = entry.path()?.to_path_buf();
        let size = entry.header().size()?;

        let path_str = path.to_str().unwrap().to_string();
        let size_str = format!("{} bytes", size);
        let modified_time = entry
            .header()
            .mtime()
            .map(|t| SystemTime::UNIX_EPOCH + Duration::new(t, 0))
            .unwrap();
        let modified_time_epoch = modified_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Convert the modified time to a human readable format that is ISO 8601 compliant
        let modified_time_str = chrono::DateTime::from_timestamp(modified_time_epoch as i64, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        entries.push(Row::new(vec![
            Cell::from(path_str),
            Cell::from(size_str),
            Cell::from(modified_time_str),
        ]));
    }

    Ok(entries)
}
