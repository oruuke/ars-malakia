use crate::model::Model;
use crate::pages::ch0_pg0::create_page;
use ratatui::{
    Frame,
    layout::Margin,
    widgets::{Block, Borders, Padding, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

// rendering view to always produce same ui representation for given model
pub fn view(model: &mut Model, frame: &mut Frame) {
    // area for pages, minus borders (2), padding (4), scrollbar (1)
    let area = frame.area();
    let page = create_page(area.width.saturating_sub(7), model.y_pos);
    let block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 0, 2));
    frame.render_widget(page.content.block(block), area);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let max_scroll = (page.height as usize).saturating_sub(area.height as usize);
    if max_scroll == 0 {
        model.is_scrollable = false;
    } else {
        model.is_scrollable = true;
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
}
