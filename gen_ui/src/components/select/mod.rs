mod event;
pub mod item;
pub mod options;
mod prop;
mod register;

use std::{cell::RefCell, rc::Rc};

pub use event::*;
pub use prop::*;
pub use register::register as select_register;

use makepad_widgets::*;

use crate::{
    active_event, animation_open_then_redraw, components::{
        item::{GSelectItem, SelectItemBasicStyle}, options::GSelectOptions, BasicStyle, Component, GComponent, GLabel, GSvg, GView, LabelBasicStyle, LifeCycle, PopupComponent, SlotComponent, SlotStyle, Style, SvgBasicStyle, ViewBasicStyle
    }, error::Error, event_option, hit_hover_in, hit_hover_out, lifecycle, play_animation, prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER}, traits::ToFloat, ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks, SlotDrawer, ToSlotMap, ToStateMap
    }, pure_after_apply, set_animation, set_index, set_scope_path, shader::draw_view::DrawView, sync, themes::conf::Conf, visible, ComponentAnInit
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GSelectBase = {{GSelect}} {}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSelect {
    #[live]
    pub style: SelectStyle,
    #[live]
    pub draw_select: DrawView,
    #[live]
    pub item: GSelectItem,
    #[live]
    pub select_options: Option<LivePtr>,
    #[live]
    pub option: Option<LivePtr>,
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
    pub state: SelectState,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live(true)]
    pub open: bool,
    #[live]
    pub selected: u32,
    #[live]
    pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_slot_map: ApplySlotMap<SelectState, SelectPart>,
    #[rust]
    defer_walks: DeferWalks,
    #[rust(true)]
    pub redraw_flag: bool,
}

#[derive(Default, Clone)]
pub struct SelectOptionsGlobal {
    pub map: Rc<RefCell<ComponentMap<LivePtr, GSelectOptions>>>,
}

impl WidgetNode for GSelect {
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
        self.draw_select.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_select.redraw(cx);
        if self.prefix.visible {
            self.prefix.redraw(cx);
        }
        if self.item.visible {
            self.item.redraw(cx);
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

impl Widget for GSelect {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let style = self.style.get(self.state);
        let _ = self.draw_select.begin(cx, walk, style.layout());
        self.item.as_item = true;
        let real_height = self.count_real_height(cx);
        let mut slots: [(LiveId, GComponent); 3] = [
            (live_id!(prefix), (&mut self.prefix).into()),
            (live_id!(item), (&mut self.item).into()),
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

        let _ = self.draw_select.end(cx);
        cx.add_nav_stop(self.area(), NavRole::DropDown, Margin::default());
        // draw options menu
        if self.open && self.select_options.is_some() {
            let global = cx.global::<SelectOptionsGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let options_menu = map.get_mut(&self.select_options.unwrap()).unwrap();
            let mut options_walk = options_menu.walk();
            options_walk.width = Size::Fixed(self.area().rect(cx).size.x);
            options_menu.begin(cx, options_walk);
            options_menu.draw_popup(cx, scope, None, 0.0, &mut self.redraw_flag);
            // options_menu.item = self.option;
            let area = self.area().rect(cx);
            let shift = DVec2 {
                x: 0.0,
                y: area.size.y + 2.0,
            };
            options_menu.end(cx, scope, self.area(), shift);
        }

        self.set_scope_path(&scope.path);
        return DrawStep::done();
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }
        self.set_animation(cx);
        cx.global::<ComponentAnInit>().select = true;
        let area = self.area();
        let hit = event.hits(cx, area);

        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            let uid = self.widget_uid();
            // self.handle_widget_event(cx, event, hit, area);
            if self.open && self.select_options.is_some() {
                let global = cx.global::<SelectOptionsGlobal>().clone();
                let mut map = global.map.borrow_mut();
                let select_options = map.get_mut(&self.select_options.unwrap()).unwrap();
                // select_options.handle_event_with(cx, event, scope, self.area());
                let mut active_index = None;
                select_options.handle_event_with_action(
                    cx,
                    event,
                    self.area(),
                    &mut |cx, select_event| match select_event {
                        SelectOptionsEvent::Changed(e) => {
                            self.value = e.value.to_string();
                            active_index = Some(e.index);
                            // pub real select event
                            cx.widget_action(uid, &scope.path, SelectEvent::Changed(e));

                            // self.close_inner(cx, false);
                        }
                        _ => {}
                    },
                );

                if let Some(index) = active_index {
                    self.selected = index as u32;
                    self.item
                        .clone_from_ptr(cx, &select_options.children.get(index).unwrap().1);
                    self.redraw(cx);
                }

                if let Event::MouseDown(e) = event {
                    let is_in = select_options.menu_contains_pos(cx, e.abs);
                    self.switch_state_with_animation(cx, SelectState::Basic);
                    self.close_inner(cx, is_in);
                    return;
                }
            }
            match event.hits_with_sweep_area(cx, self.area(), self.area()) {
                Hit::FingerHoverIn(_) => {
                    cx.set_cursor(self.style.get(self.state).container.cursor);
                    self.switch_state_with_animation(cx, SelectState::Hover);
                }
                Hit::FingerHoverOut(_) => {
                    cx.set_cursor(Default::default());
                    self.switch_state_with_animation(cx, SelectState::Basic);
                }
                Hit::FingerUp(e) => {
                    if e.is_over && e.device.has_hovers() {
                        self.switch_state_with_animation(cx, SelectState::Active);
                        self.open_inner(cx);
                    }
                }
                _ => {}
            }
        }
    }
}

impl LiveHook for GSelect {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let global = cx.global::<SelectOptionsGlobal>().clone();
        let mut map = global.map.borrow_mut();
        map.retain(|k, _| cx.live_registry.borrow().generation_valid(*k));
        let menu = self.select_options.unwrap();
        map.get_or_insert(cx, menu, |cx| GSelectOptions::new_from_ptr(cx, Some(menu)));

        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [
                live_id!(active),
                live_id!(basic),
                live_id!(hover),
                live_id!(disabled),
            ],
            [
                (SelectPart::Container, &ViewBasicStyle::live_props()),
                (SelectPart::Select, &SelectItemBasicStyle::live_props()),
                (SelectPart::Prefix, &ViewBasicStyle::live_props()),
                (SelectPart::Suffix, &ViewBasicStyle::live_props()),
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

impl SlotComponent<SelectState> for GSelect {
    type Part = SelectPart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.item.style.basic = self.style.basic.item;
        self.item.style.hover = self.style.hover.item;
        self.item.style.active = self.style.active.item;
        self.item.style.disabled = self.style.disabled.item;

        self.prefix.style.basic = self.style.basic.prefix;
        self.prefix.style.hover = self.style.hover.prefix;
        self.prefix.style.pressed = self.style.active.prefix;
        self.prefix.style.disabled = self.style.disabled.prefix;

        self.suffix.style.basic = self.style.basic.suffix;
        self.suffix.style.hover = self.style.hover.suffix;
        self.suffix.style.pressed = self.style.active.suffix;
        self.suffix.style.disabled = self.style.disabled.suffix;
    }
}

impl Component for GSelect {
    type Error = Error;

    type State = SelectState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.select;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(SelectState::Disabled);
        } else {
            if self.open {
                self.switch_state(SelectState::Active);
            } else {
                self.switch_state(SelectState::Basic);
            }
        }
        let state = self.state;
        let style = self.style.get(state);
        self.draw_select.merge(&style.container);
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

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        // animation_open_then_redraw!(self, cx, event);

        // match hit {
        //     Hit::FingerDown(_) => {
        //         if self.grab_key_focus {
        //             cx.set_key_focus(area);
        //         }
        //     }
        //     Hit::FingerHoverIn(e) => {
        //         cx.set_cursor(self.style.get(self.state).container.cursor);
        //         self.switch_state_with_animation(cx, SelectState::Hover);
        //         // hit_hover_in!(self, cx, e);
        //     }
        //     Hit::FingerHoverOut(e) => {
        //         self.switch_state_with_animation(cx, SelectState::Basic);
        //         // hit_hover_out!(self, cx, e);
        //     }
        //     Hit::FingerUp(e) => {
        //         if e.is_over {
        //             if e.has_hovers() {
        //                 self.open = true;
        //                 self.switch_state_with_animation(cx, SelectState::Active);
        //                 self.play_animation(cx, id!(hover.active));
        //             } else {
        //                 self.switch_state_with_animation(cx, SelectState::Basic);
        //                 self.play_animation(cx, id!(hover.off));
        //             }
        //             // self.active_clicked(cx, Some(e));
        //         } else {
        //             self.switch_state_with_animation(cx, SelectState::Basic);
        //         }
        //     }
        //     _ => {}
        // }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.item.switch_state(state.into());
        self.prefix.switch_state(state.into());
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
            (SelectPart::Prefix, &mut self.prefix),
            (SelectPart::Suffix, &mut self.suffix),
        ] {
            crossed_map.remove(&part).map(|map| {
                slot.apply_state_map.merge(map.to_state());
                slot.focus_sync();
            });
        }

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
                nodes: draw_select = {
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
                nodes: draw_select = {
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

impl GSelect {
    fn close_inner(&mut self, cx: &mut Cx, is_in: bool) {
        if !self.open || is_in {
            return;
        }

        self.open = false;
        self.redraw(cx);
        cx.sweep_unlock(self.area());
        // self.active_toggled(cx, e_kind);
    }

    fn open_inner(&mut self, cx: &mut Cx) {
        if self.open {
            return;
        }

        self.open = true;
        self.redraw(cx);
        cx.sweep_lock(self.area());
    }

    pub fn count_real_height(&self, cx: &mut Cx) -> f64 {
        let font_metrics = cx.global::<Conf>().theme.font.metrics;
        let style = self.style.get(self.state);
        let text_style = style.item.text;
        let padding = text_style.padding.top
            + text_style.padding.bottom
            + style.item.container.padding.top
            + style.item.container.padding.bottom
            + style.container.padding.top
            + style.container.padding.bottom;
        let margin = text_style.margin.top
            + text_style.margin.bottom
            + style.item.container.margin.top
            + style.item.container.margin.bottom;

        ((text_style.font_size * font_metrics) as f64) + padding + margin + 0.8
    }
}
