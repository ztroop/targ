use std::{
    collections::HashMap,
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
    let mut dir_map: HashMap<String, Vec<FileOrDir>> = HashMap::new();

    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?;

        if !show_indicator
            && path
                .file_name()
                .and_then(|s| s.to_str())
                .map_or(false, |s| s.starts_with("._"))
        {
            continue;
        }

        let path_str = path.to_str().unwrap().to_string();
        let clean_path = path_str.trim_end_matches('/').to_string();
        let parent = Path::new(&clean_path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string();

        let size = entry.header().size()?;
        let modified_time = entry
            .header()
            .mtime()
            .map(|t| SystemTime::UNIX_EPOCH + Duration::new(t, 0))
            .unwrap();
        let modified = DateTime::<Utc>::from(modified_time);

        let file_or_dir = match entry.header().entry_type() {
            tar::EntryType::Directory => FileOrDir::Dir {
                path: clean_path,
                expanded: false,
                children: Vec::new(),
            },
            _ => FileOrDir::File {
                path: clean_path,
                size,
                modified,
            },
        };

        dir_map.entry(parent).or_default().push(file_or_dir);
    }

    fn build_tree(path: &str, dir_map: &HashMap<String, Vec<FileOrDir>>) -> Vec<FileOrDir> {
        let normalized_path = path.trim_end_matches('/');

        match dir_map.get(normalized_path) {
            Some(entries) => {
                let mut result = Vec::new();

                for entry in entries {
                    match entry {
                        FileOrDir::Dir { path: dir_path, .. } => {
                            let children = build_tree(dir_path, dir_map);
                            result.push(FileOrDir::Dir {
                                path: dir_path.clone(),
                                expanded: false,
                                children,
                            });
                        }
                        FileOrDir::File { .. } => {
                            result.push(entry.clone());
                        }
                    }
                }
                result
            }
            None => Vec::new(),
        }
    }

    let result = build_tree("", &dir_map);
    Ok(result)
}
