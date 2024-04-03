/// A struct to represent a tar entry
struct TarEntry {
    path: PathBuf,
    size: u64,
    modified_time: Option<SystemTime>,
}