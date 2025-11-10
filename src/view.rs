use crate::editor_wrapper::EditorWidget;
use crate::model::Model;
use crate::theme::tree_sitter;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders},
};

// rendering view to always produce same ui representation for given model
pub fn view(_model: &mut Model, frame: &mut Frame) {
    // setup code
    let theme = tree_sitter();
    let content = std::fs::read_to_string("./src/editor_wrapper.rs").unwrap();
    let editor = EditorWidget::new("rust", &content, theme);

    // half-split layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    // code with border
    let block = Block::default().borders(Borders::ALL).title(" editor ");
    let inner = block.inner(layout[0]);
    frame.render_widget(block, layout[0]);
    frame.render_widget(editor, inner);
}
