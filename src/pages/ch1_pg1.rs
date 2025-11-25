use crate::page_builder::{
    buffer_to_lines, render_code, render_paragraph, setup_code, setup_paragraph,
};
use crate::view::Page;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

// page for the reading!
pub fn create_page(width: &u16, _height: &u16, vertical_scroll: u16) -> Page<'static> {
    // setup hai world paragraph
    const INFO1: &str = "a shrimple hai world program. file with any name (something.rs) exists in src/ folder at root of project and must have function called main.";
    let (info1_para, info1_height) = setup_paragraph(INFO1, *width);
    // setup hai world code
    let (_, code1_editor, code1_height) = setup_code("./examples/hai_world.rs");
    // setup primitive types paragraph
    const INFO2: &str = "now to abstract dis string and some other values into each primitive type before printing them.";
    let (info2_para, info2_height) = setup_paragraph(INFO2, *width);
    // setup primitive types code
    let (_, code2_editor, code2_height) = setup_code("./examples/primitive_types_def.rs");

    // defining virtual buffer for scrolling
    let buffer_height = info1_height + code1_height + info2_height + code2_height;
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: width.to_owned(),
        height: buffer_height as u16,
    });

    // render hai world paragraph to virtual buffer
    let mut current_y = 0;
    render_paragraph(&mut buf, info1_para, current_y, *width, info1_height);
    // center hai world code
    current_y += info1_height;
    render_code(
        &mut buf,
        code1_editor,
        current_y,
        *width,
        code1_height,
        "hai-world/src/main.rs",
    );
    // render primitive types paragraph
    current_y += code1_height;
    render_paragraph(&mut buf, info2_para, current_y, *width, info2_height);
    // center primitive types code
    current_y += info2_height;
    render_code(
        &mut buf,
        code2_editor,
        current_y,
        *width,
        code2_height,
        "primitive-types/src/main.rs",
    );

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
