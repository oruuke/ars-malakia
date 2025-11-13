//use crate::editor_wrapper::EditorWidget;
use crate::model::Model;
use crate::pages::ch0_pg0::create_page;
//use crate::theme::tree_sitter;
use ratatui::{
    Frame,
    layout::Margin,
    widgets::{Block, Borders, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

// rendering view to always produce same ui representation for given model
pub fn view(model: &mut Model, frame: &mut Frame) {
    let area = frame.area();
    let page = create_page(area.width.saturating_sub(1), model.y_pos);
    let block = Block::default().borders(Borders::ALL);
    frame.render_widget(page.content.block(block), area);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let max_scroll = (page.height as usize).saturating_sub(area.height as usize);
    let mut scrollbar_state = ScrollbarState::new(max_scroll).position(model.y_pos);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    )
}
