use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum PaginationEvent {
    Changed(PaginationChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct PaginationChanged {
    pub meta: Option<FingerUpEvent>,
    pub current: usize,
    pub page_size: usize,
}
