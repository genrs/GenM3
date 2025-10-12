pub mod area;
mod event;
mod prop;
mod register;
mod rely;

use crate::{
    ComponentAnInit,
    components::{
        BadgePart, BasicStyle, Component, GComponent, GView, LifeCycle, SlotComponent, SlotStyle,
        Style, ViewBasicStyle,
        area::{GInputArea, InputAreaBasicStyle},
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks, SlotDrawer,
        ToSlotMap, ToStateMap,
        manuel::{BASIC, DISABLED, EMPTY, FOCUS, HOVER},
        traits::{ToColor, ToFloat},
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible,
};
pub use event::*;
use makepad_widgets::*;
pub use prop::*;
pub use register::register as input_register;
use rely::*;

live_design! {
    link genui_basic;
    use link::theme::*;
    use link::genui_animation_prop::*;

    pub GInputBase = {{GInput}} {
        animator: {
            input = {
                default: basic,

                basic = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_input: <AN_DRAW_VIEW> {}
                    }
                }

                hover = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_input: <AN_DRAW_VIEW> {}
                    }
                }

                empty = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_input: <AN_DRAW_VIEW> {}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_input: <AN_DRAW_VIEW> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_input: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GInput {
    #[live]
    pub style: InputStyle,
    #[live]
    pub draw_input: DrawView,
    #[live]
    pub input: GInputArea,
    #[live]
    pub suffix: GView,
    #[live]
    pub prefix: GView,
    #[live]
    pub value: String,
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
    #[rust]
    pub state: InputState,
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
    pub apply_slot_map: ApplySlotMap<InputState, InputPart>,
    #[rust]
    defer_walks: DeferWalks,
}

impl WidgetNode for GInput {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_input.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_input.redraw(cx);
        if self.prefix.visible {
            self.prefix.redraw(cx);
        }
        if self.input.visible {
            self.input.redraw(cx);
        }
        if self.suffix.visible {
            self.suffix.redraw(cx);
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

impl Widget for GInput {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_input.begin(cx, walk, style.layout());

        let real_height = self.count_real_height(cx);
        let mut slots: [(LiveId, GComponent); 3] = [
            (live_id!(prefix), (&mut self.prefix).into()),
            (live_id!(input), (&mut self.input).into()),
            (live_id!(suffix), (&mut self.suffix).into()),
        ];

        for (id, component) in &mut slots {
            if component.visible() {
                let mut walk = component.walk(cx);
                if let Some(fw) = cx.defer_walk(walk) {
                    // if is fill, defer the walk
                    self.defer_walks.push((*id, fw));
                } else {
                    if *id == live_id!(prefix) || *id == live_id!(suffix) {
                        walk.height = Size::Fixed(real_height);
                        let _ = component.draw_walk(cx, scope, walk);
                    }
                }
            }
        }

        for (id, df_walk) in self.defer_walks.iter_mut() {
            for (slot_id, slot) in &mut slots {
                if *id == *slot_id {
                    let mut res_walk = df_walk.resolve(cx);
                    if *id == live_id!(prefix) || *id == live_id!(suffix) {
                        res_walk.height = Size::Fixed(real_height);
                    }
                    let _ = slot.draw_walk(cx, scope, res_walk);
                    break;
                }
            }
        }

        self.draw_input.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
}

impl LiveHook for GInput {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [
                live_id!(empty),
                live_id!(basic),
                live_id!(hover),
                live_id!(focus),
                live_id!(disabled),
            ],
            [
                (InputPart::Container, &ViewBasicStyle::live_props()),
                (InputPart::Input, &InputAreaBasicStyle::live_props()),
                (InputPart::Prefix, &ViewBasicStyle::live_props()),
                (InputPart::Suffix, &ViewBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(InputState::Basic, applys);
                }
                HOVER => {
                    component.apply_slot_map.insert(InputState::Hover, applys);
                }
                FOCUS => {
                    component.apply_slot_map.insert(InputState::Focus, applys);
                }
                EMPTY => {
                    component.apply_slot_map.insert(InputState::Empty, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(InputState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<InputState> for GInput {
    type Part = InputPart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.input.style.basic = self.style.basic.input;
        self.input.style.hover = self.style.hover.input;
        self.input.style.focus = self.style.focus.input;
        self.input.style.empty = self.style.empty.input;
        self.input.style.disabled = self.style.disabled.input;
        self.prefix.style.basic = self.style.basic.prefix;
        self.prefix.style.hover = self.style.hover.prefix;
        self.prefix.style.pressed = self.style.focus.prefix;
        self.prefix.style.disabled = self.style.disabled.prefix;
        self.suffix.style.basic = self.style.basic.suffix;
        self.suffix.style.hover = self.style.hover.suffix;
        self.suffix.style.pressed = self.style.focus.suffix;
        self.suffix.style.disabled = self.style.disabled.suffix;
    }
}

impl Component for GInput {
    type Error = Error;

    type State = InputState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.input;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(InputState::Disabled);
        } else if self.value.is_empty() {
            self.switch_state(InputState::Empty);
        } else {
            self.switch_state(InputState::Basic);
        }
        let style = self.style.get(self.state);
        self.draw_input.merge(&style.container);
        let _ = self.input.render(cx);
        let _ = self.prefix.render(cx);
        let _ = self.suffix.render(cx);
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, InputState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        if self.state != state {
            self.state = state;
        }
        self.prefix.switch_state(state.into());
        self.input.switch_state(state);
        self.suffix.switch_state(state.into());
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

        for (part, slot) in [
            (InputPart::Prefix, &mut self.prefix),
            (InputPart::Suffix, &mut self.suffix),
        ] {
            crossed_map.remove(&part).map(|map| {
                slot.apply_state_map.merge(map.to_state());
                slot.focus_sync();
            });
        }

        crossed_map.remove(&InputPart::Input).map(|map| {
            self.input.apply_slot_map.merge_slot(map.to_slot());
            self.input.focus_sync();
        });

        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().input;

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
            let basic_prop = self.style.get(InputState::Basic).container;
            let hover_prop = self.style.get(InputState::Hover).container;
            let focus_prop = self.style.get(InputState::Focus).container;
            let empty_prop = self.style.get(InputState::Empty).container;
            let disabled_prop = self.style.get(InputState::Disabled).container;
            let (
                mut basic_index,
                mut disabled_index,
                mut empty_index,
                mut hover_index,
                mut focus_index,
            ) = (None, None, None, None, None);
            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(basic).as_instance(),
                ],
            ) {
                basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(hover).as_instance(),
                ],
            ) {
                hover_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(focus).as_instance(),
                ],
            ) {
                focus_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(empty).as_instance(),
                ],
            ) {
                empty_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_input = {
                    basic_index => {
                        background_color => basic_prop.background_color,
                        border_color =>basic_prop.border_color,
                        border_radius => basic_prop.border_radius,
                        border_width =>(basic_prop.border_width as f64),
                        shadow_color => basic_prop.shadow_color,
                        spread_radius => (basic_prop.spread_radius as f64),
                        blur_radius => (basic_prop.blur_radius as f64),
                        shadow_offset => basic_prop.shadow_offset,
                        background_visible => basic_prop.background_visible.to_f64()
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
                        background_visible => hover_prop.background_visible.to_f64()
                    },
                    focus_index => {
                        background_color => focus_prop.background_color,
                        border_color => focus_prop.border_color,
                        border_radius => focus_prop.border_radius,
                        border_width => (focus_prop.border_width as f64),
                        shadow_color => focus_prop.shadow_color,
                        spread_radius => (focus_prop.spread_radius as f64),
                        blur_radius => (focus_prop.blur_radius as f64),
                        shadow_offset => focus_prop.shadow_offset,
                        background_visible => focus_prop.background_visible.to_f64()
                    },
                    empty_index => {
                        background_color => empty_prop.background_color,
                        border_color => empty_prop.border_color,
                        border_radius => empty_prop.border_radius,
                        border_width => (empty_prop.border_width as f64),
                        shadow_color => empty_prop.shadow_color,
                        spread_radius => (empty_prop.spread_radius as f64),
                        blur_radius => (empty_prop.blur_radius as f64),
                        shadow_offset => empty_prop.shadow_offset,
                        background_visible => empty_prop.background_visible.to_f64()
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
                        background_visible => disabled_prop.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state).container;
            let index = match state {
                InputState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(basic).as_instance(),
                    ],
                ),
                InputState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(hover).as_instance(),
                    ],
                ),
                InputState::Focus => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(focus).as_instance(),
                    ],
                ),
                InputState::Empty => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(empty).as_instance(),
                    ],
                ),
                InputState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_input = {
                    index => {
                        background_color => style.background_color,
                        border_color => style.border_color,
                        border_radius => style.border_radius,
                        border_width => (style.border_width as f64),
                        shadow_color => style.shadow_color,
                        spread_radius => (style.spread_radius as f64),
                        blur_radius => (style.blur_radius as f64),
                        shadow_offset => style.shadow_offset,
                        background_visible => style.background_visible.to_f64()
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

impl GInput {
    pub fn count_real_height(&self, cx: &mut Cx) -> f64 {
        let font_metrics = cx.global::<Conf>().theme.font.metrics;
        let style = self.style.get(self.state);
        let text_style = style.input.text;
        let padding = text_style.padding.top
            + text_style.padding.bottom
            + style.input.container.padding.top
            + style.input.container.padding.bottom
            + style.container.padding.top
            + style.container.padding.bottom;
        let margin = text_style.margin.top
            + text_style.margin.bottom
            + style.input.container.margin.top
            + style.input.container.margin.bottom;

        ((text_style.font_size * font_metrics) as f64) + padding + margin
    }
}
