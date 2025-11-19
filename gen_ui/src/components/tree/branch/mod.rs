mod prop;

pub use prop::*;

use super::event::*;

use crate::{
    ComponentAnInit, area,
    components::{
        BasicStyle, Component, GLabel, GSvg, GView, LabelBasicStyle, LifeCycle, SlotComponent,
        SlotStyle, Style, SvgBasicStyle, ViewBasicStyle,
    },
    error::Error,
    event_option, getter, lifecycle, play_animation,
    prop::{
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ToStateMap,
        manuel::{ACTIVE, BASIC, DISABLED},
        traits::{NewFrom, ToFloat},
    },
    pure_after_apply, set_animation, set_index, set_scope_path, setter,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible,
};
use makepad_widgets::*;


live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GBranchBase = {{GBranch}} {
        animator: {
            active = {
                default: off
                off = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    apply: {
                        draw_branch: <AN_DRAW_VIEW> {},
                        fold: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                    }
                }
                on = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    apply: {
                        draw_branch: <AN_DRAW_VIEW> {},
                        fold: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]
                    }
                }
                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    ease: InOutQuad,
                    apply: {
                        draw_branch: <AN_DRAW_VIEW> {},
                        fold: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                    }
                }
            }
        }
    }
}

/// A branch node in the tree
/// branch is act as a collapsible container for leaf nodes
/// ## Display
/// ```
/// branch              ---> branch can contain leaf nodes and other branch nodes
///     leaf
///     leaf
///     ...
///     branch          ---> branch can be nested
///         leaf
/// ---
/// ┌──────────────────────────┐
/// │ ┌───────────────────┐    │
/// │ │ svg  text         │    │  ---> branch header
/// │ └───────────────────┘    │
/// │ ┌───────────────────────┐│
/// │ │ padding │ leaf/branch ││  ---> branch body
/// │ └───────────────────────┘│
/// └──────────────────────────┘  
/// ```
/// ## Example
/// ```
/// <GBranch> {
///     text: <GLabel> { text: "Branch 1" }
///     body: <GView> {
///         <GLeaf> {}
///         <GLeaf> {}
///         <GBranch> {
///             text: <GLabel> { text: "Branch 1.1" }
///         }
///     }
/// }
/// ```
#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GBranch {
    #[live]
    pub style: BranchStyle,
    #[live]
    pub icon: GSvg,
    #[live]
    pub text: GLabel,
    #[live]
    pub body: GView,
    #[live]
    pub draw_branch: DrawView,
    #[live]
    pub active: bool,
    #[live]
    pub value: String,
    #[live]
    pub fold: f64,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    #[rust]
    pub area_header: Area,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    // use animation counter to prevent multiple animations
    #[rust(true)]
    animation_counter: bool,
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
    pub state: BranchState,
    #[rust]
    pub draw_state: DrawStateWrap<DrawBranchState>,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_slot_map: ApplySlotMap<BranchState, BranchPart>,
}

impl Widget for GBranch {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get_mut(self.state);
        self.fold = self.active.to_f64();
        let body_walk = self.body.walk(cx);
        let icon_walk = self.icon.walk(cx);
        let text_walk = self.text.walk(cx);
        self.draw_branch.begin(cx, walk, style.layout());
        if self.draw_state.begin(cx, DrawBranchState::DrawHeader) {
            cx.begin_turtle(
                Walk {
                    abs_pos: None,
                    margin: Margin::default(),
                    width: Size::Fill,
                    height: Size::Fit,
                },
                Layout {
                    flow: Flow::Right,
                    padding: Padding::from_xy(0.0, 4.0),
                    spacing: 12.0,
                    align: Align {
                        x: 0.0,
                        y: 0.5,
                    },
                    ..Default::default()
                },
            );

            if self.icon.visible {
                let _ = self.icon.draw_walk(cx, scope, icon_walk);
            }

            if self.text.visible {
                let _ = self.text.draw_walk(cx, scope, text_walk);
            }

            cx.end_turtle_with_area(&mut self.area_header);

            self.draw_state.set(DrawBranchState::DrawBody);
        }

        if let Some(DrawBranchState::DrawBody) = self.draw_state.get() {
            if self.fold == 1.0 {
                self.animator_play(cx, id!(active.on));
                let _ = self.body.draw_walk(cx, scope, body_walk);
            } else {
                self.animator_play(cx, id!(active.off));
            }
        }
        self.draw_branch.end(cx);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible {
            return;
        }
        self.set_animation(cx);
        cx.global::<ComponentAnInit>().branch = true;
        let hit = event.hits(cx, sweep_area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, sweep_area);
            if self.active {
                self.body.handle_event(cx, event, scope);
            }
        }
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }
        self.set_animation(cx);
        cx.global::<ComponentAnInit>().branch = true;
        let area = self.area_header;
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
            if self.active {
                self.body.handle_event(cx, event, scope);
            }
        }
    }
}

impl WidgetNode for GBranch {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for (_, child) in self.body.children.iter() {
            let x = child.uid_to_widget(uid);
            if !x.is_empty() {
                return x;
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for (_, child) in &self.body.children {
            child.find_widgets(path, cached, results);
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.container.walk()
    }

    fn area(&self) -> Area {
        if self.active {
            self.draw_branch.area
        } else {
            self.area_header
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_branch.redraw(cx);
        if self.icon.visible {
            self.icon.redraw(cx);
        }
        if self.text.visible {
            self.text.redraw(cx);
        }
        if self.body.visible {
            self.body.redraw(cx);
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

impl LiveHook for GBranch {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let live_props = ViewBasicStyle::live_props();
        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [live_id!(basic), live_id!(active), live_id!(disabled)],
            [
                (BranchPart::Container, &live_props),
                (BranchPart::Icon, &SvgBasicStyle::live_props()),
                (BranchPart::Text, &LabelBasicStyle::live_props()),
                (BranchPart::Body, &live_props),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(BranchState::Basic, applys);
                }
                ACTIVE => {
                    component.apply_slot_map.insert(BranchState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(BranchState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<BranchState> for GBranch {
    type Part = BranchPart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.icon.style.basic = self.style.basic.icon;
        self.icon.style.pressed = self.style.active.icon;
        self.icon.style.disabled = self.style.disabled.icon;
        self.text.style.basic = self.style.basic.text;
        self.text.style.disabled = self.style.disabled.text;
        self.body.style.basic = self.style.basic.body;
        self.body.style.pressed = self.style.active.body;
        self.body.style.disabled = self.style.disabled.body;
    }
}

impl Component for GBranch {
    type Error = Error;

    type State = BranchState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.branch;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(BranchState::Disabled);
        } else {
            if self.active {
                self.switch_state(BranchState::Active);
            } else {
                self.switch_state(BranchState::Basic);
            }
        }
        let state = self.state;
        let style = self.style.get(state);
        self.draw_branch.merge(&style.container);
        let _ = self.icon.render(cx)?;
        let _ = self.text.render(cx)?;
        let _ = self.body.render(cx)?;
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, BranchState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }
    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        if !self.animation_open && self.animation_counter {
            if self.animator_handle_event(cx, event).must_redraw() {
                if self.animator.is_track_animating(cx, id!(active)) {
                    self.area().redraw(cx);
                    self.animation_counter = !self.animation_counter;
                }
            }
        }

        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(area);
                }
            }
            Hit::FingerHoverIn(_meta) => {
                cx.set_cursor(MouseCursor::Hand);
                // self.switch_state_with_animation(cx, BranchState::Hover);
                // self.active_hover_in(cx, meta);
            }
            Hit::FingerHoverOut(_meta) => {
                self.switch_state_with_animation(cx, BranchState::Basic);
                // self.active_hover_out(cx, meta);
            }
            Hit::FingerUp(meta) => {
                self.active = !self.active;
                self.fold = self.active.to_f32() as f64;
                if self.active {
                    self.switch_state_with_animation(cx, BranchState::Active);
                    self.animator_play(cx, id!(active.on));
                } else {
                    self.switch_state_with_animation(cx, BranchState::Basic);
                    self.animator_play(cx, id!(active.off));
                }
                self.active_changed(cx, Some(meta));
                self.animation_counter = true;
            }
            _ => {}
        }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        // self.header.switch_state(state.into());
        self.icon.switch_state(state.into());
        self.text.switch_state(state.into());
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

        crossed_map.remove(&BranchPart::Body).map(|map| {
            self.body.apply_state_map.merge(map.to_state());
            self.body.focus_sync();
        });
        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().branch;
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
            let basic_prop = self.style.get(BranchState::Basic);
            let active_prop = self.style.get(BranchState::Active);
            let disabled_prop = self.style.get(BranchState::Disabled);
            let (mut basic_index, mut active_index, mut disabled_index) = (None, None, None);
            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(active).as_instance(),
                    live_id!(off).as_instance(),
                ],
            ) {
                basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(active).as_instance(),
                    live_id!(on).as_instance(),
                ],
            ) {
                active_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(active).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_branch = {
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
                BranchState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                BranchState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                BranchState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_branch = {
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

impl GBranch {
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    BranchEvent::Changed(BranchChanged {
                        meta,
                        active: self.active,
                        value: self.value.to_string(),
                    }),
                );
            });
        }
    }
    event_option! {
        changed: BranchEvent::Changed => BranchChanged
    }
    area! {
        area_icon, icon,
        area_text, text,
        area_body, body
    }
    pub fn generate_value(&mut self, index_chain: &Vec<usize>) {
        if self.value.is_empty() {
            self.value = index_chain
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("_")
        }
    }
    getter! {
        GBranch {
            get_active(bool) {|c| {c.active}}
        }
    }
    setter! {
        GBranch {
            set_active(active: bool) {|c, cx| {c.active = active; c.redraw(cx); Ok(())}}
        }
    }
}

#[derive(Clone, Copy)]
pub enum DrawBranchState {
    DrawHeader,
    DrawBody,
}
