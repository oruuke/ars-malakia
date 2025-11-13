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

pub fn create_page(width: u16, vertical_scroll: usize) -> Page<'static> {
    // defining virtual buffer for scrolling
    let buffer_height = 150;
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: width,
        height: buffer_height as u16,
    });

    // track scroll position
    let mut current_y = 0;

    // test widget
    let p_hai_world = "dis is a simple hai world program";
    let para = Paragraph::new(p_hai_world);
    let para_height = p_hai_world.lines().count() as u16;

    // render widget to virtual buffer
    para.render(
        Rect {
            x: 0,
            y: current_y,
            width: buf.area.width,
            height: para_height,
        },
        &mut buf,
    );

    // setup code
    current_y += para_height;
    let theme = tree_sitter();
    let content = std::fs::read_to_string("./examples/hai_world.rs").unwrap();
    let editor = EditorWidget::new("rust", &content, theme);
    let editor_height = content.lines().count() as u16 + 6;
    // render code
    let editor_section = Rect::new(0, current_y, width, editor_height);
    let [centered] = Layout::horizontal([Constraint::Length(100)])
        .flex(Flex::Center)
        .areas(editor_section);
    let padding = centered.inner(Margin::new(2, 2));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Line::from(" code ").centered());

    let inner = block.inner(padding);
    block.render(padding, &mut buf);
    editor.render(inner, &mut buf);

    // convert buffer to lines
    let lines: Vec<Line> = (0..buffer_height)
        .map(|y| {
            let mut spans = Vec::new();
            let mut current_text = String::new();
            let mut current_style = Style::new();
            for x in 0..buf.area.width {
                let cell = &buf[(x, y)];
                if cell.style() != current_style && !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                }
                current_style = cell.style();
                current_text.push_str(cell.symbol());
            }
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
