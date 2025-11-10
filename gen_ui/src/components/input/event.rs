use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum InputEvent {
    None,
    Focus(InputFocus),
    FocusLost(InputFocusLost),
    HoverIn(InputHoverIn),
    HoverOut(InputHoverOut),
    Changed(InputChanged),
    Clicked(InputClicked),
    Backspace(InputKeyDown),
    Esc(InputKeyDown),
    Returned(InputKeyDown),
    KeyDownUnhandled(InputKeyDown),
    MaxLengthReached(InputMaxLengthReached),
}

#[derive(Debug, Clone)]
pub struct InputMaxLengthReached {
    /// 当前输入框的值
    pub value: String,
    /// 用户尝试输入的新值
    pub new_input: String,
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

impl From<KeyFocusEvent> for InputFocusMetaEvent {
    fn from(e: KeyFocusEvent) -> Self {
        InputFocusMetaEvent::KeyFocus(e)
    }
}

impl From<FingerDownEvent> for InputFocusMetaEvent {
    fn from(e: FingerDownEvent) -> Self {
        InputFocusMetaEvent::FingerDown(e)
    }
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
    pub meta: InputChangedMetaEvent,
    pub value: String,
}

#[derive(Debug, Clone, Default)]
pub enum InputChangedMetaEvent {
    TextInput(TextInputEvent),
    Redo(KeyEvent),
    Undo(KeyEvent),
    Delete(KeyEvent),
    Returned(KeyEvent),
    Cut(String),
    #[default]
    None,
}

impl From<TextInputEvent> for InputChangedMetaEvent {
    fn from(e: TextInputEvent) -> Self {
        InputChangedMetaEvent::TextInput(e)
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyType {
    Backspace,
    Esc,
    Return,
    #[default]
    Other,
}

impl KeyType {
    pub fn event(&self, meta: Option<KeyEvent>, value: String) -> InputEvent {
        match self {
            KeyType::Backspace => InputEvent::Backspace(InputKeyDown { meta, value }),
            KeyType::Esc => InputEvent::Esc(InputKeyDown { meta, value }),
            KeyType::Return => InputEvent::Returned(InputKeyDown { meta, value }),
            KeyType::Other => InputEvent::KeyDownUnhandled(InputKeyDown { meta, value }),
        }
    }
}
