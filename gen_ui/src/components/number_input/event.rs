use makepad_widgets::{ActionDefaultRef, DefaultNone};

use crate::components::InputChangedMetaEvent;

#[derive(Clone, Debug, DefaultNone)]
pub enum NumberInputEvent {
    Changed(NumberInputChanged),
    None,
}

// #[derive(Clone, Debug, DefaultNone)]
// pub enum NumberCtrEvent {
//     Up(NumberCtrUp),
//     Down(NumberCtrDown),
//     None,
// }

// #[derive(Clone, Debug)]
// pub struct NumberCtrUp {
//     pub meta: Option<>,
    
// }

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum NumberInputAdjust {
    Up,
    Down,
    Clear,
}

#[derive(Clone, Debug)]
pub struct NumberInputChanged {
    pub meta: Option<InputChangedMetaEvent>,
    pub value: f32,
    pub adjust: NumberInputAdjust,
}
