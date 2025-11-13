use std::time::Duration;

use color_eyre;
use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::model::{Model, RunningState};

// update handling with a message for each action/event (logic)
#[derive(PartialEq)]
pub enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
}

pub fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('h') => Some(Message::Reset),
        KeyCode::Char('j') => Some(Message::Increment),
        KeyCode::Char('k') => Some(Message::Decrement),
        KeyCode::Char('l') => Some(Message::Reset),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    // match all possible messages and return new model reflecting changes
    match msg {
        Message::Increment => {
            model.y_pos += 1;
        }
        Message::Decrement => {
            if model.y_pos > 0 {
                model.y_pos -= 1;
            }
        }
        Message::Reset => model.y_pos = 0,
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
    };
    None
}
