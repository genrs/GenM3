use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum ColorPanelEvent {
    Changed(ColorPanelChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct ColorPanelChanged {
    pub meta: Option<FingerUpEvent>,
    pub color: String,
}
