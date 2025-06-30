use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Block, Widget},
};
use tui_textarea::{CursorMove, TextArea};

#[derive(Clone)]
pub struct Search {
    textarea: TextArea<'static>,
}

impl Default for Search {
    fn default() -> Self {
        let block = Block::bordered().title("Search");
        let mut textarea = TextArea::default();
        textarea.set_block(block);
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("City, State, or Zip Code");
        Self { textarea }
    }
}

impl Widget for Search {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render the textarea as usual
        self.textarea.render(area, buf);
        // If loading, render throbber to the right inside the input box
    }
}

impl Search {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        _ = match (key_event.code, key_event.modifiers) {
            (KeyCode::Char(' '), KeyModifiers::CONTROL) | (KeyCode::Char('/'), _) => {
                self.clear_text();
                true
            }
            (KeyCode::Enter, _) => true,
            _ => {
                if self.text().len() <= 100 {
                    self.textarea.input(key_event)
                } else {
                    true
                }
            }
        };
        ()
    }

    pub fn text(&self) -> String {
        self.textarea.lines()[0].to_string()
    }

    pub fn clear_text(&mut self) {
        self.textarea.move_cursor(CursorMove::Head);
        self.textarea.delete_str(self.text().len());
    }
}
