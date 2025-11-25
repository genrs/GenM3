pub mod area;
mod event;
mod prop;
mod register;
mod rely;

use crate::{
    components::{
        BasicStyle, Component, GComponent, GView, LifeCycle, SlotComponent, SlotStyle, Style,
        ViewBasicStyle,
        area::{GInputArea, InputAreaBasicStyle},
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks,
        ToSlotMap, ToStateMap,
        manuel::{BASIC, DISABLED, EMPTY, FOCUS, HOVER},
    },
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible,
};
pub use event::*;
use makepad_widgets::*;
pub use prop::*;
pub use register::register as input_register;


live_design! {
    link genui_basic;
    use link::theme::*;
    use link::genui_animation_prop::*;

    pub GInputBase = {{GInput}} {}
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
    // ---
    #[live(None)]
    pub length: Option<usize>,
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

        self.defer_walks.clear();
        for (id, component) in &mut slots {
            if component.visible() {
                let mut walk = component.walk(cx);
                if let Some(fw) = cx.defer_walk(walk) {
                    // if is fill, defer the walk
                    self.defer_walks.push((*id, fw));
                } else {
                    if *id == live_id!(prefix) || *id == live_id!(suffix) {
                        walk.height = Size::Fixed(real_height);
                    }
                    let _ = component.draw_walk(cx, scope, walk);
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

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        let area = self.area();

        self.prefix.handle_event(cx, event, scope);
        self.input.handle_event(cx, event, scope);
        self.suffix.handle_event(cx, event, scope);

        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        }
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

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {}

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

        // keep length in sync
        self.input.length = self.length;
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
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

        ((text_style.font_size * font_metrics) as f64) + padding + margin + 0.8
    }
}
