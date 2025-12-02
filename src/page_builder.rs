use crate::editor_wrapper::EditorWidget;
use crate::theme::{PINK3, build_style, tree_sitter};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

// setup paragraph and calculate height
pub fn setup_paragraph(content: &str, width: u16) -> (Paragraph<'static>, u16) {
    let para = Paragraph::new(content.to_string()).wrap(Wrap { trim: true });
    let height = ((content.chars().count() + width as usize - 1) / width as usize).max(1) as u16;
    (para, height)
}

// render paragraph directly to buffer at y position
pub fn render_paragraph(buf: &mut Buffer, para: Paragraph, y_pos: u16, width: u16, height: u16) {
    let rect = Rect::new(0, y_pos, width, height);
    let block = Block::default();
    let inner = block.inner(rect);
    para.render(inner, buf);
}

// setup code and calculate height
pub fn setup_code(file_path: &str) -> (String, EditorWidget, u16) {
    let content = std::fs::read_to_string(file_path).unwrap();
    let height = content.lines().count() as u16 + 5;
    let editor = EditorWidget::new("rust", &content, tree_sitter());
    (content, editor, height)
}

// render centered code editor directly to buffer at y position
pub fn render_code(
    buf: &mut Buffer,
    editor: EditorWidget,
    y_pos: u16,
    width: u16,
    height: u16,
    title: &str,
) {
    // centered and margined layout wit max width to help break up page
    let rect = Rect::new(0, y_pos, width, height);
    let margin = rect.inner(Margin::new(0, 1));
    let [layout] = Layout::horizontal([Constraint::Length(90)])
        .flex(Flex::Center)
        .areas(margin);
    // in basic titled border
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(build_style(PINK3))
        .title(Line::from(format!(" {} ", title)).centered());
    let inner = block.inner(layout);
    // rendered to buffer
    block.render(layout, buf);
    editor.render(inner, buf);
}

// convert buffer to styled lines
pub fn buffer_to_lines(buf: &Buffer, height: u16) -> Vec<Line<'static>> {
    (0..height)
        .map(|y| {
            // allocation
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
        .collect()
}
