mod prop;

use makepad_widgets::*;

pub use prop::*;

use crate::{
    components::{BasicStyle, Component, LifeCycle, Style}, error::Error, prop::{
        manuel::{BASIC, DISABLED, IN_PROGRESS}, ApplyStateMap, Direction, ProgressMode
    }, pure_after_apply, shader::draw_progress::DrawProgress, themes::conf::Conf, visible
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GProgress = {{GProgress}} {

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GProgress {
    #[live]
    pub style: ProgressStyle,
    #[live]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    #[live]
    pub mode: ProgressMode,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<ProgressState>,
    #[rust]
    pub state: ProgressState,
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
    #[live(true)]
    pub sync: bool,
    #[live]
    pub draw_progress: DrawProgress,
    #[live(0.0)]
    pub min: f32,
    #[live(1.0)]
    pub max: f32,
    #[live(0.0)]
    pub value: f32,
}

impl WidgetNode for GProgress {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        return WidgetRef::empty();
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_progress.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_progress.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GProgress {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let style = self.style.get(self.state);
        let _ = self.draw_progress.begin(cx, walk, style.layout());
        let _ = self.draw_progress.end(cx);
        self.set_scope_path(&scope.path);
        return DrawStep::done();
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl LiveHook for GProgress {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &ProgressBasicStyle::live_props(),
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(pressed),
                live_id!(disabled),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_state_map
                        .insert(ProgressState::Basic, applys);
                }
                IN_PROGRESS => {
                    component
                        .apply_state_map
                        .insert(ProgressState::InProgress, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(ProgressState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GProgress {
    type Error = Error;

    type State = ProgressState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.progress;
        self.style = style.clone();
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
