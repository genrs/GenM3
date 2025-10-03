mod prop;

pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{
        BasicStyle, Component, GComponent, GLabel, GSvg, LifeCycle, SlotComponent, SlotStyle, Style,
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, ApplyStateMap,
        DeferWalks, SlotDrawer, ToSlotMap, ToStateMap,
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

    pub GSelectItemBase = {{GSelectItem}} {}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSelectItem {
    #[live]
    pub style: SelectItemStyle,
    #[live]
    pub active: bool,
    #[live]
    pub value: String,
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
    pub apply_slot_map: ApplySlotMap<SelectItemState, SelectItemPart>,
    #[live]
    pub draw_item: DrawView,
    // --- slot --------------------
    /// prefix icon
    #[live]
    pub icon: GSvg,
    #[live]
    pub text: GLabel,
    /// suffix icon
    #[live]
    pub suffix: GSvg,
    #[rust]
    defer_walks: DeferWalks,
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
    pub state: SelectItemState,
}

impl WidgetNode for GSelectItem {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.style.get(self.state).walk()
    }

    fn area(&self) -> Area {
        self.draw_item.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_item.redraw(cx);
        for mut slot in [
            GComponent::Svg(&mut self.icon),
            GComponent::Label(&mut self.text),
            GComponent::Svg(&mut self.suffix),
        ] {
            if slot.visible() {
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

impl Widget for GSelectItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let style = self.style.get(self.state);
        let _ = self.draw_item.begin(cx, walk, style.layout());

        let _ = SlotDrawer::new(
            [
                (live_id!(icon), (&mut self.icon).into()),
                (live_id!(text), (&mut self.text).into()),
                (live_id!(suffix), (&mut self.suffix).into()),
            ],
            &mut self.defer_walks,
        )
        .draw_walk(cx, scope);

        let _ = self.draw_item.end(cx);
        self.set_scope_path(&scope.path);
        return DrawStep::done();
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl LiveHook for GSelectItem {
    pure_after_apply!();
}

impl SlotComponent<SelectItemState> for GSelectItem {
    type Part = SelectItemPart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.icon.style.basic = self.style.basic.icon;
        self.icon.style.hover = self.style.hover.icon;
        self.icon.style.pressed = self.style.active.icon;
        self.icon.style.disabled = self.style.disabled.icon;

        self.text.style.basic = self.style.basic.text;
        self.text.style.disabled = self.style.disabled.text;

        self.suffix.style.basic = self.style.basic.suffix;
        self.suffix.style.hover = self.style.hover.suffix;
        self.suffix.style.pressed = self.style.active.suffix;
        self.suffix.style.disabled = self.style.disabled.suffix;
    }
}

impl Component for GSelectItem {
    type Error = Error;

    type State = SelectItemState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.select_item;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.state;
        let style = self.style.get(state);
        self.draw_item.merge(&style.container);
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        ()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.icon.switch_state(state.into());
        self.text.switch_state(state.into());
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

        crossed_map.remove(&SelectItemPart::Icon).map(|map| {
            self.icon.apply_slot_map.merge_slot(map.to_slot());
            self.icon.focus_sync();
        });

        crossed_map.remove(&SelectItemPart::Text).map(|map| {
            self.text.apply_state_map.merge(map.to_state());
            self.text.focus_sync();
        });

        crossed_map.remove(&SelectItemPart::Suffix).map(|map| {
            self.suffix.apply_slot_map.merge_slot(map.to_slot());
            self.suffix.focus_sync();
        });

        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}
