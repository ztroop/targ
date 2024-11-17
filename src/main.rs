#![allow(dead_code)]
use std::io;

use app::App;
use clap::Parser;
use event::{Event, EventHandler};
use handler::handle_key_events;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;

mod app;
mod debug;
mod event;
mod handler;
mod structs;
mod tar;
mod tui;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = structs::Args::parse();
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1_000);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let mut app = App::new(args);
    app.table_state.select(Some(0));

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
