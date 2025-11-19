use crate::ascii::TITLE;
use crate::ascii_art::AsciiArt;
use crate::theme::center;
use crate::view::Page;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

// page for the reading!
pub fn create_page(width: &u16, height: &u16, vertical_scroll: u16) -> Page<'static> {
    // get ascii dimensions
    let ascii_width_usize = TITLE.lines().map(|line| line.len()).max().unwrap_or(1);
    let ascii_width = ascii_width_usize.min(u16::MAX as usize) as u16;
    let ascii_height = TITLE.lines().count().max(1) as u16;
    // create ascii for cover art
    let ascii_cover = AsciiArt::new(TITLE.to_owned());

    // ensure usable space is enough to maintain rendering
    let available_width = (*width).max(1);
    let available_height = (*height).max(1);

    // render ascii into off-screen buffer at natural size
    let ascii_area = Rect {
        x: 0,
        y: 0,
        width: ascii_width,
        height: ascii_height,
    };
    let mut ascii_buf = Buffer::empty(ascii_area);
    ascii_cover.render(ascii_area, &mut ascii_buf);

    // find tight bounding box around symbols
    let mut min_x = ascii_width;
    let mut min_y = ascii_height;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut has_pixels = false;

    // scan through ascii to determine bounds
    for y in 0..ascii_height as usize {
        for x in 0..ascii_width as usize {
            let cell = &ascii_buf[(x as u16, y as u16)];
            // ignore whitespace
            if cell.symbol().trim().is_empty() {
                continue;
            }
            has_pixels = true;
            min_x = min_x.min(x as u16);
            min_y = min_y.min(y as u16);
            max_x = max_x.max(x as u16);
            max_y = max_y.max(y as u16);
        }
    }

    // create bounding box
    let usable_area = if has_pixels {
        Rect {
            x: min_x,
            y: min_y,
            width: max_x.saturating_sub(min_x) + 1,
            height: max_y.saturating_sub(min_y) + 1,
        }
    } else {
        ascii_area
    };

    // calculate visible space without exceeding terminal
    let display_width = usable_area.width.min(available_width);
    let display_height = usable_area.height.min(available_height);

    // build final buffer matching what page's display
    let area = Rect {
        x: 0,
        y: 0,
        width: available_width,
        height: available_height,
    };
    let mut buf = Buffer::empty(area);
    let ascii_view = center(&usable_area, display_width, display_height);
    let target_area = center(&area, display_width, display_height);

    // copy pixels line-by-line to maintain colour and style info
    for y in 0..display_height {
        for x in 0..display_width {
            let src = ascii_buf[(ascii_view.x + x, ascii_view.y + y)].clone();
            let dst = &mut buf[(target_area.x + x, target_area.y + y)];
            *dst = src;
        }
    }

    // convert buffer to lines
    let lines: Vec<Line> = (0..available_height)
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
        height: available_height,
    }
}
