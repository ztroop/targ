use chrono::{DateTime, Utc};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(help = "Path to the tar file")]
    #[arg(short = 't', long = "tar-file")]
    pub tar_file: String,
    #[clap(help = "Show indicator files (e.g. ._* files)")]
    #[arg(long = "show-indicator")]
    pub show_indicator: bool,
}

pub enum FileOrDir {
    File {
        path: String,
        size: u64,
        modified: DateTime<Utc>,
    },
    Dir {
        path: String,
        expanded: bool,
    },
}
