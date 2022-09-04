use std::fmt::Debug;

use log::Log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(string: String);
}

pub struct Logger<T: Debug> {
    pub label: Option<String>,
    pub level: Option<LogLevel>,
    pub message: Option<T>,
}

impl<T: Debug> Logger<T> {
    pub fn new() -> Self {
        Self {
            label: None,
            level: None,
            message: None,
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_owned());
        self
    }

    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn message(mut self, message: T) -> Self {
        self.message = Some(message);
        self
    }

    pub fn log(self) {
        if let Some(log_message) = self.message {
            let message = format!(
                "{}<{:?}>: {:?}",
                self.label.unwrap_or_default(),
                self.level.unwrap_or_default(),
                log_message
            );

            log(message);
        } else {
            log(String::from("Error, message not added"));
        }
    }
}

#[derive(Debug)]
pub enum LogLevel {
    Normal,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Normal
    }
}
