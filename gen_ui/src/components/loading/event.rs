use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum LoadingEvent {
    Changed(LoadingChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct LoadingChanged {
    pub value: bool,
}