pub mod controller;
mod event;
mod prop;
mod register;

use std::f32;

pub use event::*;
pub use prop::*;
pub use register::register as number_input_register;

use makepad_widgets::*;

use crate::{
    components::{
        BasicStyle, Component, GComponent, InputChanged, InputChangedMetaEvent, LifeCycle,
        SlotComponent, SlotStyle, Style, ViewBasicStyle,
        area::{GInputArea, InputAreaBasicStyle},
        controller::{GNumberCtr, NumberCtrBasicStyle},
    },
    error::Error,
    lifecycle,
    prop::{
        ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks, ToSlotMap,
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
    pub input: GInputArea,
    #[live]
    pub ctr: GNumberCtr,
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
    #[live(1.0)]
    pub step: f32,
    #[live(0.0)]
    pub min: f32,
    #[live(100.0)]
    pub max: f32,
    /// 严格模式
    /// - value必须在min和max之间
    /// - value必须是step的整数倍
    /// - 当用户输入的value不符合要求时，会自动调整到符合要求的值
    #[live(true)]
    pub strict: bool,
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
        if self.ctr.visible {
            self.ctr.redraw(cx);
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

        let mut slots: [(LiveId, GComponent); 2] = [
            (live_id!(input), (&mut self.input).into()),
            (live_id!(ctr), (&mut self.ctr).into()),
        ];
        let mut real_height = 0.0;
        let mut draw_ctr = true;
        let ctr_width = slots[1].1.walk(cx).width;
        self.defer_walks.clear();
        // 由于makepad中没有反向绘制的机制，所以这里需要手动处理，为了完整性，看起来比较麻烦
        // 实际上：1. 不直接先绘制input，将它放到defer中；
        // 2. defer会传递真正的宽度，减去ctr的宽度,来确定真正的input宽度，完成input绘制后获取input的高度，最后用这个高度来绘制ctr
        // 后续作为TODO，可以考虑在makepad中增加反向绘制的机制，从而简化这里的逻辑，如果makepad支持反向绘制较为困难可以尝试在SlotDrawer中增加支持
        for (id, component) in &mut slots {
            if component.visible() {
                let mut walk = component.walk(cx);
                if let Some(fw) = cx.defer_walk(walk) {
                    // if is fill, defer the walk
                    self.defer_walks.push((*id, fw));
                } else {
                    if *id == live_id!(ctr) {
                        // 需要跳过，因为input的高度还没有确定
                        if real_height == 0.0 {
                            draw_ctr = false;
                            continue;
                        } else {
                            walk.height = Size::Fixed(real_height);
                        }
                    }
                    let _ = component.draw_walk(cx, scope, walk);

                    if *id == live_id!(input) {
                        real_height = component.area().rect(cx).size.y;
                    }
                }
            }
        }
        let mut new_defer_walks = DeferWalks::new();
        for (id, df_walk) in self.defer_walks.iter_mut() {
            for (slot_id, slot) in &mut slots {
                if *id == *slot_id {
                    let mut res_walk = df_walk.resolve(cx);
                    if *id == live_id!(ctr) {
                        if real_height == 0.0 {
                            // 如果依然是0.0说明input没有显示出来，我们需要再次加入到defer队列
                            new_defer_walks.push((*id, df_walk.clone()));
                            break;
                        } else {
                            res_walk.height = Size::Fixed(real_height);
                        }
                    } else if *id == live_id!(input) {
                        if let Size::Fixed(w) = res_walk.width {
                            if let Size::Fixed(cw) = ctr_width {
                                if w >= cw {
                                    res_walk.width = Size::Fixed(w - cw - style.container.spacing);
                                }
                            }
                        }
                    }
                    let _ = slot.draw_walk(cx, scope, res_walk);
                    if *id == live_id!(input) {
                        real_height = slot.area().rect(cx).size.y;
                    }
                    break;
                }
            }
        }

        if !draw_ctr {
            if self.ctr.visible {
                let mut walk = self.ctr.walk(cx);
                walk.height = Size::Fixed(real_height);
                let _ = self.ctr.draw_walk(cx, scope, walk);
            }
        }

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

        if self.ctr.visible {
            self.ctr.handle_event(cx, event, scope);
        }
    }
}

impl MatchEvent for GNumberInput {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // 接收input的事件，输入事件需要进行特殊处理，因为input只是担任输入框的角色，没有实际的输入逻辑
        if let Some(param) = self.input.changed(actions) {
            self.handle_changed(cx, param);
        }

        if let Some(_) = self.ctr.up(actions) {
            self.handle_adjust(cx, NumberInputAdjust::Up);
        }
        if let Some(_) = self.ctr.down(actions) {
            self.handle_adjust(cx, NumberInputAdjust::Down);
        }
    }
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
                (NumberInputPart::Ctr, &NumberCtrBasicStyle::live_props()),
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

        self.apply_data(cx);
    }
}

impl SlotComponent<NumberInputState> for GNumberInput {
    type Part = NumberInputPart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.ctr.style.basic = self.style.basic.ctr;
        self.ctr.style.disabled = self.style.disabled.ctr;
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

        crossed_map.remove(&NumberInputPart::Ctr).map(|map| {
            self.ctr.apply_slot_map.merge_slot(map.to_slot());
            self.ctr.focus_sync();
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
    pub fn handle_adjust(&mut self, cx: &mut Cx, adjust: NumberInputAdjust) {
        match adjust {
            NumberInputAdjust::Up => {
                self.value += self.step;
                if self.strict {
                    if self.value > self.max {
                        self.value = self.max;
                    }
                    self.value = self.value.clamp(f32::MIN, f32::MAX);
                }
                self.active_changed(cx, None, adjust);
            }
            NumberInputAdjust::Down => {
                self.value -= self.step;
                if self.strict {
                    if self.value < self.min {
                        self.value = self.min;
                    }
                    self.value = self.value.clamp(f32::MIN, f32::MAX);
                }
                self.active_changed(cx, None, adjust);
            }
            NumberInputAdjust::Clear => {
                unreachable!("Clear adjust is not supported in GNumberInput now");
            }
        }
        self.apply_data(cx);
        self.input.redraw(cx);
    }
    pub fn handle_changed(&mut self, cx: &mut Cx, param: InputChanged) {
        let mut adjust = NumberInputAdjust::Up; // 默认是向上调整
        if let Ok(value) = param.value.parse::<f32>() {
            if value == self.value {
                return; // 如果值没有变化，则不处理
            }

            if value < self.value {
                adjust = NumberInputAdjust::Down; // 如果小于最小值，则向下调整
            }

            if self.strict {
                if value < self.min
                    || value > self.max
                    || (value - self.min).abs() % self.step != 0.0
                {
                    // 如果不符合严格模式的要求，则调整到符合要求的值
                    let adjusted_value =
                        ((value - self.min) / self.step).round() * self.step + self.min;
                    self.value = adjusted_value.clamp(self.min, self.max);
                } else {
                    self.value = value;
                }
            } else {
                self.value = value;
            }
        } else {
            // 如果解析失败，保持原值
            return;
        }
        self.active_changed(cx, Some(param.meta), adjust);
        self.apply_data(cx);
        self.input.redraw(cx);
    }
    // 用于将数据应用到各个子组件上
    pub fn apply_data(&mut self, cx: &mut Cx) {
        self.input.set_text(cx, &self.value.to_string());
    }
}
