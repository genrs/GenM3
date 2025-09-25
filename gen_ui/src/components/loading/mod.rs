mod event;
mod prop;

pub use event::*;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    ComponentAnInit,
    components::{BasicStyle, Component, LifeCycle, Style},
    error::Error,
    lifecycle, play_animation,
    prop::{ApplyStateMap, LoadingMode, traits::ToFloat},
    set_animation, set_index, set_scope_path,
    shader::draw_loading::DrawLoading,
    switch_state, sync,
    themes::conf::Conf,
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GLoadingBase = {{GLoading}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GLoading {
    #[live]
    pub style: LoadingStyle,
    // deref -------------------
    #[live]
    pub draw_loading: DrawLoading,
    #[live]
    pub mode: LoadingMode,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    // frame -------------------
    #[live]
    pub time: f32,
    #[rust]
    next_frame: NextFrame,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<LoadingState>,
    #[rust]
    pub state: LoadingState,
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
    pub loading: bool,
}

impl WidgetNode for GLoading {
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
        self.draw_loading.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_loading.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GLoading {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_loading.begin(cx, walk, style.layout());
        self.draw_loading.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.animation_open || !self.visible {
            return;
        }
        if let Some(ne) = self.next_frame.is_event(event) {
            // update time to use for animation
            self.time = (ne.time * 0.1).fract() as f32;
            // force updates, so that we can animate in the absence of user-generated events
            self.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
    }
}

impl LiveHook for GLoading {
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.sync();
        self.render_after_apply(cx);
        // starts the animation cycle on startup
        if self.animation_open {
            self.next_frame = cx.new_next_frame();
        }
    }
}

impl Component for GLoading {
    type Error = Error;

    type State = LoadingState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.loading;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(LoadingState::Disabled);
        }
        let style = self.style.get(self.state);
        self.draw_loading.merge(style);
        self.draw_loading.mode = self.mode;
        self.draw_loading.loading = self.loading.to_f32();
        Ok(())
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
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
        let init_global = cx.global::<ComponentAnInit>().loading;

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
            let basic_prop = self.style.get(LoadingState::Basic);
            let loading_prop = self.style.get(LoadingState::Loading);
            let disabled_prop = self.style.get(LoadingState::Disabled);
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
                nodes: draw_loading = {
                    basic_index => {
                        color => basic_prop.color,
                        loading => self.loading.to_f64()
                    },
                    loading_index => {
                        color => loading_prop.color,
                        loading => self.loading.to_f64()
                    },
                    disabled_index => {
                        color => disabled_prop.color,
                        loading => self.loading.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                LoadingState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(loading).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                LoadingState::Loading => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(loading).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                LoadingState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(loading).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_loading = {
                    index => {
                        color => style.color,
                        loading => self.loading.to_f64()
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
