// heavily copied from le rat gods over at ratatui...
use crate::ascii::RAT;
use crate::theme::*;
use itertools::Itertools;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

const FULL: char = '█';
const MOST: char = '▓';
const HALF: char = '▒';
const QUARTER: char = '░';
const ALT: char = 'h';
const EMPTY: char = ' ';

// widget to render ascii
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiArt {
    ascii: String,
    should_squash: bool,
    full_colour: Color,
    most_colour: Color,
    half_colour: Color,
    quarter_colour: Color,
    alt_colour: Color,
}

impl Default for AsciiArt {
    fn default() -> Self {
        Self {
            ascii: RAT.to_owned(),
            should_squash: false,
            full_colour: build_style(WHITE),
            most_colour: build_style(PINK3),
            half_colour: Color::Indexed(237),
            quarter_colour: build_style(CYAN2),
            alt_colour: Color::Indexed(196),
        }
    }
}

impl AsciiArt {
    // create new artwork widget
    pub fn new(ascii: String) -> Self {
        Self {
            ascii: ascii,
            ..Default::default()
        }
    }

    const fn colour_for(&self, c: char) -> Option<Color> {
        match c {
            FULL => Some(self.full_colour),
            MOST => Some(self.most_colour),
            HALF => Some(self.half_colour),
            QUARTER => Some(self.quarter_colour),
            ALT => Some(self.alt_colour),
            _ => None,
        }
    }
}

impl Widget for AsciiArt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }

        if !self.should_squash {
            for (y, line) in self.ascii.lines().enumerate() {
                for (x, ch) in line.chars().enumerate() {
                    let x = area.left() + x as u16;
                    let y = area.top() + y as u16;
                    // check if coords within buffer area
                    if x >= area.right() || y >= area.bottom() {
                        continue;
                    }
                    let cell = &mut buf[(x, y)];
                    let colour = match ch {
                        EMPTY => None,
                        c => self.colour_for(c),
                    };
                    let symbol = match ch {
                        EMPTY => None,
                        _ => Some('█'),
                    };
                    if let Some(colour) = colour {
                        cell.fg = colour;
                    }
                    if let Some(symb) = symbol {
                        cell.set_char(symb);
                    }
                }
            }
        } else {
            for (y, (line1, line2)) in self.ascii.lines().tuples().enumerate() {
                for (x, (ch1, ch2)) in line1.chars().zip(line2.chars()).enumerate() {
                    let x = area.left() + x as u16;
                    let y = area.top() + y as u16;

                    // check if coords within buffer area
                    if x >= area.right() || y >= area.bottom() {
                        continue;
                    }

                    let cell = &mut buf[(x, y)];
                    let (fg, bg) = match (ch1, ch2) {
                        (EMPTY, EMPTY) => (None, None),
                        (c, EMPTY) | (EMPTY, c) => (self.colour_for(c), None),
                        (c1, c2) => (self.colour_for(c1), self.colour_for(c2)),
                    };
                    // symbol should make the empty space or terminal bg as the empty part of the block
                    let symbol = match (ch1, ch2) {
                        (EMPTY, EMPTY) => None,
                        (_, EMPTY) => Some('▀'),
                        (EMPTY, _) => Some('▄'),
                        (c, d) if c == d => Some('█'),
                        (_, _) => Some('▀'),
                    };
                    if let Some(fg) = fg {
                        cell.fg = fg;
                    }
                    if let Some(bg) = bg {
                        cell.bg = bg;
                    }
                    if let Some(symb) = symbol {
                        cell.set_char(symb);
                    }
                }
            }
        }
    }
}
