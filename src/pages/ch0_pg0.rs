use crate::ascii::{AsciiArt, TITLE};
use crate::theme::center;
use crate::view::Page;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

// page for the reading!
pub fn create_page(width: &u16, height: &u16, vertical_scroll: u16) -> Page<'static> {
    // create ascii for cover art
    let ascii_cover = AsciiArt::new();
    let ascii_width_usize = TITLE.lines().map(|line| line.len()).max().unwrap_or(0);
    let ascii_width = (ascii_width_usize as u16).min(u16::MAX);
    let ascii_height = TITLE.lines().count() as u16;

    // defining virtual buffer for scrolling
    let area = Rect {
        x: 0,
        y: 0,
        width: width.to_owned(),
        height: height.to_owned(),
    };
    let mut buf = Buffer::empty(area);

    // render ascii art
    let inner = center(&area, ascii_width, ascii_height);
    ascii_cover.render(inner, &mut buf);

    // convert buffer to lines
    let lines: Vec<Line> = (0..height.to_owned())
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
        height: height.to_owned(),
    }
}
