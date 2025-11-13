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
    // initial wait for event and full bypass if none
    if !event::poll(Duration::from_millis(69))? {
        return Ok(None);
    }

    // store only one event message
    let mut last_message = None;

    // capture expected event
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            last_message = handle_key(key);
        }
    }

    // drain que of all additional events
    while event::poll(Duration::ZERO)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if let Some(msg) = handle_key(key) {
                    // overwrite wit most recent message
                    last_message = Some(msg);
                }
            }
        }
    }
    Ok(last_message)
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
            model.y_pos = (model.y_pos + 1).min(model.max_scroll);
        }
        Message::Decrement => {
            model.y_pos = model.y_pos.saturating_sub(1);
        }
        Message::Reset => model.y_pos = 0,
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
    };
    None
}
