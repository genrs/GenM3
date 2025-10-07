use makepad_widgets::{
    event::FingerLongPressEvent, ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent,
    FingerMoveEvent, FingerUpEvent, KeyEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum BadgeEvent {
    FingerDown(BadgeFingerDown),
    FingerUp(BadgeFingerUp),
    LongPress(BadgeLongPress),
    Move(BadgeMove),
    HoverIn(BadgeHoverIn),
    HoverOut(BadgeHoverOut),
    HoverOver(BadgeHoverOver),
    KeyDown(BadgeKeyDown),
    KeyUp(BadgeKeyUp),
    Clicked(BadgeClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct BadgeClicked {
    pub meta: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeKeyDown {
    pub meta: KeyEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeKeyUp {
    pub meta: KeyEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeFingerDown {
    pub meta: FingerDownEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeFingerUp {
    pub meta: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeLongPress {
    pub meta: FingerLongPressEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeMove {
    pub meta: FingerMoveEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeHoverOver {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct BadgeHoverOut {
    pub meta: FingerHoverEvent,
}
