mod prop;

use super::event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    ComponentAnInit, active_event, animation_open_then_redraw,
    components::{
        BasicStyle, Component, GComponent, GLabel, GSvg, LabelBasicStyle, LifeCycle, SelectState,
        SlotComponent, SlotStyle, Style, SvgBasicStyle,
    },
    error::Error,
    event_option, hit_hover_in, hit_hover_out, lifecycle, play_animation,
    prop::{
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks, SlotDrawer,
        ToSlotMap, ToStateMap,
        manuel::{ACTIVE, BASIC, DISABLED, HOVER},
        traits::ToFloat,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GSelectItemBase = {{GSelectItem}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_item: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        active: Forward {duration: (AN_DURATION)},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_item: <AN_DRAW_VIEW> {}
                    }
                }

                active = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_item: <AN_DRAW_VIEW> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_item: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
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
    pub apply_slot_map: ApplySlotMap<SelectState, SelectItemPart>,
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
    pub state: SelectState,
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

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        self.set_animation(cx);
        cx.global::<ComponentAnInit>().select_item = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl LiveHook for GSelectItem {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let svg_props = SvgBasicStyle::live_props();
        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(active),
                live_id!(disabled),
            ],
            [
                (SelectItemPart::Icon, &svg_props),
                (SelectItemPart::Text, &LabelBasicStyle::live_props()),
                (SelectItemPart::Suffix, &svg_props),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(SelectState::Basic, applys);
                }
                HOVER => {
                    component.apply_slot_map.insert(SelectState::Hover, applys);
                }
                ACTIVE => {
                    component.apply_slot_map.insert(SelectState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(SelectState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<SelectState> for GSelectItem {
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

    type State = SelectState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.select_item;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(SelectState::Disabled);
        } else {
            if self.active {
                self.switch_state(SelectState::Active);
            } else {
                self.switch_state(SelectState::Basic);
            }
        }
        let state = self.state;
        let style = self.style.get(state);
        self.draw_item.merge(&style.container);
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, SelectState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        // as a select item we need to dispatch action to parent select, see `self.handle_event_with_action`
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
        let init_global = cx.global::<ComponentAnInit>().select_item;
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
            let basic_prop = self.style.get(SelectState::Basic);
            let hover_prop = self.style.get(SelectState::Hover);
            let active_prop = self.style.get(SelectState::Active);
            let disabled_prop = self.style.get(SelectState::Disabled);
            let (mut basic_index, mut hover_index, mut active_index, mut disabled_index) =
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
                    live_id!(active).as_instance(),
                ],
            ) {
                active_index = Some(index);
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
                nodes: draw_item = {
                    basic_index => {
                        background_color => basic_prop.container.background_color,
                        border_color =>basic_prop.container.border_color,
                        border_radius => basic_prop.container.border_radius,
                        border_width =>(basic_prop.container.border_width as f64),
                        shadow_color => basic_prop.container.shadow_color,
                        spread_radius => (basic_prop.container.spread_radius as f64),
                        blur_radius => (basic_prop.container.blur_radius as f64),
                        shadow_offset => basic_prop.container.shadow_offset,
                        background_visible => basic_prop.container.background_visible.to_f64()
                    },
                    hover_index => {
                        background_color => hover_prop.container.background_color,
                        border_color => hover_prop.container.border_color,
                        border_radius => hover_prop.container.border_radius,
                        border_width => (hover_prop.container.border_width as f64),
                        shadow_color => hover_prop.container.shadow_color,
                        spread_radius => (hover_prop.container.spread_radius as f64),
                        blur_radius => (hover_prop.container.blur_radius as f64),
                        shadow_offset => hover_prop.container.shadow_offset,
                        background_visible => hover_prop.container.background_visible.to_f64()
                    },
                    active_index => {
                        background_color => active_prop.container.background_color,
                        border_color => active_prop.container.border_color,
                        border_radius => active_prop.container.border_radius,
                        border_width => (active_prop.container.border_width as f64),
                        shadow_color => active_prop.container.shadow_color,
                        spread_radius => (active_prop.container.spread_radius as f64),
                        blur_radius => (active_prop.container.blur_radius as f64),
                        shadow_offset => active_prop.container.shadow_offset,
                        background_visible => active_prop.container.background_visible.to_f64()
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
                        background_visible => disabled_prop.container.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                SelectState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                SelectState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                SelectState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(active).as_instance(),
                    ],
                ),
                SelectState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_item = {
                    index => {
                        background_color => style.container.background_color,
                        border_color => style.container.border_color,
                        border_radius => style.container.border_radius,
                        border_width => (style.container.border_width as f64),
                        shadow_color => style.container.shadow_color,
                        spread_radius => (style.container.spread_radius as f64),
                        blur_radius => (style.container.blur_radius as f64),
                        shadow_offset => style.container.shadow_offset,
                        background_visible => style.container.background_visible.to_f64()
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

impl GSelectItem {
    active_event! {
        active_hover_in: SelectItemEvent::HoverIn |meta: FingerHoverEvent| => SelectItemHoverIn { meta },
        active_hover_out: SelectItemEvent::HoverOut |meta: FingerHoverEvent| => SelectItemHoverOut { meta }
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    SelectItemEvent::Clicked(SelectItemClicked {
                        active: self.active,
                        value: self.value.to_string(),
                        meta,
                    }),
                );
            });
        }
    }
    event_option! {
        hover_in: SelectItemEvent::HoverIn => SelectItemHoverIn,
        hover_out: SelectItemEvent::HoverOut => SelectItemHoverOut,
        clicked: SelectItemEvent::Clicked => SelectItemClicked
    }

    pub fn toggle(&mut self, cx: &mut Cx, active: bool, init: bool) -> () {
        self.active = active;

        let (state, hover_id) = match (active, init) {
            (true, false) => (SelectState::Active, Some(id!(hover.active))),
            (true, true) => (SelectState::Active, None),
            (false, true) => (SelectState::Basic, None),
            (false, false) => (SelectState::Basic, Some(id!(hover.off))),
        };
        self.switch_state(state);
        if let Some(hover_id) = hover_id {
            self.play_animation(cx, hover_id);
        }

        self.active_clicked(cx, None);
    }

    pub fn handle_event_with_action(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,
        dispatch_action: &mut dyn FnMut(&mut Cx, SelectItemEvent),
    ) {
        animation_open_then_redraw!(self, cx, event);
        if !self.active {
            match event.hits_with_options(
                cx,
                self.area(),
                HitOptions::new().with_sweep_area(sweep_area),
            ) {
                Hit::FingerDown(_) => {
                    if self.grab_key_focus {
                        cx.set_key_focus(sweep_area);
                    }
                }
                Hit::FingerHoverIn(e) => {
                    cx.set_cursor(self.style.get(self.state).container.cursor);
                    self.switch_state_with_animation(cx, SelectState::Hover);
                    hit_hover_in!(self, cx, e);
                }
                Hit::FingerHoverOut(e) => {
                    self.switch_state_with_animation(cx, SelectState::Basic);
                    hit_hover_out!(self, cx, e);
                }
                Hit::FingerUp(e) => {
                    self.active = true;
                    if !e.is_sweep {
                        dispatch_action(
                            cx,
                            SelectItemEvent::Clicked(SelectItemClicked {
                                meta: Some(e),
                                active: self.active,
                                value: self.value.to_string(),
                            }),
                        );
                    }
                    self.switch_state_with_animation(cx, SelectState::Active);
                }
                _ => {}
            }
        }
    }
}
