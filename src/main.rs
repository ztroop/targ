use std::{env, io};

use app::App;
use event::{Event, EventHandler};
use handler::handle_key_events;
use ratatui::{backend::CrosstermBackend, Terminal};
use tar::read_tar_contents;
use tui::Tui;

mod app;
mod event;
mod handler;
mod tar;
mod tui;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_tar_file>", args[0]);
        std::process::exit(1);
    }
    let tar_file_path = &args[1];

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1_000);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let tar_contents = read_tar_contents(tar_file_path).unwrap();

    let mut app = App::new(tar_contents);
    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick()?,
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Resize(_, _) => {}
            Event::Mouse(_) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
