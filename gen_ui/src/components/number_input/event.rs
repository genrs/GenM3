use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerUpEvent};

use crate::components::InputChangedMetaEvent;

#[derive(Clone, Debug, DefaultNone)]
pub enum NumberInputEvent {
    Changed(NumberInputChanged),
    None,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum NumberCtrEvent {
    Up(NumberCtrClicked),
    Down(NumberCtrClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct NumberCtrClicked {
    pub meta: Option<FingerUpEvent>,
}



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
