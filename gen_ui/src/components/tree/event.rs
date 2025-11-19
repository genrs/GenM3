use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum LeafEvent {
    HoverIn(LeafHoverIn),
    HoverOut(LeafHoverOut),
    Clicked(LeafClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct LeafHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct LeafHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct LeafClicked {
    pub meta: Option<FingerUpEvent>,
    pub active: bool,
    pub value: String,
}

#[derive(Debug, Clone, DefaultNone)]
pub enum BranchEvent {
    Changed(BranchChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct BranchChanged {
    pub active: bool,
    pub value: String,
    pub meta: Option<FingerUpEvent>,
}

#[derive(Debug, Clone)]
pub enum TreeActionType {
    Branch(BranchChanged),
    Leaf(LeafClicked),
}

#[derive(Debug, Clone, DefaultNone)]
pub enum TreeEvent {
    HoverIn(TreeHoverIn),
    HoverOut(TreeHoverOut),
    Changed(TreeChanged),
    None,
}

#[derive(Clone, Debug)]
pub struct TreeHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct TreeHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct TreeChanged {
    pub meta: Option<FingerUpEvent>,
    /// The value of the active
    pub actives: Vec<String>,
}
