mod event;
mod prop;

use makepad_widgets::*;

pub use event::*;
pub use prop::*;

use crate::{
    ComponentAnInit, active_event, animation_open_then_redraw,
    components::{BasicStyle, Component, LifeCycle, Style},
    error::Error,
    lifecycle, play_animation,
    prop::{
        ApplyStateMap, ProgressMode,
        manuel::{BASIC, DISABLED, DRAGGING, HOVER},
        traits::ToFloat,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_slider::DrawSlider as DrawGSlider,
    switch_state, sync,
    themes::conf::Conf,
    utils::{normalization, round_2_decimals_f32, round_step},
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GSliderBase = {{GSlider}} {
        animator: {
            dragging = {
                default: off,

                off = {
                    from: {all: Forward {duration: (0.0)}},
                    ease: InOutQuad,
                    apply: {
                        draw_slider: <AN_DRAW_SLIDER> {
                            dragging: 0.0,
                        }
                    }
                }

                hover = {
                    from: {all: Forward {duration: (0.0)}},
                    ease: InOutQuad,
                    apply: {
                        draw_slider: <AN_DRAW_SLIDER> {
                            dragging: 0.0,
                        }
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (0.0),},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_slider: <AN_DRAW_SLIDER> {
                            dragging: 1.0,
                       }
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_slider: <AN_DRAW_SLIDER> {
                            dragging: 0.0,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSlider {
    #[live]
    pub style: SliderStyle,
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
    pub apply_state_map: ApplyStateMap<SliderState>,
    #[rust]
    pub state: SliderState,
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
    pub draw_slider: DrawGSlider,
    #[live(0.0)]
    pub min: f32,
    #[live(1.0)]
    pub max: f32,
    #[live(0.0)]
    pub value: f32,
    #[live(0.1)]
    pub step: f32,
    #[live(1.0)]
    pub proportion: f32,
}

impl WidgetNode for GSlider {
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
        self.draw_slider.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_slider.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GSlider {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let style = self.style.get(self.state);
        let _ = self.draw_slider.begin(cx, walk, style.layout());
        let _ = self.draw_slider.end(cx);
        self.set_scope_path(&scope.path);
        return DrawStep::done();
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().slider = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl LiveHook for GSlider {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            apply.from,
            nodes,
            index,
            &SliderBasicStyle::live_props(),
            [
                live_id!(basic),
                live_id!(dragging),
                live_id!(hover),
                live_id!(disabled),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(SliderState::Basic, applys);
                }
                HOVER => {
                    component.apply_state_map.insert(SliderState::Hover, applys);
                }
                DRAGGING => {
                    component
                        .apply_state_map
                        .insert(SliderState::Dragging, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(SliderState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GSlider {
    type Error = Error;

    type State = SliderState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.slider;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(SliderState::Disabled);
        }
        let style = self.style.get(self.state);
        self.draw_slider.merge(style);
        self.draw_slider.mode = self.mode;
        self.draw_slider.value = normalization(self.value, self.min, self.max);
        self.draw_slider.step = self.step;
        self.draw_slider.proportion = self.proportion.clamp(0.0, 1.0);
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);

        match hit {
            Hit::FingerDown(e) => {
                self.switch_state_with_animation(cx, SliderState::Dragging);
                if self.grab_key_focus {
                    cx.set_key_focus(area);
                }
                self.play_animation(cx, id!(dragging.on));
                self.active_finger_down(cx, e);
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.style.get(self.state).cursor);
                self.switch_state_with_animation(cx, SliderState::Hover);
                self.play_animation(cx, id!(dragging.hover));
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, SliderState::Basic);
                self.play_animation(cx, id!(dragging.off));
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.switch_state_with_animation(cx, SliderState::Hover);
                        self.play_animation(cx, id!(dragging.hover));
                    } else {
                        self.switch_state_with_animation(cx, SliderState::Basic);
                        self.play_animation(cx, id!(dragging.off));
                    }

                    self.active_changed(cx, Some(e));
                } else {
                    self.switch_state_with_animation(cx, SliderState::Basic);
                    self.play_animation(cx, id!(dragging.off));
                    self.active_finger_up(cx, e);
                }
            }
            Hit::FingerMove(e) => {
                match self.mode {
                    ProgressMode::Horizontal | ProgressMode::Circle => {
                        let real_len = e.abs.x - e.rect.pos.x;
                        // percentage
                        let v = (real_len / e.rect.size.x).clamp(0.0, 1.0);
                        self.value = round_step(
                            round_2_decimals_f32((v as f32) * (self.max - self.min) + self.min),
                            self.step,
                        );
                    }
                    ProgressMode::Vertical => {
                        // For vertical mode, we need to invert the calculation
                        // because y increases downward, but we want progress to increase upward
                        let real_len = e.rect.pos.y + e.rect.size.y - e.abs.y;
                        // percentage
                        let v = (real_len / e.rect.size.y).clamp(0.0, 1.0);
                        self.value = round_step(
                            round_2_decimals_f32((v as f32) * (self.max - self.min) + self.min),
                            self.step,
                        );
                    }
                }
                self.switch_state_with_animation(cx, SliderState::Dragging);
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
        self.redraw(cx);
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().slider;

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
            let basic_prop = self.style.get(SliderState::Basic);
            let hover_prop = self.style.get(SliderState::Hover);
            let dragging_prop = self.style.get(SliderState::Dragging);
            let disabled_prop = self.style.get(SliderState::Disabled);
            let (mut basic_index, mut dragging_index, mut hover_index, mut disabled_index) =
                (None, None, None, None);
            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(dragging).as_instance(),
                    live_id!(off).as_instance(),
                ],
            ) {
                basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(dragging).as_instance(),
                    live_id!(on).as_instance(),
                ],
            ) {
                dragging_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(dragging).as_instance(),
                    live_id!(hover).as_instance(),
                ],
            ) {
                hover_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(dragging).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }
            let v = normalization(self.value, self.min, self.max);
            let proportion = self.proportion.clamp(0.0, 1.0);
            set_animation! {
                nodes: draw_slider = {
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
                        value => (v as f64),
                        proportion => (proportion as f64)
                    },
                    hover_index => {
                        background_color => hover_prop.background_color,
                        border_color => hover_prop.border_color,
                        border_radius => hover_prop.border_radius,
                        border_width => (hover_prop.border_width as f64),
                        shadow_color => hover_prop.shadow_color,
                        spread_radius => (hover_prop.spread_radius as f64),
                        blur_radius => (hover_prop.blur_radius as f64),
                        shadow_offset => hover_prop.shadow_offset,
                        background_visible => hover_prop.background_visible.to_f64(),
                        color => hover_prop.color,
                        value => (v as f64),
                        proportion => (proportion as f64)
                    },
                    dragging_index => {
                        background_color => dragging_prop.background_color,
                        border_color => dragging_prop.border_color,
                        border_radius => dragging_prop.border_radius,
                        border_width => (dragging_prop.border_width as f64),
                        shadow_color => dragging_prop.shadow_color,
                        spread_radius => (dragging_prop.spread_radius as f64),
                        blur_radius => (dragging_prop.blur_radius as f64),
                        shadow_offset => dragging_prop.shadow_offset,
                        background_visible => dragging_prop.background_visible.to_f64(),
                        color => dragging_prop.color,
                        value => (v as f64),
                        proportion => (proportion as f64)
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
                        value => (v as f64),
                        proportion => (proportion as f64)
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let v = normalization(self.value, self.min, self.max);
            let proportion = self.proportion.clamp(0.0, 1.0);
            let index = match state {
                SliderState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(dragging).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                SliderState::Dragging => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(dragging).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                SliderState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(dragging).as_instance(),
                        live_id!(hover).as_instance(),
                    ],
                ),
                SliderState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(dragging).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_slider = {
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
                        value => (v as f64),
                        proportion => (proportion as f64)
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

impl GSlider {
    active_event! {
        active_hover_in: SliderEvent::HoverIn |meta: FingerHoverEvent| => SliderHoverIn { meta },
        active_hover_out: SliderEvent::HoverOut |meta: FingerHoverEvent| => SliderHoverOut { meta },
        active_finger_up: SliderEvent::FingerUp |meta: FingerUpEvent| => SliderFingerUp { meta },
        active_finger_down: SliderEvent::FingerDown |meta: FingerDownEvent| => SliderFingerDown { meta }
    }
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    SliderEvent::Changed(SliderChanged {
                        meta,
                        value: self.value as f64,
                        step: self.step as f64,
                        range: [self.min as f64, self.max as f64],
                    }),
                );
            });
        }
    }
}
