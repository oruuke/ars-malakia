use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
};
use std::collections::HashMap;

// basics
pub const WHITE: &str = "#FFFFFF";
// pinks
pub const PINK1: &str = "#FFAFFF";
pub const PINK2: &str = "#DC8BB2";
pub const PINK3: &str = "#B55088";
// cyans
//pub const CYAN1: &str = "#99CCCC";
pub const CYAN2: &str = "#007F7F";
//pub const CYAN3: &str = "#004C4C";
// other
pub const ROSE1: &str = "#DD4042";
pub const GOLD1: &str = "#FCDD6C";
pub const PURP1: &str = "#EA00F7";

// for mapping theme to ratatui styles
pub type Theme = HashMap<String, Style>;

// convert hex strings into ratatui rgb colours
pub fn build_theme(hex_colours: &[(&str, &str)]) -> Theme {
    let mut result_styles = HashMap::new();

    // parse each name-hex pair
    for (name, hex) in hex_colours {
        if let Ok(color) = hex.parse::<csscolorparser::Color>() {
            let [r, g, b, _] = color.to_rgba8();
            result_styles.insert(name.to_string(), Style::default().fg(Color::Rgb(r, g, b)));
        }
    }
    result_styles
}

pub fn build_style(hex: &str) -> Color {
    let style = hex.parse::<csscolorparser::Color>().unwrap();
    let [r, g, b, _] = style.to_rgba8();
    Color::Rgb(r, g, b)
}

pub fn tree_sitter() -> Vec<(&'static str, &'static str)> {
    vec![
        ("keyword", PINK3),
        ("string", GOLD1),
        ("comment", CYAN2),
        ("function", PURP1),
        ("variable", PINK1),
        ("namespace", PINK2),
        ("type", ROSE1),
    ]
}

pub fn center(area: &Rect, width: u16, height: u16) -> Rect {
    let [horizontal] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area.to_owned());

    let [centered] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(horizontal);

    centered
}
