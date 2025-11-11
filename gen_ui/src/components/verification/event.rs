use makepad_widgets::{ActionDefaultRef, DefaultNone};

use crate::components::InputChangedMetaEvent;

#[derive(Clone, Debug, DefaultNone)]
pub enum VerificationEvent {
    Changed(VerificationChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct VerificationChanged {
    pub meta: Option<InputChangedMetaEvent>,
    pub value: Vec<String>,
    pub length: usize,
}
