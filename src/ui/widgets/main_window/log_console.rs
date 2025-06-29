use std::collections::VecDeque;

use iced::{
    widget::{container, scrollable, text, Column, Container},
    Color, Length,
};

use crate::ui::message::MainWindowMessage;

#[derive(Debug, Clone)]
enum LogConsoleInput {
    Normal(String),
    Warning(String),
    Error(String),
}

#[derive(Debug)]
pub struct LogConsole {
    logs_input: VecDeque<LogConsoleInput>,
    max_inputs: usize,
    font_size: u16,
}

impl LogConsole {
    pub fn new(max_inputs: usize) -> Self {
        Self {
            logs_input: VecDeque::with_capacity(max_inputs),
            max_inputs,
            font_size: 12,
        }
    }

    pub fn log_message(&mut self, msg: &str) {
        self.logs_input
            .push_back(LogConsoleInput::Normal(msg.to_string()));

        while self.logs_input.len() > self.max_inputs {
            let _ = self.logs_input.pop_front();
        }
    }

    pub fn log_warning(&mut self, msg: &str) {
        self.logs_input
            .push_back(LogConsoleInput::Warning(msg.to_string()));

        while self.logs_input.len() > self.max_inputs {
            let _ = self.logs_input.pop_front();
        }
    }

    pub fn log_error(&mut self, msg: &str) {
        self.logs_input
            .push_back(LogConsoleInput::Error(msg.to_string()));

        while self.logs_input.len() > self.max_inputs {
            let _ = self.logs_input.pop_front();
        }
    }

    pub fn view<'a>(&'a self) -> Container<'a, MainWindowMessage> {
        let mut texts_col = Column::new();

        for entry in &self.logs_input {
            texts_col = texts_col.push(match entry {
                LogConsoleInput::Normal(msg) => text(msg).size(self.font_size).color(Color::WHITE),
                LogConsoleInput::Warning(msg) => text(msg)
                    .size(self.font_size)
                    .color(Color::from_rgb(0.99, 0.66, 0.01)),
                LogConsoleInput::Error(msg) => text(msg)
                    .size(self.font_size)
                    .color(Color::from_rgb(0.93, 0.40, 0.29)),
            });
        }

        container(
            scrollable(texts_col)
                .width(Length::Fill)
                .height(Length::Fill)
                .anchor_bottom(),
        )
        .style(|s| container::Style {
            background: Some(iced::Background::Color(Color::BLACK)),
            text_color: Some(Color::WHITE),
            ..container::rounded_box(s)
        })
        .padding(5)
    }
}

impl Default for LogConsole {
    fn default() -> Self {
        Self::new(100)
    }
}
