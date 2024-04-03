use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::{app::App, structs::FileOrDir};

/// Render the UI.
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(frame.size());

    let contents: Vec<Row<'static>> = app
        .display_contents()
        .iter()
        .map(|item| match item {
            FileOrDir::File {
                path,
                size,
                modified,
            } => {
                let size_str = format!("{} bytes", size);
                let date_str = modified.format("%Y-%m-%d %H:%M:%S").to_string();
                Row::new(vec![
                    Cell::from(path.clone()),
                    Cell::from(size_str),
                    Cell::from(date_str),
                ])
            }
            FileOrDir::Dir { path, .. } => Row::new(vec![
                Span::styled(path.clone(), Style::default().fg(Color::Yellow)),
                Span::from(""),
                Span::from(""),
            ]),
        })
        .collect();

    let table = Table::new(
        contents,
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

    frame.render_stateful_widget(table, chunks[0], &mut app.table_state);

    let hint_rows = vec![Row::new(vec![
        "[q → exit]".to_string(),
        "[up/down → navigate]".to_string(),
    ])
    .style(Style::default().fg(Color::DarkGray))];
    let hint_table =
        Table::new(hint_rows, [Constraint::Length(10), Constraint::Length(20)]).column_spacing(1);

    frame.render_widget(hint_table, chunks[1])
}
