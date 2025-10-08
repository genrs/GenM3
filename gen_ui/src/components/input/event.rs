use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum InputEvent {
    None,
    // KeyFocus,
    // KeyFocusLost,
    // Returned(String),
    // Escaped,
    // Changed(String),
    // KeyDownUnhandled(KeyEvent),
    Focus(InputFocus),
    FocusLost(InputFocusLost),
    HoverIn(InputHoverIn),
    HoverOut(InputHoverOut),
    Changed(InputChanged),
    Clicked(InputClicked),
    Backspace(InputKeyDown),
    Esc(InputKeyDown),
    Return(InputKeyDown),
    KeyDownUnhandled(InputKeyDown),
}

#[derive(Debug, Clone)]
pub struct InputFocus {
    pub meta: InputFocusMetaEvent,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum InputFocusMetaEvent {
    KeyFocus(KeyFocusEvent),
    FingerDown(FingerDownEvent),
    None,
}

#[derive(Debug, Clone)]
pub struct InputFocusLost {
    pub meta: Option<KeyFocusEvent>,
}

#[derive(Debug, Clone)]
pub struct InputHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct InputHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct InputChanged {
    pub meta: Option<TextInputEvent>,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct InputClicked {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct InputKeyDown {
    pub meta: Option<KeyEvent>,
    pub value: String,
}
