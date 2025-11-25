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
    let code1_content = std::fs::read_to_string("./examples/hai_world.rs").unwrap();
    let code1_editor = EditorWidget::new("rust", &code1_content, tree_sitter());
    let code1_height = code1_content.lines().count() as u16 + 5;

    // setup primitive types paragraph
    const INFO2: &str = "now to abstract dis string and some other values into each primitive type before printing them.";
    let info2_para = Paragraph::new(INFO2).wrap(Wrap { trim: true });
    let info2_height = ((INFO2.chars().count() + width_usize - 1) / width_usize).max(1) as u16;

    // setup primitive types code
    let code2_content = std::fs::read_to_string("./examples/primitive_types_def.rs").unwrap();
    let code2_editor = EditorWidget::new("rust", &code2_content, tree_sitter());
    let code2_height = code2_content.lines().count() as u16 + 5;

    // defining virtual buffer for scrolling
    let buffer_height = info1_height + code1_height + info2_height + code2_height;
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: width.to_owned(),
        height: buffer_height as u16,
    });
    // track widget positions within scroll area
    let mut current_y = 0;

    // render hai world paragraph to virtual buffer
    let info1_rect = Rect::new(0, current_y, buf.area.width, info1_height);
    let info1_block = Block::default();
    let info1_inner = info1_block.inner(info1_rect);
    info1_para.render(info1_inner, &mut buf);

    // center hai world code
    current_y += info1_height;
    let code1_rect = Rect::new(0, current_y, width.to_owned(), code1_height);
    let [code1_layout] = Layout::horizontal([Constraint::Length(100)])
        .flex(Flex::Center)
        .areas(code1_rect);
    // pad out hai world code wit borders
    let code1_block = Block::default()
        .borders(Borders::ALL)
        .title(Line::from(" hai_world/src/main.rs ").centered());
    let code1_margin = code1_layout.inner(Margin::new(0, 1));
    let code1_inner = code1_block.inner(code1_margin);
    // render hai world code
    code1_block.render(code1_margin, &mut buf);
    code1_editor.render(code1_inner, &mut buf);

    // render primitive types paragraph
    current_y += code1_height;
    let info2_rect = Rect::new(0, current_y, buf.area.width, info2_height);
    let info2_block = Block::default();
    let info2_inner = info2_block.inner(info2_rect);
    info2_para.render(info2_inner, &mut buf);

    // center primitive types code
    current_y += info2_height;
    let code2_rect = Rect::new(0, current_y, width.to_owned(), code2_height);
    let [code2_layout] = Layout::horizontal([Constraint::Length(100)])
        .flex(Flex::Center)
        .areas(code2_rect);
    // pad out primitive types code
    let code2_block = Block::default()
        .borders(Borders::ALL)
        .title(Line::from(" primitive_types/src/main.rs ").centered());
    let code2_margin = code2_layout.inner(Margin::new(0, 1));
    let code2_inner = code2_block.inner(code2_margin);
    // render primitive types code
    code2_block.render(code2_margin, &mut buf);
    code2_editor.render(code2_inner, &mut buf);

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
