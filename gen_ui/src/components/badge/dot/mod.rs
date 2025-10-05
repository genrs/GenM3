mod prop;

use makepad_widgets::*;
pub use prop::*;

use crate::{
    ComponentAnInit, animation_open_then_redraw,
    components::{
        label::{GLabel, LabelBasicStyle},
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, SlotComponent, SlotStyle, Style},
        view::ViewBasicStyle,
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, DeferWalks, SlotDrawer, ToStateMap,
        manuel::{BASIC, DISABLED},
        traits::ToFloat,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_dot::DrawDot,
    sync,
    themes::conf::Conf,
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GBadgeDotBase = {{GBadgeDot}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    apply: {
                        draw_dot: <AN_DRAW_DOT> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_dot: <AN_DRAW_DOT> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GBadgeDot {
    #[live]
    pub style: BadgeDotStyle,
    #[live]
    pub dot: bool,
    // --- draw ----------------------
    #[live]
    pub draw_dot: DrawDot,
    // --- slots ----------------------
    #[live]
    pub text: GLabel,
    // --- other ----------------------
    #[live(false)]
    pub disabled: bool,
    #[live(false)]
    pub grab_key_focus: bool,
    #[rust]
    pub apply_slot_map: ApplySlotMap<BadgeDotState, BadgeDotPart>,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    animator: Animator,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    defer_walks: DeferWalks,
    #[rust]
    pub state: BadgeDotState,
}

impl WidgetNode for GBadgeDot {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        let text_ref = self.text.uid_to_widget(uid);

        if !text_ref.is_empty() {
            return text_ref;
        }

        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.container.walk()
    }

    fn area(&self) -> Area {
        self.draw_dot.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        if self.text.visible {
            self.text.redraw(cx);
        }
        self.draw_dot.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    visible!();
}

impl LiveHook for GBadgeDot {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            nodes,
            index,
            [live_id!(basic), live_id!(disabled)],
            [
                (BadgeDotPart::Text, &LabelBasicStyle::live_props()),
                (BadgeDotPart::Container, &ViewBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_slot_map
                        .insert(BadgeDotState::Basic, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(BadgeDotState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<BadgeDotState> for GBadgeDot {
    type Part = BadgeDotPart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.text.style.basic = self.style.basic.text;
        self.text.style.disabled = self.style.disabled.text;
    }
}

impl Component for GBadgeDot {
    type Error = Error;

    type State = BadgeDotState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.badge_dot;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let state = if self.disabled {
            BadgeDotState::Disabled
        } else {
            BadgeDotState::Basic
        };
        self.switch_state(state);
        let style = self.style.get(self.state);
        self.draw_dot.merge(&style.container);
        self.draw_dot.dot = self.dot.to_f32();
        let _ = self.text.render(cx)?;
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, _hit: Hit, _area: Area) {
        animation_open_then_redraw!(self, cx, event);
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, BadgeDotState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.text.switch_state(state.into());
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        let mut crossed_map = self.apply_slot_map.cross();

        crossed_map.remove(&BadgeDotPart::Text).map(|map| {
            self.text.apply_state_map.merge(map.to_state());
            self.text.focus_sync();
        });

        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().tag;
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
        let dot = self.dot.to_f64();
        if self.lifecycle.is_created() || !init_global || self.scope_path.is_none() {
            self.lifecycle.next();
            let basic_prop = self.style.get(BadgeDotState::Basic);
            let disabled_prop = self.style.get(BadgeDotState::Disabled);
            let (mut basic_index, mut disabled_index) = (None, None);

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
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_dot = {
                    basic_index => {
                        background_color => basic_prop.container.background_color,
                        border_color => basic_prop.container.border_color,
                        border_radius => basic_prop.container.border_radius,
                        border_width => (basic_prop.container.border_width as f64),
                        shadow_color => basic_prop.container.shadow_color,
                        spread_radius => (basic_prop.container.spread_radius as f64),
                        blur_radius => (basic_prop.container.blur_radius as f64),
                        shadow_offset => basic_prop.container.shadow_offset,
                        background_visible => basic_prop.container.background_visible.to_f64(),
                        dot => dot
                    },
                    disabled_index => {
                        background_color => disabled_prop.container.background_color,
                        border_color => disabled_prop.container.border_color,
                        border_radius => disabled_prop.container.border_radius,
                        border_width => (disabled_prop.container.border_width as f64),
                        shadow_color => disabled_prop.container.shadow_color,
                        spread_radius => (disabled_prop.container.spread_radius as f64),
                        blur_radius => (disabled_prop.container.blur_radius as f64),
                        shadow_offset => disabled_prop.container.shadow_offset,
                        background_visible => disabled_prop.container.background_visible.to_f64(),
                        dot => dot
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                BadgeDotState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                BadgeDotState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_dot = {
                    index => {
                        background_color => style.container.background_color,
                        border_color => style.container.border_color,
                        border_radius => style.container.border_radius,
                        border_width => (style.container.border_width as f64),
                        shadow_color => style.container.shadow_color,
                        spread_radius => (style.container.spread_radius as f64),
                        blur_radius => (style.container.blur_radius as f64),
                        shadow_offset => style.container.shadow_offset,
                        background_visible => style.container.background_visible.to_f64(),
                        dot => dot
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
}

impl Widget for GBadgeDot {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible() {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        let _ = self.draw_dot.begin(cx, walk, style.container.layout());
        if !self.dot {
            let _ = SlotDrawer::new(
                [(live_id!(text), (&mut self.text).into())],
                &mut self.defer_walks,
            )
            .draw_walk(cx, scope);
        }

        let _ = self.draw_dot.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible() {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().tag = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl GBadgeDot {
    pub fn area_text(&self) -> Area {
        self.text.area()
    }
    pub fn slot_text(&self) -> &GLabel {
        &self.text
    }
    pub fn slot_text_mut(&mut self) -> &mut GLabel {
        &mut self.text
    }
}

impl GBadgeDotRef {
    pub fn slot_text_mut<F>(&mut self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut Cx, &mut GLabel),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            f(cx, c_ref.slot_text_mut());
        }
    }
}
