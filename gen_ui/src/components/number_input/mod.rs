pub mod controller;
mod event;
mod prop;
mod register;

pub use register::register as number_input_register;
pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{
        BasicStyle, ButtonBasicStyle, Component, GButton, InputChanged, InputChangedMetaEvent,
        InputFocus, InputFocusMetaEvent, InputKeyDown, InputMaxLengthReached, InputState,
        LifeCycle, SlotComponent, SlotStyle, Style, ViewBasicStyle,
        area::{GInputArea, InputAreaBasicStyle, InputAreaPart},
    },
    error::Error,
    lifecycle,
    prop::{
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks, SlotDrawer,
        ToSlotMap, ToStateMap,
        manuel::{BASIC, DISABLED},
        traits::{NewFrom, ToColor},
    },
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    switch_state, sync,
    themes::conf::Conf,
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GNumberInputBase = {{GNumberInput}} {}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GNumberInput {
    #[live]
    pub style: NumberInputStyle,
    #[live(4)]
    pub length: i32,
    #[live]
    pub input: GInputArea,
    #[live]
    pub up: GButton,
    #[live]
    pub down: GButton,
    // #[rust]
    // live_update_order: SmallVec<[LiveId; 1]>,
    #[live]
    pub draw_number_input: DrawView,
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
    pub apply_slot_map: ApplySlotMap<NumberInputState, NumberInputPart>,
    #[rust]
    pub index: usize,
    #[rust(true)]
    pub sync: bool,
    #[live(true)]
    pub animation_open: bool,
    #[live(true)]
    pub animation_spread: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub state: NumberInputState,
    #[rust]
    defer_walks: DeferWalks,
    // ----------------------------------
    #[live]
    pub value: f32,
}

impl WidgetNode for GNumberInput {
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
        self.draw_number_input.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_number_input.redraw(cx);
        for slot in [&mut self.up, &mut self.down] {
            if slot.visible {
                slot.redraw(cx);
            }
        }
        if self.input.visible {
            self.input.redraw(cx);
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

impl Widget for GNumberInput {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_number_input.begin(cx, walk, style.layout());

        if self.input.visible {
            let walk = self.input.walk(cx);
            let _ = self.input.draw_walk(cx, scope, walk);
        }

        let wrapper_height = self.input.area().rect(cx).size.y;
        // 绘制 up 和 down 按钮
        cx.begin_turtle(
            Walk {
                height: Size::Fixed(wrapper_height),
                width: Size::Fixed(24.0),
                ..Default::default()
            },
            Layout {
                padding: Padding::from_f64(4.0),
                align: Align::from_f64(0.5),
                flow: Flow::Down,
                spacing: 6.0,
                ..Default::default()
            },
        );

        let _ = SlotDrawer::new(
            [
                (live_id!(up), (&mut self.up).into()),
                (live_id!(down), (&mut self.down).into()),
            ],
            &mut self.defer_walks,
        )
        .draw_walk(cx, scope);

        cx.end_turtle();

        self.draw_number_input.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        if self.disabled {
            let area = self.area();
            let hit = event.hits(cx, area);
            self.handle_when_disabled(cx, event, hit);
            return;
        }

        self.match_event(cx, event);

        if self.input.visible {
            self.input.handle_event(cx, event, scope);
        }

        self.up.handle_event(cx, event, scope);
        self.down.handle_event(cx, event, scope);
    }
}

impl MatchEvent for GNumberInput {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {}
}

impl LiveHook for GNumberInput {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [live_id!(basic), live_id!(disabled)],
            [
                (NumberInputPart::Container, &ViewBasicStyle::live_props()),
                (NumberInputPart::Input, &InputAreaBasicStyle::live_props()),
                (NumberInputPart::Button, &ButtonBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_slot_map
                        .insert(NumberInputState::Basic, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(NumberInputState::Disabled, applys);
                }
                _ => {}
            },
        );

        self.apply_items(cx);
    }
}

impl SlotComponent<NumberInputState> for GNumberInput {
    type Part = NumberInputPart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.up.style.basic = self.style.basic.button;
        // self.up.style.hover = self.style.basic.button;
        self.up.style.disabled = self.style.disabled.button;
        self.down.style.basic = self.style.basic.button;
        self.down.style.disabled = self.style.disabled.button;
        self.input.style.basic = self.style.basic.input;
        self.input.style.disabled = self.style.disabled.input;
    }
}

impl Component for GNumberInput {
    type Error = Error;

    type State = NumberInputState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.number_input;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(NumberInputState::Disabled);
        }
        let style = self.style.get(self.state).container;
        self.draw_number_input.merge(&style);
        Ok(())
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
        ()
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
        let mut crossed_map = self.apply_slot_map.cross();
        // crossed_map.remove(&NumberInputPart::Input).map(|map| {
        //     self.apply_items_map.merge_slot(map.to_slot());
        // });

        crossed_map.remove(&NumberInputPart::Input).map(|map| {
            self.input.apply_slot_map.merge_slot(map.to_slot());
            self.input.focus_sync();
        });

        crossed_map.remove(&NumberInputPart::Button).map(|map| {
            self.up.apply_state_map.merge(map.clone().to_state());
            self.down.apply_state_map.merge(map.to_state());
            self.up.focus_sync();
            self.down.focus_sync();
        });

        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    sync!();
    set_scope_path!();
    set_index!();
    lifecycle!();
    switch_state!();
}

impl GNumberInput {
    pub fn active_changed(
        &mut self,
        cx: &mut Cx,
        meta: Option<InputChangedMetaEvent>,
        adjust: NumberInputAdjust,
    ) {
        if self.event_open {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    NumberInputEvent::Changed(NumberInputChanged {
                        meta,
                        value: self.value,
                        adjust,
                    }),
                );
            }
        }
    }
    pub fn apply_items(&mut self, cx: &mut Cx) {}
}
