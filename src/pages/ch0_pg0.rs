use crate::pages::editor_wrapper::EditorWidget;
use crate::pages::theme::tree_sitter;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct Page<'a> {
    pub content: Paragraph<'a>,
    pub height: u16,
}

// page for the reading!
pub fn create_page(width: u16, vertical_scroll: u16) -> Page<'static> {
    // introduce hai world
    let hai_world = "dis is a simple hai world program";
    let para_hai_world = Paragraph::new(hai_world);
    let para_height = hai_world.lines().count() as u16;

    // setup hai world code
    let theme = tree_sitter();
    let code_content = std::fs::read_to_string("./examples/hai_world.rs").unwrap();
    let code_hai_world = EditorWidget::new("rust", &code_content, theme);
    let code_height = code_content.lines().count() as u16 + 5;

    // defining virtual buffer for scrolling
    let buffer_height = para_height + code_height;
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: width,
        height: buffer_height as u16,
    });
    // track scroll position
    let mut current_y = 0;

    // render paragraph to virtual buffer
    para_hai_world.render(
        Rect {
            x: 0,
            y: current_y,
            width: buf.area.width,
            height: para_height,
        },
        &mut buf,
    );

    // center code
    current_y += para_height;
    let code_section = Rect::new(0, current_y, width, code_height);
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
    code_hai_world.render(inner, &mut buf);

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
