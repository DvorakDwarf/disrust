use crate::ui::channels::{App, DisplayMode};
use crate::api::wrapper;

#[derive(PartialEq, Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

// ChatBox holds the state of the chat box
pub struct ChatBox {
    // Current value of the input box
    pub input: String,
    // Current input mode
    pub input_mode: InputMode,
}

impl ChatBox {
    pub fn new() -> ChatBox {
        ChatBox {
            input: String::new(),
            input_mode: InputMode::Normal,
        }
    }

    //Toggles input mode
    pub fn toggle(&mut self) {
        if self.input_mode == InputMode::Normal {
            self.input_mode = InputMode::Editing;
        } else {
            self.input_mode = InputMode::Normal
        }
    }

    //Sends message from the chat box, clears it
    pub fn send_message(&mut self, app: &mut App) {
        match app.mode {
            DisplayMode::GuildMode => {self.input_mode = InputMode::Normal},
            DisplayMode::ChannelMode => {
                //Here so messages dissappear instantly
                let input_copy = self.input.clone();
                self.input.clear();
                wrapper::send_message(app, &input_copy);
            },
        }
    }
}