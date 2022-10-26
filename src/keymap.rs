use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct ExKeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl ExKeyEvent {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

impl PartialEq for ExKeyEvent {
    fn eq(&self, other: &Self) -> bool {
        let ev: KeyEvent = self.into();
        let other: KeyEvent = other.into();
        ev == other
    }
}

impl From<&ExKeyEvent> for KeyEvent {
    fn from(other: &ExKeyEvent) -> Self {
        Self::new(other.code, other.modifiers)
    }
}

pub struct ExKeyList {
    pub up: ExKeyEvent,
    pub down: ExKeyEvent,
    pub quit: ExKeyEvent,
}

pub fn key_match(key: &KeyEvent, bind: &ExKeyEvent) -> bool {
    key.code == bind.code && key.modifiers == bind.modifiers
}

impl Default for ExKeyList {
    fn default() -> Self {
        Self {
            up: ExKeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty()),
            down: ExKeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty()),
            quit: ExKeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty()),
        }
    }
}
