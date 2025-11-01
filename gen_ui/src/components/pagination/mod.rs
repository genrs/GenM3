mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{Component, GButton, LifeCycle}, error::Error, prop::ApplyStateMap, pure_after_apply, shader::draw_view::DrawView
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GPagination = {{GPagination}} {}
}

/// # Pagination
///
/// ## Display
/// ```md
/// -------------------------------
/// | < | item | item | item | > |  extra
/// -------------------------------
/// - < : prefix button
/// - > : suffix button
/// - item : page button
/// - extra : can be used to show total pages info // TODO
/// ```
#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GPagination {
    #[live]
    pub style: PaginationStyle,
    #[live]
    pub prefix: GButton,
    #[live]
    pub suffix: GButton,
    #[rust]
    pub item: Vec<(LiveId, GButton)>,
    // #[live] TODO
    // pub extra: GView,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    #[live]
    pub draw_options: DrawView,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub disabled: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<PaginationState>,
    #[rust]
    pub index: usize,
    #[rust(true)]
    pub sync: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub state: PaginationState,
}

impl WidgetNode for GPagination {
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

impl Widget for GPagination {
    fn draw_walk(&mut self, _cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
        
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        
    }
}

impl LiveHook for GPagination {
    pure_after_apply!();
}

impl Component for GPagination {
    type Error = Error;

    type State = PaginationState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_scope_path(&mut self, path: &HeapLiveIdPath) -> () {
        todo!()
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
        todo!()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        todo!()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        todo!()
    }

    fn sync(&mut self) -> () {
        todo!()
    }

    fn focus_sync(&mut self) -> () {
        todo!()
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn lifecycle(&self) -> LifeCycle {
        todo!()
    }

    fn set_index(&mut self, index: usize) -> () {
        todo!()
    }
}
