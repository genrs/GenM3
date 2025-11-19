mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{
        BasicStyle, Component, GButton, InputChanged, InputChangedMetaEvent, InputFocus, InputFocusMetaEvent, InputKeyDown, InputMaxLengthReached, InputState, LifeCycle, SlotComponent, SlotStyle, Style, ViewBasicStyle, area::{GInputArea, InputAreaBasicStyle, InputAreaPart}
    },
    error::Error,
    lifecycle,
    prop::{
        ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, ToSlotMap,
        manuel::{BASIC, DISABLED},
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
    pub input:  GInputArea,
    #[live]
    pub up: GButton,
    #[live]
    pub down: GButton,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
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
    apply_items_map: ApplySlotMap<InputState, InputAreaPart>,
    // ----------------------------------
    #[live]
    pub value: Vec<String>,
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
        self.draw_verification.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_verification.redraw(cx);
        for (_, item) in self.item.iter_mut() {
            if item.visible {
                item.redraw(cx);
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

impl Widget for GNumberInput {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_verification.begin(cx, walk, style.layout());

        for (_id, input) in self.item.iter_mut() {
            input.apply_slot_map = self.apply_items_map.clone();
            input.focus_sync();
            let walk = input.walk(cx);
            let _ = input.draw_walk(cx, scope, walk);
        }

        self.draw_verification.end(cx);
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

        for (_id, item) in self.item.iter_mut() {
            item.handle_event(cx, event, scope);
        }
    }
}

impl MatchEvent for GNumberInput {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // 捕获子输入框中的事件
        // 当输入框内容变化时，触发 changed 事件，可以知道是哪个输入框变化了来更新 value
        // 并且如果当某个输入框无法输入时，自动将焦点切换到下一个输入框，此时无法输入的输入框回返回一个max_length事件
        let mut reapply = None;
        let mut changed_event = None;
        for (index, (_id, item)) in self.item.iter_mut().enumerate() {
            if let Some(InputChanged { value, meta }) = item.changed(actions) {
                if !value.is_empty() {
                    self.value[index] = value;
                    changed_event = Some(meta);
                }
            }
            // 聚焦事件，聚焦事件的目的是为了存储当前聚焦的输入框位置，以便后续操作，这样当reach max length时可以自动切换焦点
            if let Some(InputFocus { meta, .. }) = item.focus(actions) {
                if let Some(InputFocusMetaEvent::FingerDown(FingerDownEvent { abs, .. })) = meta {
                    self.abs = (abs, index);
                }
            }

            if let Some(InputMaxLengthReached { value, new_input }) =
                item.max_length_reached(actions)
            {
                self.value[index] = value;
                // 将这个new_input设置到self.value中，然后重新apply_items
                self.value[index + 1] = new_input;
                if index + 1 >= self.length as usize {
                    // 已经是最后一个输入框了，无法切换焦点
                    continue;
                } else {
                    reapply = Some(index + 1);
                }
            }

            // 删除事件, 对应位置删除即可
            if let Some(InputKeyDown { value, meta }) = item.backspace(actions) {
                self.value[index] = value;
                if let Some(delete) = meta {
                    changed_event = Some(InputChangedMetaEvent::Delete(delete));
                }
            }
        }
        if let Some(re_index) = reapply {
            self.item[re_index].1.set_text(cx, &self.value[re_index]);
            self.redraw(cx);

            // 计算出这个输入框的绝对位置，然后设置焦点, y = self.abs.0.y, 从self.abs中可以知道到底从哪个输入框开始聚焦的
            let input_size = self.item[re_index].1.walk(cx).width;
            if let Size::Fixed(input_width) = input_size {
                let spacing = self.style.get(self.state).container.spacing;
                let abs_x =
                    self.abs.0.x + (input_width + spacing) * (re_index as f64 - self.abs.1 as f64);
                let abs = DVec2 {
                    x: abs_x,
                    y: self.abs.0.y,
                };
                // 设置焦点
                self.item[re_index].1.do_focus(cx, abs);
            }
        }

        if changed_event.is_some() {
            self.active_changed(cx, changed_event);
        }
    }
}

impl LiveHook for GNumberInput {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if nodes[index].is_instance_prop() {
                    if apply.from.is_update_from_doc() {
                        self.live_update_order.push(id);
                    }

                    if let Some((_, node)) = self.item.iter_mut().find(|(id2, _)| *id2 == id) {
                        node.apply(cx, apply, index, nodes)
                    } else {
                        self.item.push((id, GInputArea::new(cx)));
                        self.item
                            .last_mut()
                            .unwrap()
                            .1
                            .apply(cx, apply, index, nodes)
                    }
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                    nodes.skip_node(index)
                }
            }
            _ => nodes.skip_node(index),
        }
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [live_id!(basic), live_id!(disabled)],
            [
                (NumberInputPart::Container, &ViewBasicStyle::live_props()),
                (NumberInputPart::Item, &InputAreaBasicStyle::live_props()),
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
        ()
    }
}

impl Component for GNumberInput {
    type Error = Error;

    type State = NumberInputState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.verification;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(NumberInputState::Disabled);
        }
        let style = self.style.get(self.state).container;
        self.draw_verification.merge(&style);
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
        crossed_map.remove(&NumberInputPart::Item).map(|map| {
            self.apply_items_map.merge_slot(map.to_slot());
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
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<InputChangedMetaEvent>) {
        if self.event_open {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    NumberInputEvent::Changed(NumberInputChanged {
                        meta,
                        value: self.value.clone(),
                        length: self.length as usize,
                    }),
                );
            }
        }
    }
    pub fn apply_items(&mut self, cx: &mut Cx) {
        self.item.clear();
        for _ in 0..self.length {
            let mut input = GInputArea::new_from_ptr(cx, self.input);
            input.style.basic = self.style.basic.item;
            // 应该只替换walk和layout数据，其他的保持默认
            input.style.empty = self.style.basic.item;
            input.style.hover = self.style.basic.item;
            input.style.focus = self.style.basic.item;
            input.style.disabled = self.style.disabled.item;
            // value 填充
            if !self.value.is_empty() {
                input.value = self.value.get(self.item.len()).cloned().unwrap_or_default();
            }
            let id = live_id!(item);
            self.item.push((id, input));
        }
    }
}
