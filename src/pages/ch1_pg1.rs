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
    const INFO1: &str = "a shrimple hai world program. file wit any name (something.rs) exists in src/ folder at root of project and must have function called main.";
    let (info1_para, info1_height) = setup_paragraph(INFO1, *width);
    // setup hai world code
    let (_, code1_editor, code1_height) = setup_code("./examples/hai_world.rs");

    // setup primitive types paragraph
    const INFO2: &str = "now to abstract dis string and some other values into each primitive type before printing them.";
    let (info2_para, info2_height) = setup_paragraph(INFO2, *width);
    // setup primitive types code
    let (_, code2_editor, code2_height) = setup_code("./examples/primitive_types_def.rs");

    // setup sequence types paragraph
    const INFO3: &str = "but we can also store dese in sequence types...";
    let (info3_para, info3_height) = setup_paragraph(INFO3, *width);
    // setup sequence types code
    let (_, code3_editor, code3_height) = setup_code("./examples/sequence_types_def.rs");

    // setup user types paragraph
    const INFO4: &str = "...or put em into our own user defined types";
    let (info4_para, info4_height) = setup_paragraph(INFO4, *width);
    // setup user types code
    let (_, code4_editor, code4_height) = setup_code("./examples/user_types_def.rs");

    // defining virtual buffer for scrolling
    let buffer_height = info1_height
        + code1_height
        + info2_height
        + code2_height
        + info3_height
        + code3_height
        + info4_height
        + code4_height;
    let mut buf = Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: width.to_owned(),
        height: buffer_height as u16,
    });

    // render hai world paragraph to virtual buffer
    let mut current_y = 0;
    render_paragraph(&mut buf, info1_para, current_y, *width, info1_height);
    current_y += info1_height;
    // center hai world code
    render_code(
        &mut buf,
        code1_editor,
        current_y,
        *width,
        code1_height,
        "\"hai world!\" program",
    );
    current_y += code1_height;
    // render primitive types paragraph
    render_paragraph(&mut buf, info2_para, current_y, *width, info2_height);
    current_y += info2_height;
    // center primitive types code
    render_code(
        &mut buf,
        code2_editor,
        current_y,
        *width,
        code2_height,
        "primitive types",
    );
    current_y += code2_height;
    // render sequence types paragraph
    render_paragraph(&mut buf, info3_para, current_y, *width, info3_height);
    current_y += info3_height;
    // center sequence types code
    render_code(
        &mut buf,
        code3_editor,
        current_y,
        *width,
        code3_height,
        "sequence types",
    );
    current_y += code3_height;
    // render user types paragraph
    render_paragraph(&mut buf, info4_para, current_y, *width, info4_height);
    current_y += info4_height;
    // center user types code
    render_code(
        &mut buf,
        code4_editor,
        current_y,
        *width,
        code4_height,
        "user types",
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
