use crate::model::Model;
use ratatui::layout::Position;
use ratatui::{
    Frame,
    symbols::border,
    widgets::{Block, Paragraph},
};
use ratatui_code_editor::editor::Editor;

// rendering view to always produce same ui representation for given model
pub fn view(_model: &mut Model, frame: &mut Frame) {
    // pink
    let col_pink1 = "#FFAFFF";
    let col_pink2 = "#DC8BB2";
    let col_pink3 = "#B55088";
    // cyan
    let col_cyan2 = "#007F7F";
    // rose
    let col_rose1 = "#DD4042";
    // gold
    let col_gold1 = "#FCDD6C";
    // purp
    let col_purp1 = "#EA00F7";

    let theme = vec![
        ("keyword", col_pink3),
        ("string", col_gold1),
        ("comment", col_cyan2),
        ("function", col_purp1),
        ("variable", col_pink1),
        ("namespace", col_pink2),
        ("type", col_rose1),
    ];
    let content = std::fs::read_to_string("./src/update.rs").unwrap();
    let editor = Editor::new("rust", &content, theme);

    frame.render_widget(&editor, frame.area());
}
