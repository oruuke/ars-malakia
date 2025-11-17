use crate::editor_wrapper::EditorWidget;
use crate::theme::tree_sitter;
use crate::view::Page;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

// page for the reading!
pub fn create_page(width: &u16, _height: &u16, vertical_scroll: u16) -> Page<'static> {
    // introduce hai world
    let info1_str = "dis is another hai world program";
    let info1_para = Paragraph::new(info1_str);
    let info1_height = info1_str.lines().count() as u16;

    // setup hai world code
    let theme = tree_sitter();
    let code1_content = std::fs::read_to_string("./examples/hai_world.rs").unwrap();
    let code1_editor = EditorWidget::new("rust", &code1_content, theme);
    let code1_height = code1_content.lines().count() as u16 + 5;

    // defining virtual buffer for scrolling
    let buffer_height = info1_height + code1_height;
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: width.to_owned(),
        height: buffer_height as u16,
    });
    // track scroll position
    let mut current_y = 0;

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

    // center code
    current_y += info1_height;
    let code_section = Rect::new(0, current_y, width.to_owned(), code1_height);
    let [centered] = Layout::horizontal([Constraint::Length(100)])
        .flex(Flex::Center)
        .areas(code_section);
    // pad out code wit borders
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Line::from(" main.rs ").centered());
    let padding = centered.inner(Margin::new(0, 1));
    let inner = block.inner(padding);
    // render code
    block.render(padding, &mut buf);
    code1_editor.render(inner, &mut buf);

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
