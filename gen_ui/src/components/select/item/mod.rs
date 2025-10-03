mod prop;

pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{Component, GLabel, GSvg, LifeCycle},
    error::Error,
    lifecycle, play_animation,
    prop::ApplyStateMap,
    set_index, set_scope_path,
    shader::draw_view::DrawView,
    switch_state, sync,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GSelectItem = {{GSelectItem}} {}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSelectItem {
    #[live]
    pub style: SelectItemStyle,
    #[live]
    pub active: bool,
    #[live]
    pub value: String,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live]
    pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<SelectItemState>,
    #[live]
    pub draw_item: DrawView,
    // --- slot --------------------
    /// prefix icon
    #[live]
    pub icon: GSvg,
    #[live]
    pub text: GLabel,
    /// suffix icon
    #[live]
    pub suffix: GSvg,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_spread: bool,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    /// sync other state props (except related to theme) from `basic` state]
    /// means: if you set basic prop that `border_radius: 10.0`, then other state like `hover` or `pressed`
    /// will have the same `border_radius: 10.0` if you set this to true. (default is true)
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub state: SelectItemState,
}

impl WidgetNode for GSelectItem {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        todo!()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        todo!()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        todo!()
    }

    fn area(&self) -> Area {
        todo!()
    }

    fn redraw(&mut self, _cx: &mut Cx) {
        todo!()
    }
}

impl Widget for GSelectItem {
    fn draw_walk(&mut self, _cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.set_scope_path(&scope.path);
        return DrawStep::done();
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl LiveHook for GSelectItem {}

impl Component for GSelectItem {
    type Error = Error;

    type State = SelectItemState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        todo!()
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        todo!()
    }

    fn focus_sync(&mut self) -> () {
        todo!()
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
    switch_state!();
}
