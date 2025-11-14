// temp compatibility wrapper for ratatui-v0.30.0-beta.0
// basically just needa force ratatui-code-editor to switch from 'Widget for &T' to 'Widget for T'
// (incredible lib name btw)

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};
use ratatui_code_editor::code::{Code, RopeGraphemes, grapheme_width_and_bytes_len};
use std::cell::RefCell;
use std::collections::HashMap;

// for mapping theme to ratatui styles
type Theme = HashMap<String, Style>;

// represent styled span: (start, end, style)
type Highlight = (usize, usize, Style);

// cache to store completed highlights
type HighlightCache = HashMap<(usize, usize), Vec<Highlight>>;

// editor wit shared cache mutation
pub struct EditorWidget {
    code: Code,
    theme: Theme,
    highlights_cache: RefCell<HighlightCache>,
}

impl EditorWidget {
    // create editor widget wit fallback to basic text
    pub fn new(language: &str, content: &str, theme: Vec<(&str, &str)>) -> Self {
        // try to create a code instance wit tree-sitter parsing
        let code = Code::new(content, language)
            .or_else(|_| Code::new(content, "text"))
            .unwrap();

        // setup theme wit ratatui styles
        let theme = Self::build_theme(&theme);

        let highlights_cache = RefCell::new(HashMap::new());
        Self {
            code,
            theme,
            highlights_cache,
        }
    }

    // convert hex strings into ratatui rgb colours
    fn build_theme(theme: &[(&str, &str)]) -> Theme {
        let mut result = HashMap::new();

        // parse each name-hex pair
        for (name, hex_color) in theme {
            if let Ok(color) = hex_color.parse::<csscolorparser::Color>() {
                let [r, g, b, _] = color.to_rgba8();
                result.insert(name.to_string(), Style::default().fg(Color::Rgb(r, g, b)));
            }
        }
        result
    }

    // get syntax highlights for given byte range, using cache where possible
    fn highlight_interval(&self, start: usize, end: usize, theme: &Theme) -> Vec<Highlight> {
        let mut cache = self.highlights_cache.borrow_mut();

        // check if already computed highlights for dis range
        if let Some(cached) = cache.get(&(start, end)) {
            return cached.clone();
        }

        // parse and highlight range wen missing from cache
        let highlights = self.code.highlight_interval(start, end, theme);
        cache.insert((start, end), highlights.clone());
        highlights
    }
}

// widget implementation to comply wit 0.30.0-beta.0
impl Widget for EditorWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // first pass: setup and draw base text wit line numbers
        let code = &self.code;
        let total_lines = code.len_lines();
        let total_chars = code.len_chars();
        // calculate space required for line numbers
        let max_line_number = total_lines.max(1);
        let line_number_digits = max_line_number.to_string().len().max(5);
        let line_number_width = line_number_digits + 2;
        // current y coord
        let mut draw_y = area.top();
        // styles for different parts of display
        let line_number_style = Style::default().fg(Color::DarkGray);
        let default_text_style = Style::default().fg(Color::White);
        // draw each line
        for line_idx in 0..total_lines {
            if draw_y >= area.bottom() {
                break;
            }
            // draw line number
            let line_number = format!("{:^width$}", line_idx + 1, width = line_number_digits);
            buf.set_string(area.left(), draw_y, &line_number, line_number_style);
            // determine how many characters can fit
            let line_len = code.line_len(line_idx);
            let max_x = (area.width as usize).saturating_sub(line_number_width);
            // start at column 0 since no horizontal scrolling
            let start_col = 0;
            let end_col = (start_col + max_x).min(line_len);
            // convert line + column coords to absolute character positions
            let line_start_char = code.line_to_char(line_idx);
            let char_start = line_start_char + start_col;
            let char_end = line_start_char + end_col;
            // extract visible part of dis line
            let visible_chars = code.char_slice(char_start, char_end);
            let displayed_line = visible_chars.to_string().replace("\t", "    ");
            // draw code text
            let text_x = area.left() + line_number_width as u16;
            if text_x < area.left() + area.width && draw_y < area.top() + area.height {
                buf.set_string(text_x, draw_y, &displayed_line, default_text_style);
            }
            draw_y += 1;
        }
        // second pass: overlay syntax highlighting
        if code.is_highlight() {
            // process each visible line
            for screen_y in 0..(area.height as usize) {
                let line_idx = screen_y;
                if line_idx >= total_lines {
                    break;
                }

                // rerun ccalculations from first pass
                let line_len = code.line_len(line_idx);
                let max_x = (area.width as usize).saturating_sub(line_number_width);
                let line_start_char = code.line_to_char(line_idx);
                let start_char = line_start_char;
                let visible_len = line_len;
                let end = max_x.min(visible_len);
                let end_char = start_char + end;

                // safety check
                if start_char > total_chars || end_char > total_chars {
                    continue;
                }

                // visible characters for dis line
                let chars = code.char_slice(start_char, end_char);

                // convert to byte for tree-sitter
                let start_byte = code.char_to_byte(start_char);
                let end_byte = code.char_to_byte(end_char);

                // get syntax tokens in byte range
                let highlights = self.highlight_interval(start_byte, end_byte, &self.theme);

                // iterate through visible characters and apply colors
                let mut x = 0;
                let mut byte_idx_in_rope = start_byte;

                // handle graphemes
                for g in RopeGraphemes::new(&chars) {
                    // get dis grapheme's display width and byte length
                    let (g_width, g_bytes) = grapheme_width_and_bytes_len(g);

                    if x >= max_x {
                        break;
                    }

                    // calculate screen coordinates for dis grapheme
                    let start_x = area.left() + line_number_width as u16 + x as u16;
                    let draw_y = area.top() + screen_y as u16;

                    // color all cells dat dis grapheme occupies
                    for dx in 0..g_width {
                        if x + dx >= max_x {
                            break;
                        }
                        let draw_x = start_x + dx as u16;

                        // check which highlight range dis byte falls into
                        for &(start, end, s) in &highlights {
                            if start <= byte_idx_in_rope && byte_idx_in_rope < end {
                                buf[(draw_x, draw_y)].set_style(s);
                                break;
                            }
                        }
                    }

                    // move to next grapheme
                    x = x.saturating_add(g_width);
                    byte_idx_in_rope += g_bytes;
                }
            }
        }
    }
}
