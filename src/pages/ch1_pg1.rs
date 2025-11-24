use crate::editor_wrapper::EditorWidget;
use crate::theme::tree_sitter;
use crate::view::Page;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

// page for the reading!
pub fn create_page(width: &u16, _height: &u16, vertical_scroll: u16) -> Page<'static> {
    // use usize for calculations wit other usize
    let width_usize = *width as usize;
    // setup hai world paragraph
    const INFO1: &str = "a shrimple hai world program. file with any name (something.rs) exists in src/ folder at root of project and must have function called main.";
    let info1_para = Paragraph::new(INFO1).wrap(Wrap { trim: true });
    // get total characters divided by characters per line, rounded up
    let info1_height = ((INFO1.chars().count() + width_usize - 1) / width_usize).max(1) as u16;

    // setup hai world code
    let theme = tree_sitter();
    let code1_content = std::fs::read_to_string("./examples/hai_world.rs").unwrap();
    let code1_editor = EditorWidget::new("rust", &code1_content, theme);
    let code1_height = code1_content.lines().count() as u16 + 5;

    const INFO2: &str = "now to abstract dis string and many other values into some useful types before printing them.";
    let info2_para = Paragraph::new(INFO2).wrap(Wrap { trim: true });
    let info2_height = ((INFO2.chars().count() + width_usize - 1) / width_usize).max(1) as u16;

    // defining virtual buffer for scrolling
    let buffer_height = info1_height + code1_height + info2_height;
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: width.to_owned(),
        height: buffer_height as u16,
    });
    // track scroll position
    let mut current_y = 0;

    // render paragraph to virtual buffer
    let info1_rect = Rect::new(0, current_y, buf.area.width, info1_height);
    let info1_block = Block::default();
    let info1_inner = info1_block.inner(info1_rect);
    info1_para.render(info1_inner, &mut buf);

    // center code
    current_y += info1_height;
    let code_rect = Rect::new(0, current_y, width.to_owned(), code1_height);
    let [centered] = Layout::horizontal([Constraint::Length(100)])
        .flex(Flex::Center)
        .areas(code_rect);
    // pad out code wit borders
    let code_block = Block::default()
        .borders(Borders::ALL)
        .title(Line::from(" main.rs ").centered());
    let margin = centered.inner(Margin::new(0, 1));
    let inner = code_block.inner(margin);
    // render code
    code_block.render(margin, &mut buf);
    code1_editor.render(inner, &mut buf);

    // render second paragraph
    current_y += code1_height;
    let info2_rect = Rect::new(0, current_y, buf.area.width, info2_height);
    let info2_block = Block::default();
    let info2_inner = info2_block.inner(info2_rect);
    info2_para.render(info2_inner, &mut buf);

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
