use std::cell::RefCell;

use makepad_widgets::*;

use crate::{
    components::{DrawState, LifeCycle, ViewTextureCache},
    shader::draw_view::DrawView,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GContainerBase = {{GContainer}} {}
}

/// # Container
/// container is a lowest component which can use to expand each view component, such as: `View`, `Badge`
/// - it can hold children widgets
/// - it has no specific `style`, `state`, etc.
/// - it use for inherits
#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
#[allow(dead_code)]
pub struct GContainer {
    // --- other props ------------
    #[live(true)]
    pub visible: bool,
    #[live]
    pub scroll: DVec2,
    #[live]
    pub scroll_bars: Option<LivePtr>,
    #[live]
    pub dpi_factor: Option<f64>,
    #[live]
    pub optimize: ViewOptimize,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub block_signal_event: bool,
    #[live(false)]
    pub capture_overload: bool,
    #[live]
    pub event_order: EventOrder,
    #[live(false)]
    pub event_open: bool,
    #[live]
    pub disabled: bool,
    // --- texture and cache ------
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub find_cache: RefCell<SmallVec<[(u64, WidgetSet); 3]>>,
    #[rust]
    pub scroll_bars_obj: Option<Box<ScrollBars>>,
    #[rust]
    pub view_size: Option<DVec2>,
    #[rust]
    pub area: Area,
    #[rust]
    pub draw_list: Option<DrawList2d>,
    #[rust]
    pub texture_cache: Option<ViewTextureCache>,
    #[rust]
    pub defer_walks: SmallVec<[(LiveId, DeferWalk); 1]>,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    #[rust]
    pub children: SmallVec<[(LiveId, WidgetRef); 2]>,
    #[rust]
    pub live_update_order: SmallVec<[LiveId; 1]>,
    // --- animation --------------
    #[animator]
    pub animator: Animator,
    #[live(false)]
    pub animation_open: bool,
    #[live(true)]
    pub animation_spread: bool,
    // --- draw -------------------
    #[live]
    pub draw_view: DrawView,
    // --- lifecycle --------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub index: usize,
    #[live(true)]
    pub sync: bool,
}

impl LiveHook for GContainer {}

impl WidgetNode for GContainer {
    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        Walk::default()
    }

    fn area(&self) -> Area {
        self.area
    }

    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for (_, child) in &self.children {
            let x = child.uid_to_widget(uid);
            if !x.is_empty() {
                return x;
            }
        }
        WidgetRef::empty()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx);
        self.draw_view.redraw(cx);
        for (_, child) in &mut self.children {
            child.redraw(cx);
        }
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {}
}

impl Widget for GContainer {}
