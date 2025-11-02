mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{BasicStyle, Component, GButton, LifeCycle, Style},
    error::Error,
    lifecycle, play_animation,
    prop::ApplyStateMap,
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    switch_state, sync,
    themes::conf::Conf,
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GPaginationBase = {{GPagination}} {}
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
    pub draw_pagination: DrawView,
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
    #[live(true)]
    pub animation_spread: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub state: PaginationState,
    // --- pagination
    #[live]
    pub total: usize,
    #[live]
    pub current: usize,
}

impl WidgetNode for GPagination {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_pagination.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_pagination.redraw(cx);
        for button in [&mut self.prefix, &mut self.suffix] {
            if button.visible {
                button.redraw(cx);
            }
        }
        for (_, item) in self.item.iter_mut() {
            if item.visible {
                item.redraw(cx);
            }
        }
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GPagination {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_pagination.begin(cx, walk, style.layout());
        if self.prefix.visible {
            let walk = self.prefix.walk(cx);
            let _ = self.prefix.draw_walk(cx, scope, walk);
        }
        for (_id, item) in self.item.iter_mut() {
            if item.visible {
                let walk = item.walk(cx);
                let _ = item.draw_walk(cx, scope, walk);
            }
        }
        if self.suffix.visible {
            let walk = self.suffix.walk(cx);
            let _ = self.suffix.draw_walk(cx, scope, walk);
        }
        self.draw_pagination.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl LiveHook for GPagination {
    pure_after_apply!();
}

impl Component for GPagination {
    type Error = Error;

    type State = PaginationState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.pagination;
        self.style = style.clone();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(PaginationState::Disabled);
        }
        let style = self.style.get(self.state).container;
        self.draw_pagination.merge(&style);

        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, PaginationState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        ()
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }
    fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
        ()
    }

    sync!();
    // play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
    switch_state!();
}
