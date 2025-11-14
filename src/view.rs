use crate::model::Model;
use crate::pages::ALL_PAGES;
use ratatui::{
    Frame,
    layout::Margin,
    widgets::{
        Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
};

pub struct Page<'a> {
    pub content: Paragraph<'a>,
    pub height: u16,
}

// rendering view to always produce same ui representation for given model
pub fn view(model: &mut Model, frame: &mut Frame) {
    // area for pages, minus borders (2), padding (4), scrollbar (1)
    let area = frame.area();
    let page_width = area.width.saturating_sub(7);
    let page = ALL_PAGES[model.page as usize](&page_width, model.y_pos);
    let block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1));
    frame.render_widget(page.content.block(block), area);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let visible_height = area.height.saturating_sub(3);
    model.max_scroll = page.height.saturating_sub(visible_height);
    if model.max_scroll > 0 {
        let mut scrollbar_state =
            ScrollbarState::new(model.max_scroll.into()).position(model.y_pos.into());
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
