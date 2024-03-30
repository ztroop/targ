use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use flate2::bufread::GzDecoder;
use ratatui::{
    backend::CrosstermBackend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Cell, Row, Table},
};
use ratatui::{
    widgets::{Block, Borders},
    Terminal,
};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
    time::SystemTime,
};
use tar::Archive;

enum Event<I> {
    Input(I),
    Tick,
}

#[allow(dead_code)]
struct TarEntry {
    path: PathBuf,
    size: u64,
    modified_time: Option<SystemTime>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).expect("Failed to poll for events") {
                if let CEvent::Key(key) = event::read().expect("Failed to read the event") {
                    tx.send(Event::Input(key))
                        .expect("Failed to send keyboard input event");
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if tx.send(Event::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_tar_file>", args[0]);
        std::process::exit(1);
    }
    let tar_file_path = &args[1];
    let tar_contents = read_tar_contents(tar_file_path).unwrap();

    loop {
        terminal.draw(|f| {
            let table = Table::new(
                tar_contents.clone(),
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ],
            )
            .block(Block::default().title("TAR Contents").borders(Borders::ALL))
            .header(
                Row::new(vec!["File Path", "File Size", "Last Modified"])
                    .style(Style::default().fg(Color::Yellow)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            let size = f.size();
            f.render_widget(table, size);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn read_tar_contents<P: AsRef<Path>>(tar_path: P) -> io::Result<Vec<Row<'static>>> {
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
