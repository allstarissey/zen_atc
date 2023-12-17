use ratatui::{
    layout::{Alignment, Constraint, Direction as LDirection, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let frame_width = frame.size().width;
    let (map_width, map_height) = app.dimensions();

    let pad_width = (frame_width - map_width) / 2;

    // Chunks
    let chunks = Layout::default()
        .direction(LDirection::Vertical)
        .constraints([
            Constraint::Max(2),
            Constraint::Length(*map_height),
            Constraint::Max(2),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.size());
    let upper_chunks = Layout::default()
        .direction(LDirection::Horizontal)
        .constraints([
            Constraint::Max(pad_width),
            Constraint::Min(*map_width),
            Constraint::Max(pad_width),
        ])
        .split(chunks[1]);
    let lower_chunks = Layout::default()
        .direction(LDirection::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(chunks[3]);

    // Blocks
    let radar_block = make_block("Radar");
    let command_list_block = make_block("Commands");
    let plane_list_block = make_block("Planes");
    let command_block = make_block("Command");

    let command_writer = Paragraph::new(app.cur_command()).block(command_block);

    frame.render_widget(radar_block, upper_chunks[1]);
    frame.render_widget(command_list_block, lower_chunks[0]);
    frame.render_widget(plane_list_block, lower_chunks[1]);
    frame.render_widget(command_writer, chunks[4]);
}

fn make_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_alignment(Alignment::Center)
}
