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

// #[derive(Debug, Clone, DefaultNone)]
// pub enum SelectItemGroupEvent {
//     Changed(SelectItemGroupChanged),
//     None,
// }

// #[derive(Clone, Debug)]
// pub struct SelectItemGroupChanged {
//     pub meta: Option<FingerUpEvent>,
//     /// The index of the active radio.
//     pub index: i32,
//     /// The value of the active radio.
//     pub value: Option<String>,
// }
