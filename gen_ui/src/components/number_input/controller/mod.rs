mod prop;

pub use prop::*;

use super::event::*;
use crate::{
    active_event, components::{
        BasicStyle, ButtonBasicStyle, ButtonState, Component, GButton, LifeCycle, NumberCtrClicked,
        SlotComponent, SlotStyle, Style, ViewBasicStyle,
    }, error::Error, event_option, event_option_ref, lifecycle, prop::{
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, DeferWalks, SlotDrawer, ToStateMap,
        manuel::{BASIC, DISABLED, HOVER, PRESSED},
    }, pure_after_apply, set_index, set_scope_path, shader::draw_view::DrawView, switch_state, sync, themes::conf::Conf, visible
};
use makepad_widgets::*;

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GNumberCtrBase = {{GNumberCtr}} {}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GNumberCtr {
    #[live]
    pub style: NumberCtrStyle,
    #[live]
    pub up: GButton,
    #[live]
    pub down: GButton,
    #[live]
    pub draw_number_ctr: DrawView,
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
    pub apply_slot_map: ApplySlotMap<ButtonState, NumberCtrPart>,
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
    pub state: ButtonState,
    #[rust]
    defer_walks: DeferWalks,
    // ----------------------------------
    #[live]
    pub value: f32,
}

impl WidgetNode for GNumberCtr {
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
        self.draw_number_ctr.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_number_ctr.redraw(cx);
        for slot in [&mut self.up, &mut self.down] {
            if slot.visible {
                slot.redraw(cx);
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

impl Widget for GNumberCtr {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_number_ctr.begin(cx, walk, style.layout());

        let _ = SlotDrawer::new(
            [
                (live_id!(up), (&mut self.up).into()),
                (live_id!(down), (&mut self.down).into()),
            ],
            &mut self.defer_walks,
        )
        .draw_walk(cx, scope);
        self.draw_number_ctr.end(cx);
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

        self.up.handle_event(cx, event, scope);
        self.down.handle_event(cx, event, scope);
    }
}

impl MatchEvent for GNumberCtr {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if let Some(param) = self.up.clicked(actions) {
            self.active_up(cx, Some(param.meta));
        }

        if let Some(param) = self.down.clicked(actions) {
            self.active_down(cx, Some(param.meta));
        }
    }
}

impl LiveHook for GNumberCtr {
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
                live_id!(basic),
                live_id!(disabled),
                live_id!(hover),
                live_id!(pressed),
            ],
            [
                (NumberCtrPart::Container, &ViewBasicStyle::live_props()),
                (NumberCtrPart::Button, &ButtonBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(ButtonState::Basic, applys);
                }
                HOVER => {
                    component.apply_slot_map.insert(ButtonState::Hover, applys);
                }
                PRESSED => {
                    component
                        .apply_slot_map
                        .insert(ButtonState::Pressed, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(ButtonState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<ButtonState> for GNumberCtr {
    type Part = NumberCtrPart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.up.style.basic = self.style.basic.button;
        self.up.style.hover = self.style.hover.button;
        self.up.style.pressed = self.style.pressed.button;
        self.up.style.disabled = self.style.disabled.button;
        self.down.style.basic = self.style.basic.button;
        self.down.style.hover = self.style.hover.button;
        self.down.style.pressed = self.style.pressed.button;
        self.down.style.disabled = self.style.disabled.button;
    }
}

impl Component for GNumberCtr {
    type Error = Error;

    type State = ButtonState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.number_ctr;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(ButtonState::Disabled);
        }
        let style = self.style.get(self.state).container;
        self.draw_number_ctr.merge(&style);
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

        crossed_map.remove(&NumberCtrPart::Button).map(|map| {
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

impl GNumberCtr {
    active_event! {
        active_up: NumberCtrEvent::Up | meta: Option<FingerUpEvent> | => NumberCtrClicked {meta},
        active_down: NumberCtrEvent::Down | meta: Option<FingerUpEvent> | => NumberCtrClicked {meta}
    }

    event_option! {
        up: NumberCtrEvent::Up => NumberCtrClicked,
        down: NumberCtrEvent::Down => NumberCtrClicked
    }
}

impl GNumberCtrRef {
    event_option_ref! {
        up => NumberCtrClicked,
        down => NumberCtrClicked
    }
}