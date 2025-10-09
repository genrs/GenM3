mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    ComponentAnInit, active_event, animation_open_then_redraw,
    components::{BasicStyle, Component, LifeCycle, Style},
    error::Error,
    event_option, hit_finger_down, hit_finger_up, lifecycle, play_animation,
    prop::{
        ApplyStateMap,
        manuel::{BASIC, DISABLED, HOVER, PRESSED},
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_rate::DrawRate,
    switch_state, sync,
    themes::conf::Conf,
    utils::{round_step},
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GRateBase = {{GRate}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION_NO)}},
                    ease: InOutQuad,
                    apply: {
                        draw_rate: <AN_DRAW_RATE> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION_NO),},
                        pressed: Forward {duration: (AN_DURATION_NO)},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_rate: <AN_DRAW_RATE> {}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (AN_DURATION_NO)}},
                    ease: InOutQuad,
                    apply: {
                        draw_rate: <AN_DRAW_RATE> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION_NO)}},
                    ease: InOutQuad,
                    apply: {
                        draw_rate: <AN_DRAW_RATE> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GRate {
    #[live]
    pub style: RateStyle,
    /// number of stars
    #[live(5)]
    pub count: i32,
    /// The proportion of filled stars [0, count] step is 1 / 0.5 depending on allow_half
    #[live(0.0)]
    pub value: f32,
    #[live(true)]
    pub allow_half: bool,
    #[live]
    pub tmp_value: Option<f32>,
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
    pub apply_state_map: ApplyStateMap<RateState>,
    #[live]
    pub draw_rate: DrawRate,
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
    pub state: RateState,
}

impl WidgetNode for GRate {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        let mut walk = style.walk();
        let mut w = if walk.height.is_fixed() {
            walk.height.fixed_or_zero()
        } else {
            18.0
        };
        if self.count >= 0 {
            w = w * self.count as f64 + style.spacing * (self.count - 1) as f64;
        } else {
            panic!("count of rate should not be zero");
        }
        walk.width = Size::Fixed(w);
        walk
    }

    fn area(&self) -> Area {
        self.draw_rate.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_rate.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GRate {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let style = self.style.get(self.state);
        let _ = self.draw_rate.begin(cx, walk, style.layout());
        let _ = self.draw_rate.end(cx);

        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().rate = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl LiveHook for GRate {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            apply.from,
            nodes,
            index,
            &RateBasicStyle::live_props(),
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(pressed),
                live_id!(disabled),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(RateState::Basic, applys);
                }
                HOVER => {
                    component.apply_state_map.insert(RateState::Hover, applys);
                }
                PRESSED => {
                    component.apply_state_map.insert(RateState::Pressed, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(RateState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GRate {
    type Error = Error;

    type State = RateState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.rate;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(RateState::Disabled);
        }
        let value = round_step(self.value, if self.allow_half { 0.5 } else { 1.0 })
            .clamp(0.0, self.count as f32);
        if value != self.value {
            self.value = value;
        }
        let style = self.style.get(self.state);
        self.draw_rate.merge(style);
        self.draw_rate.count = self.count as f32;
        self.draw_rate.value = self.value;
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);

        match hit {
            Hit::FingerDown(e) => {
                if let Some(v) = self.tmp_value {
                    self.value_for_star(cx, v, false);
                }
                self.switch_state_with_animation(cx, RateState::Pressed);
                hit_finger_down!(self, cx, area, e);
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.style.get(self.state).cursor);

                let real_len = e.abs.x - e.rect.pos.x;
                let mut v = (real_len / e.rect.size.x).clamp(0.0, 1.0);
                v = v * self.count as f64;
                self.value_for_star(cx, v as f32, true);

                self.switch_state_with_animation(cx, RateState::Hover);
                self.play_animation(cx, id!(hover.on));
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                // reset tmp_value
                self.tmp_value = None;
                self.switch_state_with_animation(cx, RateState::Basic);
                self.play_animation(cx, id!(hover.off));
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.switch_state_with_animation(cx, RateState::Hover);
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.switch_state_with_animation(cx, RateState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_changed(cx, e.into());
                } else {
                    self.switch_state_with_animation(cx, RateState::Basic);
                    hit_finger_up!(self, cx, e);
                }
            }
            Hit::FingerHoverOver(e) => {
                let real_len = e.abs.x - e.rect.pos.x;
                let mut v = (real_len / e.rect.size.x).clamp(0.0, 1.0);
                v = v * self.count as f64;
                self.value_for_star(cx, v as f32, true);
                self.switch_state_with_animation(cx, RateState::Hover);
                self.play_animation(cx, id!(hover.on));
                
            }
            _ => {}
        };
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, RateState::Disabled);
                cx.set_cursor(self.style.get(self.state).cursor);
            }
            _ => {}
        }
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().rate;

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
        let mut value = if let Some(v) = self.tmp_value {
            v
        } else {
            self.value
        };
        value = round_step(value, if self.allow_half { 0.5 } else { 1.0 })
            .clamp(0.0, self.count as f32);
        if self.lifecycle.is_created() || !init_global || self.scope_path.is_none() {
            self.lifecycle.next();
            let basic_prop = self.style.get(RateState::Basic);
            let hover_prop = self.style.get(RateState::Hover);
            let pressed_prop = self.style.get(RateState::Pressed);
            let disabled_prop = self.style.get(RateState::Disabled);
            let (mut basic_index, mut hover_index, mut pressed_index, mut disabled_index) =
                (None, None, None, None);
            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(off).as_instance(),
                ],
            ) {
                basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(on).as_instance(),
                ],
            ) {
                hover_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(pressed).as_instance(),
                ],
            ) {
                pressed_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_rate = {
                    basic_index => {
                        color => basic_prop.color,
                        spacing => basic_prop.spacing as f64,
                        count => self.count as f64,
                        value => value as f64
                    },
                    hover_index => {
                        color => hover_prop.color,
                        spacing => hover_prop.spacing as f64,
                        count => self.count as f64,
                        value => value as f64
                    },
                    pressed_index => {
                        color => pressed_prop.color,
                        spacing => pressed_prop.spacing as f64,
                        count => self.count as f64,
                        value => value as f64
                    },
                    disabled_index => {
                        color => disabled_prop.color,
                        spacing => disabled_prop.spacing as f64,
                        count => self.count as f64,
                        value => value as f64
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                RateState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                RateState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                RateState::Pressed => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(pressed).as_instance(),
                    ],
                ),
                RateState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_rate = {
                    index => {
                        color => style.color,
                        spacing => style.spacing as f64,
                        count => self.count as f64,
                        value => value as f64
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

impl GRate {
    active_event! {
        active_hover_in: RateEvent::HoverIn |meta: FingerHoverEvent| => RateHoverIn { meta },
        active_hover_out: RateEvent::HoverOut |meta: FingerHoverEvent| => RateHoverOut { meta },
        active_finger_up: RateEvent::FingerUp |meta: FingerUpEvent| => RateFingerUp { meta },
        active_finger_down: RateEvent::FingerDown |meta: FingerDownEvent| => RateFingerDown { meta }
    }

    event_option! {
        hover_in: RateEvent::HoverIn => RateHoverIn,
        hover_out: RateEvent::HoverOut => RateHoverOut,
        finger_up: RateEvent::FingerUp => RateFingerUp,
        finger_down: RateEvent::FingerDown => RateFingerDown,
        changed: RateEvent::Changed => RateChanged
    }

    pub fn active_changed(&mut self, cx: &mut Cx, meta: RateChangedMetaEvent) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    RateEvent::Changed(RateChanged {
                        meta,
                        value: self.value,
                    }),
                );
            });
        }
    }

    fn value_for_star(&mut self, _cx: &mut Cx, value: f32, is_hover: bool) -> () {
        if value == self.value {
            return;
        }

        let value = round_step(value, if self.allow_half { 0.5 } else { 1.0 })
            .clamp(0.0, self.count as f32);
        if is_hover {
            self.tmp_value.replace(value);
        } else {
            self.value = value;
        };
    }

    pub fn set_value(&mut self, cx: &mut Cx, value: f32) -> () {
        self.value_for_star(cx, value, false);
    }
}
