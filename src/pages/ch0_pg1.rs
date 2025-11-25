use crate::page_builder::{buffer_to_lines, render_paragraph, setup_paragraph};
use crate::view::Page;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

// page for the reading!
pub fn create_page(width: &u16, _height: &u16, vertical_scroll: u16) -> Page<'static> {
    // setup prelude paragraph
    const INFO1: &str = "prelude here...";
    let (info1_para, info1_height) = setup_paragraph(INFO1, *width);

    // defining virtual buffer for scrolling
    let buffer_height = info1_height;
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: width.to_owned(),
        height: buffer_height as u16,
    });
    // track scroll position
    let current_y = 0;
    render_paragraph(&mut buf, info1_para, current_y, *width, info1_height);

    // convert buffer to lines
    let lines = buffer_to_lines(&buf, buffer_height);

    // create scrollable paragraph
    let content = Paragraph::new(lines)
        .scroll((vertical_scroll as u16, 0))
        .block(Block::new().borders(Borders::NONE));

    Page {
        content,
        height: buffer_height,
    }
}
