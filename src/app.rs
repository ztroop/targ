use std::error;

use ratatui::widgets::{Row, TableState};

use crate::{structs::Args, tar::read_tar_contents};

pub struct App {
    pub running: bool,
    pub tar_contents: Vec<Row<'static>>,
    pub table_state: TableState,
}

impl App {
    /// Create a new App with the given tar contents.
    pub fn new(args: Args) -> Self {
        Self {
            running: true,
            tar_contents: read_tar_contents(args.tar_file, args.show_indicator).unwrap(),
            table_state: TableState::default(),
        }
    }

    /// Move the selection up in the table.
    pub fn move_up(&mut self) {
        let previous = match self.table_state.selected() {
            Some(selected) => {
                if selected == 0 {
                    self.tar_contents.len() - 1
                } else {
                    selected - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(previous));
    }

    /// Move the selection down in the table.
    pub fn move_down(&mut self) {
        let next = match self.table_state.selected() {
            Some(selected) => {
                if selected >= self.tar_contents.len() - 1 {
                    0
                } else {
                    selected + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(next));
    }

    /// Update the app state on a tick event.
    pub fn tick(&mut self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    /// Update the app state to quit.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
