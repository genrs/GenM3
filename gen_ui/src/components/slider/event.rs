use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum SliderEvent {
    HoverIn(SliderHoverIn),
    HoverOut(SliderHoverOut),
    FingerUp(SliderFingerUp),
    FingerDown(SliderFingerDown),
    Changed(SliderChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct SliderHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct SliderHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct SliderChanged {
    pub meta: Option<FingerUpEvent>,
    pub value: f64,
    pub step: f64,
    pub range: [f64; 2],
}

#[derive(Debug, Clone)]
pub struct SliderFingerUp {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct SliderFingerDown {
    pub meta: FingerDownEvent,
}
