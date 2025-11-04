use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend, layout::Position};
use ratatui_code_editor::editor::Editor;
use std::io::stdout;

use crate::model::Model;
use ratatui::{
    Frame,
    symbols::border,
    widgets::{Block, Paragraph},
};

// rendering view to always produce same ui representation for given model
pub fn view(_model: &mut Model, frame: &mut Frame) {
    let theme = vec![
        ("keyword", "#ff6b6b"),
        ("string", "#4ecdc4"),
        ("comment", "#95a5a6"),
        ("function", "#f39c12"),
        ("variable", "#B55088"),
        ("namespace", "#eb34de"),
        ("type", "#4ecdc4"),
    ];
    let content = "fn main() {\n    println!(\"Hello, world!\");\n}";
    let mut editor = Editor::new("rust", content, theme);

    let block = Block::bordered().border_set(border::THICK);
    frame.render_widget(&editor, frame.area());
}
