// Copyright (c) 2026 Jeremie Corbier

use enigo::{Direction, Enigo, Key, Keyboard, Settings};

pub struct InputManager {
    enigo: Option<Enigo>,
}

impl InputManager {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default()).ok();
        if enigo.is_none() {
            xdebug!("Failed to initialize Enigo input simulation.");
        }
        InputManager { enigo }
    }

    pub fn press_key(&mut self, key_name: &str) {
        if let Some(key) = parse_key(key_name) {
            if let Some(enigo) = &mut self.enigo {
                let _ = enigo.key(key, Direction::Press);
            }
        }
    }

    pub fn release_key(&mut self, key_name: &str) {
        if let Some(key) = parse_key(key_name) {
            if let Some(enigo) = &mut self.enigo {
                let _ = enigo.key(key, Direction::Release);
            }
        }
    }
}

fn parse_key(key: &str) -> Option<Key> {
    match key.to_lowercase().as_str() {
        "ctrl" | "control" => Some(Key::Control),
        "shift" => Some(Key::Shift),
        "alt" | "option" => Some(Key::Alt),
        "meta" | "command" | "windows" => Some(Key::Meta),
        "space" => Some(Key::Space),
        "enter" | "return" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        "backspace" => Some(Key::Backspace),
        "escape" | "esc" => Some(Key::Escape),
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Some(Key::Unicode(c))
        }
        _ => None,
    }
}
