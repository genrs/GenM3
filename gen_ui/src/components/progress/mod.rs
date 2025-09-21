mod prop;

use makepad_widgets::*;

pub use prop::*;

use crate::{
    ComponentAnInit,
    components::{BasicStyle, Component, LifeCycle, Style},
    error::Error,
    lifecycle, play_animation,
    prop::{
        ApplyStateMap, ProgressMode,
        manuel::{BASIC, DISABLED, LOADING},
        traits::ToFloat,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_progress::DrawProgress,
    switch_state, sync,
    themes::conf::Conf,
    utils::normalization,
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GProgressBase = {{GProgress}} {
        animator: {
            loading = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_progress: <AN_DRAW_PROGRESS> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        pressed: Forward {duration: (AN_DURATION)},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_progress: <AN_DRAW_PROGRESS> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_progress: <AN_DRAW_PROGRESS> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GProgress {
    #[live]
    pub style: ProgressStyle,
    #[live(true)]
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
            [live_id!(basic), live_id!(loading), live_id!(disabled)],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_state_map
                        .insert(ProgressState::Basic, applys);
                }
                LOADING => {
                    component
                        .apply_state_map
                        .insert(ProgressState::Loading, applys);
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

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(ProgressState::Disabled);
        }
        let style = self.style.get(self.state);
        self.draw_progress.merge(style);
        self.draw_progress.mode = self.mode;
        self.draw_progress.value = normalization(self.value, self.min, self.max);
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        ()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
        // self.redraw(cx);
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().progress;

        let live_ptr = match self.animator.live_ptr {
            Some(ptr) => ptr.file_id.0,
            None => return,
        };

        let mut registry = cx.live_registry.borrow_mut();
        let live_file = match registry.live_files.get_mut(live_ptr as usize) {
            Some(lf) => lf,
            None => return,
        };

        let nodes = &mut live_file.expanded.nodes;

        if self.lifecycle.is_created() || !init_global || self.scope_path.is_none() {
            self.lifecycle.next();
            let basic_prop = self.style.get(ProgressState::Basic);
            let loading_prop = self.style.get(ProgressState::Loading);
            let disabled_prop = self.style.get(ProgressState::Disabled);
            let (mut basic_index, mut loading_index, mut disabled_index) = (None, None, None);
            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(loading).as_instance(),
                    live_id!(off).as_instance(),
                ],
            ) {
                basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(loading).as_instance(),
                    live_id!(on).as_instance(),
                ],
            ) {
                loading_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(loading).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_progress = {
                    basic_index => {
                        background_color => basic_prop.background_color,
                        border_color =>basic_prop.border_color,
                        border_radius => basic_prop.border_radius,
                        border_width =>(basic_prop.border_width as f64),
                        shadow_color => basic_prop.shadow_color,
                        spread_radius => (basic_prop.spread_radius as f64),
                        blur_radius => (basic_prop.blur_radius as f64),
                        shadow_offset => basic_prop.shadow_offset,
                        background_visible => basic_prop.background_visible.to_f64(),
                        color => basic_prop.color,
                        value => (self.value as f64)
                    },
                    loading_index => {
                        background_color => loading_prop.background_color,
                        border_color => loading_prop.border_color,
                        border_radius => loading_prop.border_radius,
                        border_width => (loading_prop.border_width as f64),
                        shadow_color => loading_prop.shadow_color,
                        spread_radius => (loading_prop.spread_radius as f64),
                        blur_radius => (loading_prop.blur_radius as f64),
                        shadow_offset => loading_prop.shadow_offset,
                        background_visible => loading_prop.background_visible.to_f64(),
                        color => loading_prop.color,
                        value => (self.value as f64)
                    },
                    disabled_index => {
                        background_color => disabled_prop.background_color,
                        border_color => disabled_prop.border_color,
                        border_radius => disabled_prop.border_radius,
                        border_width => (disabled_prop.border_width as f64),
                        shadow_color => disabled_prop.shadow_color,
                        spread_radius => (disabled_prop.spread_radius as f64),
                        blur_radius => (disabled_prop.blur_radius as f64),
                        shadow_offset => disabled_prop.shadow_offset,
                        background_visible => disabled_prop.background_visible.to_f64(),
                        color => disabled_prop.color,
                        value => (self.value as f64)
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                ProgressState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(loading).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                ProgressState::Loading => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(loading).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                ProgressState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(loading).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_progress = {
                    index => {
                        background_color => style.background_color,
                        border_color => style.border_color,
                        border_radius => style.border_radius,
                        border_width => (style.border_width as f64),
                        shadow_color => style.shadow_color,
                        spread_radius => (style.spread_radius as f64),
                        blur_radius => (style.blur_radius as f64),
                        shadow_offset => style.shadow_offset,
                        background_visible => style.background_visible.to_f64(),
                        color => style.color,
                        value => (self.value as f64)
                    }
                }
            }
        }
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
    switch_state!();
}
