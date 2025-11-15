use crate::view::Page;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

// page for the reading!
pub fn create_page(width: &u16, vertical_scroll: u16) -> Page<'static> {
    // introduce hai world
    let info1_str = "table of contents here...";
    let info1_para = Paragraph::new(info1_str);
    let info1_height = info1_str.lines().count() as u16;

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

    // render paragraph to virtual buffer
    info1_para.render(
        Rect {
            x: 0,
            y: current_y,
            width: buf.area.width,
            height: info1_height,
        },
        &mut buf,
    );

    // convert buffer to lines
    let lines: Vec<Line> = (0..buffer_height)
        .map(|y| {
            // empty allocation
            let mut spans = Vec::new();
            let mut current_text = String::new();
            let mut current_style = Style::new();
            // iterate each character in line
            for x in 0..buf.area.width {
                let cell = &buf[(x, y)];
                // add colours back in
                if cell.style() != current_style && !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                }
                current_style = cell.style();
                current_text.push_str(cell.symbol());
            }
            // add line if not already added
            if !current_text.is_empty() {
                spans.push(Span::styled(current_text, current_style));
            }
            Line::from(spans)
        })
        .collect();

    // create scrollable paragraph
    let content = Paragraph::new(lines)
        .scroll((vertical_scroll as u16, 0))
        .block(Block::new().borders(Borders::NONE));

    Page {
        content,
        height: buffer_height,
    }
}
