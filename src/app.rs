use crate::{
    debug::debug_log,
    structs::{Args, FileOrDir},
    tar::read_tar_contents,
};
use ratatui::widgets::TableState;
use std::error;
use std::path::Path;

pub struct App {
    pub running: bool,
    pub tar_contents: Vec<FileOrDir>,
    pub table_state: TableState,
    pub current_path: Vec<String>,
    pub debug: bool,
}

impl App {
    pub fn new(args: Args) -> Self {
        let mut app = Self {
            running: true,
            tar_contents: read_tar_contents(args.tar_file, args.show_indicator).unwrap(),
            table_state: TableState::default(),
            current_path: Vec::new(),
            debug: args.debug,
        };
        app.table_state.select(Some(0));
        app
    }

    pub fn display_contents(&self) -> Vec<&FileOrDir> {
        let current_path = self.current_path.join("/");
        if self.debug {
            debug_log(&format!("\nDisplay contents for path: '{}'", current_path));
        }

        self.tar_contents
            .iter()
            .flat_map(|item| match item {
                FileOrDir::Dir { path, children, .. } => {
                    if path.trim_end_matches('/') == current_path {
                        if self.debug {
                            debug_log(&format!(
                                "Returning {} children of {}",
                                children.len(),
                                path
                            ));
                        }
                        children.iter().collect::<Vec<&FileOrDir>>()
                    } else if current_path.is_empty() && !path.contains('/') {
                        if self.debug {
                            debug_log(&format!("Root level item: {}", path));
                        }
                        vec![item].into_iter().collect()
                    } else {
                        Vec::new().into_iter().collect()
                    }
                }
                FileOrDir::File { path, .. } => {
                    let parent = Path::new(path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or("");
                    if parent == current_path {
                        if self.debug {
                            debug_log(&format!("Including file: {}", path));
                        }
                        vec![item].into_iter().collect()
                    } else if current_path.is_empty() && !path.contains('/') {
                        if self.debug {
                            debug_log(&format!("Root level file: {}", path));
                        }
                        vec![item].into_iter().collect()
                    } else {
                        Vec::new().into_iter().collect()
                    }
                }
            })
            .collect()
    }

    pub fn enter_directory(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            let contents = self.display_contents();
            if selected < contents.len() {
                if let FileOrDir::Dir { path, .. } = contents[selected] {
                    debug_log(&format!("Entering directory: {}", path));
                    let components: Vec<String> = path
                        .trim_end_matches('/')
                        .split('/')
                        .map(String::from)
                        .collect();
                    debug_log(&format!("New path components: {:?}", components));
                    self.current_path = components;
                    self.table_state.select(Some(0));
                }
            }
        }
    }

    pub fn go_back(&mut self) {
        if !self.current_path.is_empty() {
            self.current_path.pop();
            self.table_state.select(Some(0));
        }
    }

    pub fn move_up(&mut self) {
        let contents = self.display_contents();
        let previous = match self.table_state.selected() {
            Some(selected) => {
                if selected == 0 {
                    contents.len().saturating_sub(1)
                } else {
                    selected.saturating_sub(1)
                }
            }
            None => 0,
        };
        if !contents.is_empty() {
            self.table_state.select(Some(previous));
        }
    }

    pub fn move_down(&mut self) {
        let contents = self.display_contents();
        let next = match self.table_state.selected() {
            Some(selected) => {
                if selected >= contents.len().saturating_sub(1) {
                    0
                } else {
                    selected + 1
                }
            }
            None => 0,
        };
        if !contents.is_empty() {
            self.table_state.select(Some(next));
        }
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
