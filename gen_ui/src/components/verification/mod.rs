mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{
        BasicStyle, Component, GInput, InputState, LifeCycle, SlotComponent, SlotStyle, Style,
        ViewBasicStyle,
        area::{GInputArea, InputAreaBasicStyle, InputAreaPart},
    },
    error::Error,
    lifecycle,
    prop::{
        ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, ApplyStateMap, ToSlotMap,
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

    pub GVerificationBase = {{GVerification}} {

    }
}

/// # Verification Code
/// can display multiple boxes for input verification code
/// ## Display
/// ```
///  ┌───┐ ┌───┐ ┌───┐ ┌───┐
///  │ x │ │ x │ │ x │ │   │
///  └───┘ └───┘ └───┘ └───┘
/// ```
#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GVerification {
    #[live]
    pub style: VerificationStyle,
    #[live(4)]
    pub length: i32,
    #[live]
    pub input: Option<LivePtr>,
    #[rust]
    pub item: Vec<(LiveId, GInputArea)>,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    #[live]
    pub draw_verification: DrawView,
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
    pub apply_slot_map: ApplySlotMap<VerificationState, VerificationPart>,
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
    pub state: VerificationState,
    #[rust]
    apply_items_map: ApplySlotMap<InputState, InputAreaPart>,
    // ---
    /// 值，被一个输入框对应进行填充，Vec的长度和 length 保持一致
    #[live]
    pub value: Vec<String>,
}

impl WidgetNode for GVerification {
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

impl Widget for GVerification {
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

impl MatchEvent for GVerification {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // 捕获子输入框中的事件
        // 当输入框内容变化时，触发 changed 事件，可以知道是哪个输入框变化了来更新 value
        // 并且如果当某个输入框无法输入时，自动将焦点切换到下一个输入框，此时无法输入的输入框回返回一个max_length事件
        for (_id, item) in self.item.iter_mut() {
            if let Some(e) = item.changed(actions) {

            }

            if let Some(e) = item.max_length_reached(actions) {
                // 自动切换焦点到下一个输入框
                // let current_index = self.item.iter().position(|(_id, input)| {
                //     input.scope_path == item.scope_path
                // });
                // if let Some(index) = current_index {
                //     if index + 1 < self.item.len() {
                //         let next_input = &mut self.item[index + 1].1;
                //         next_input.focus(cx);
                //     }
                // }
                dbg!(e);
            }
        }
    }
}

impl LiveHook for GVerification {
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
                (VerificationPart::Container, &ViewBasicStyle::live_props()),
                (VerificationPart::Item, &InputAreaBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_slot_map
                        .insert(VerificationState::Basic, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(VerificationState::Disabled, applys);
                }
                _ => {}
            },
        );

        self.apply_items(cx);
    }
}

impl SlotComponent<VerificationState> for GVerification {
    type Part = VerificationPart;

    fn merge_prop_to_slot(&mut self) -> () {
        ()
    }
}

impl Component for GVerification {
    type Error = Error;

    type State = VerificationState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.verification;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(VerificationState::Disabled);
        }
        let style = self.style.get(self.state).container;
        self.draw_verification.merge(&style);
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        ()
    }

    fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
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
        crossed_map.remove(&VerificationPart::Item).map(|map| {
            self.apply_items_map.merge_slot(map.to_slot());
        });

        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        ()
    }

    sync!();
    set_scope_path!();
    set_index!();
    lifecycle!();
    switch_state!();
}

impl GVerification {
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
