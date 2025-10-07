use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum TextInputEvent {
    None,
    KeyFocus,
    KeyFocusLost,
    Returned(String),
    Escaped,
    Changed(String),
    KeyDownUnhandled(KeyEvent),
}