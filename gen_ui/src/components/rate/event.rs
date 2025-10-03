use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum RateEvent {
    HoverIn(RateHoverIn),
    HoverOut(RateHoverOut),
    Changed(RateChanged),
    FingerUp(RateFingerUp),
    FingerDown(RateFingerDown),
    None,
}
#[derive(Debug, Clone)]
pub struct RateHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct RateHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct RateChanged {
    pub meta: RateChangedMetaEvent,
    pub value: f32,
}

#[derive(Debug, Clone)]
pub struct RateFingerUp {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct RateFingerDown {
    pub meta: FingerDownEvent,
}

#[derive(Debug, Clone)]
pub enum RateChangedMetaEvent {
    FingerUp(FingerUpEvent),
    None,
    FingerHover(FingerHoverEvent),
}

impl From<FingerUpEvent> for RateChangedMetaEvent {
    fn from(e: FingerUpEvent) -> Self {
        RateChangedMetaEvent::FingerUp(e)
    }
}

impl From<FingerHoverEvent> for RateChangedMetaEvent {
    fn from(e: FingerHoverEvent) -> Self {
        RateChangedMetaEvent::FingerHover(e)
    }
}
