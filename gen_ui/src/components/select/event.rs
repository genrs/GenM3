use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum SelectItemEvent {
    HoverIn(SelectItemHoverIn),
    HoverOut(SelectItemHoverOut),
    Clicked(SelectItemClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct SelectItemHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct SelectItemHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct SelectItemClicked {
    pub meta: Option<FingerUpEvent>,
    pub active: bool,
    pub value: String,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum SelectEvent {
    Changed(SelectChangedEvent),
    None,
}

#[derive(Clone, Debug)]
pub struct SelectChangedEvent {
    pub meta: Option<FingerUpEvent>,
    pub value: String,
}
