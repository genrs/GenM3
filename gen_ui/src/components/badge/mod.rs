pub mod dot;
mod event;
mod prop;
mod register;

pub use event::*;
pub use register::register as badge_register;
use std::cell::RefCell;

use crate::{
    ComponentAnInit, active_event, animation_open_then_redraw,
    components::{
        BasicStyle, Component, DrawState, LifeCycle, SlotComponent, SlotStyle, Style,
        ViewBasicStyle, ViewTextureCache,
        dot::{BadgeDotBasicStyle, GBadgeDot},
        is_texture, needs_draw_list,
    },
    do_view_livehook_pre,
    error::Error,
    event_option, hit_finger_up, hit_hover_out, inherits_view_find_widgets,
    lifecycle, play_animation,
    prop::{
        ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, Position, ToSlotMap,
        manuel::{BASIC, DISABLED},
        traits::ToFloat,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible,
};

pub use prop::*;

use makepad_widgets::{event::FingerLongPressEvent, *};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GBadgeBase = {{GBadge}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_badge: <AN_DRAW_VIEW> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_badge: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GBadge {
    #[live(true)]
    pub visible: bool,
    #[live]
    pub scroll: DVec2,
    #[live]
    pub scroll_bars: Option<LivePtr>,
    #[live]
    pub dpi_factor: Option<f64>,
    #[live]
    pub optimize: ViewOptimize,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub block_signal_event: bool,
    #[live(false)]
    pub capture_overload: bool,
    #[live]
    pub event_order: EventOrder,
    #[live(false)]
    pub event_open: bool,
    #[live]
    pub disabled: bool,
    // --- texture and cache ------
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub find_cache: RefCell<SmallVec<[(u64, WidgetSet); 3]>>,
    #[rust]
    pub scroll_bars_obj: Option<Box<ScrollBars>>,
    #[rust]
    pub view_size: Option<DVec2>,
    #[rust]
    pub area: Area,
    #[rust]
    pub draw_list: Option<DrawList2d>,
    #[rust]
    pub texture_cache: Option<ViewTextureCache>,
    #[rust]
    pub defer_walks: SmallVec<[(LiveId, DeferWalk); 1]>,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    #[rust]
    pub children: SmallVec<[(LiveId, WidgetRef); 2]>,
    #[rust]
    pub live_update_order: SmallVec<[LiveId; 1]>,
    // --- animation --------------
    #[animator]
    pub animator: Animator,
    #[live(false)]
    pub animation_open: bool,
    #[live(true)]
    pub animation_spread: bool,
    // --- lifecycle --------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub index: usize,
    #[live(true)]
    pub sync: bool,
    #[live]
    pub draw_badge: DrawView,
    #[live]
    pub style: BadgeStyle,
    #[rust]
    pub state: BadgeState,
    #[rust]
    pub apply_slot_map: ApplySlotMap<BadgeState, BadgePart>,
    #[live]
    pub dot: GBadgeDot,
    #[live(Position::TopRight)]
    pub position: Position,
    /// offset for badge dot
    #[live]
    pub offset: Vec2,
}

impl WidgetNode for GBadge {
    inherits_view_find_widgets!();

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.area
    }

    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for (_, child) in &self.children {
            let x = child.uid_to_widget(uid);
            if !x.is_empty() {
                return x;
            }
        }
        WidgetRef::empty()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.area.redraw(cx);
        self.draw_badge.redraw(cx);
        for (_, child) in &mut self.children {
            child.redraw(cx);
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

impl Widget for GBadge {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);

        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                self.draw_state.end();
                self.set_scope_path(&scope.path);
                return DrawStep::done();
            }
            self.defer_walks.clear();

            match self.optimize {
                ViewOptimize::Texture => {
                    let walk = self.walk_from_previous_size(walk);
                    if !cx.will_redraw(self.draw_list.as_mut().unwrap(), walk) {
                        if let Some(texture_cache) = self.texture_cache.as_ref() {
                            self.draw_badge
                                .draw_vars
                                .set_texture(0, &texture_cache.color_texture);
                            let mut rect = cx.walk_turtle_with_area(&mut self.area, walk);
                            // NOTE(eddyb) see comment lower below for why this is
                            // disabled (it used to match `set_pass_scaled_area`).
                            if false {
                                rect.size *= 2.0 / self.dpi_factor.unwrap_or(1.0);
                            }
                            self.draw_badge.draw_abs(cx, rect);
                            self.area = self.draw_badge.area();
                            cx.set_pass_area(&texture_cache.pass, self.area);
                        }
                        self.set_scope_path(&scope.path);
                        return DrawStep::done();
                    }
                    // lets start a pass
                    if self.texture_cache.is_none() {
                        self.texture_cache = Some(ViewTextureCache {
                            pass: Pass::new(cx),
                            _depth_texture: Texture::new(cx),
                            color_texture: Texture::new(cx),
                        });
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        //cache.pass.set_depth_texture(cx, &cache.depth_texture, PassClearDepth::ClearWith(1.0));
                        texture_cache.color_texture = Texture::new_with_format(
                            cx,
                            TextureFormat::RenderBGRAu8 {
                                size: TextureSize::Auto,
                                initial: true,
                            },
                        );
                        texture_cache.pass.set_color_texture(
                            cx,
                            &texture_cache.color_texture,
                            PassClearColor::ClearWith(vec4(0.0, 0.0, 0.0, 0.0)),
                        );
                    }
                    let texture_cache = self.texture_cache.as_mut().unwrap();
                    cx.make_child_pass(&texture_cache.pass);
                    cx.begin_pass(&texture_cache.pass, self.dpi_factor);
                    self.draw_list.as_mut().unwrap().begin_always(cx)
                }
                ViewOptimize::DrawList => {
                    let walk = self.walk_from_previous_size(walk);
                    if self
                        .draw_list
                        .as_mut()
                        .unwrap()
                        .begin(cx, walk)
                        .is_not_redrawing()
                    {
                        cx.walk_turtle_with_area(&mut self.area, walk);
                        self.set_scope_path(&scope.path);
                        return DrawStep::done();
                    }
                }
                _ => (),
            }

            // ok so.. we have to keep calling draw till we return LiveId(0)
            let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                scroll_bars.begin_nav_area(cx);
                scroll_bars.get_scroll_pos()
            } else {
                self.scroll
            };

            let layout = style.layout().with_scroll(scroll);

            if self.visible {
                self.draw_badge.begin(cx, walk, layout);
            } else {
                cx.begin_turtle(walk, layout);
            }
        }

        while let Some(DrawState::Drawing(step, resume)) = self.draw_state.get() {
            if step < self.children.len() {
                if let Some((id, child)) = self.children.get_mut(step) {
                    if child.visible() {
                        let walk = child.walk(cx);
                        child.set_disabled(cx, self.disabled);

                        if resume {
                            scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                        } else if let Some(fw) = cx.defer_walk(walk) {
                            self.defer_walks.push((*id, fw));
                        } else {
                            self.draw_state.set(DrawState::Drawing(step, true));
                            scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                        }
                    }
                }
                self.draw_state.set(DrawState::Drawing(step + 1, false));
            } else {
                self.draw_state.set(DrawState::DeferWalk(0));
            }
        }

        while let Some(DrawState::DeferWalk(step)) = self.draw_state.get() {
            if step < self.defer_walks.len() {
                let (id, dw) = &mut self.defer_walks[step];
                if let Some((id, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                    let walk = dw.resolve(cx);
                    child.set_disabled(cx, self.disabled);
                    scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.draw_scroll_bars(cx);
                };

                if self.visible {
                    if is_texture(self.optimize) {
                        panic!("dont use show_bg and texture caching at the same time");
                    }
                    self.draw_badge.end(cx);
                    self.area = self.draw_badge.area();
                } else {
                    cx.end_turtle_with_area(&mut self.area);
                };

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.set_area(self.area);
                    scroll_bars.end_nav_area(cx);
                };

                if needs_draw_list(self.optimize) {
                    let rect = self.area.rect(cx);
                    self.view_size = Some(rect.size);
                    self.draw_list.as_mut().unwrap().end(cx);

                    if is_texture(self.optimize) {
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        cx.end_pass(&texture_cache.pass);
                        self.draw_badge
                            .draw_vars
                            .set_texture(0, &texture_cache.color_texture);
                        self.draw_badge.draw_abs(cx, rect);
                        let area = self.draw_badge.area();
                        let texture_cache = self.texture_cache.as_mut().unwrap();

                        cx.set_pass_area(&texture_cache.pass, area);
                    }
                }
                self.draw_state.end();
            }
        }

        // draw badge dot for container
        if self.dot.visible {
            if self.draw_list.is_none() {
                self.draw_list = Some(DrawList2d::new(cx));
            }
            let container_area = self.area();
            let area = container_area.rect(cx);
            if let Some(draw_list) = self.draw_list.as_mut() {
                draw_list.begin_overlay_reuse(cx);
                cx.begin_pass_sized_turtle(Layout::flow_down());
                let dot_walk = self.dot.walk(cx);
                let _ = self.dot.draw_walk(cx, scope, dot_walk);
                let dot_rect = self.dot.area().rect(cx);
                let mut shift = match self.position {
                    Position::Bottom => DVec2 {
                        x: -dot_rect.size.x / 2.0 + area.size.x / 2.0,
                        y: area.size.y - dot_rect.size.y / 2.0,
                    },
                    Position::BottomLeft => DVec2 {
                        x: 0.0,
                        y: area.size.y - dot_rect.size.y / 2.0,
                    },
                    Position::BottomRight => DVec2 {
                        x: area.size.x - dot_rect.size.x,
                        y: area.size.y - dot_rect.size.y / 2.0,
                    },
                    Position::Top => DVec2 {
                        x: -dot_rect.size.x / 2.0 + area.size.x / 2.0,
                        y: -dot_rect.size.y + dot_rect.size.y / 2.0,
                    },
                    Position::TopLeft => DVec2 {
                        x: 0.0,
                        y: -dot_rect.size.y + dot_rect.size.y / 2.0,
                    },
                    Position::TopRight => DVec2 {
                        x: area.size.x - dot_rect.size.x,
                        y: -dot_rect.size.y + dot_rect.size.y / 2.0,
                    },
                    Position::Left => DVec2 {
                        x: -dot_rect.size.x / 2.0,
                        y: area.size.y / 2.0 - dot_rect.size.y / 2.0,
                    },
                    Position::LeftTop => DVec2 {
                        x: -dot_rect.size.x / 2.0,
                        y: -dot_rect.size.y / 2.0,
                    },
                    Position::LeftBottom => DVec2 {
                        x: -dot_rect.size.x / 2.0,
                        y: -dot_rect.size.y / 2.0 + area.size.y,
                    },
                    Position::Right => DVec2 {
                        x: area.size.x - dot_rect.size.x / 2.0,
                        y: area.size.y / 2.0 - dot_rect.size.y / 2.0,
                    },
                    Position::RightTop => DVec2 {
                        x: area.size.x - dot_rect.size.x / 2.0,
                        y: -dot_rect.size.y / 2.0,
                    },
                    Position::RightBottom => DVec2 {
                        x: area.size.x - dot_rect.size.x / 2.0,
                        y: -dot_rect.size.y / 2.0 + area.size.y,
                    },
                };

                shift.x += self.offset.x as f64;
                shift.y += self.offset.y as f64;

                cx.end_pass_sized_turtle_with_shift(container_area, shift);
                draw_list.end(cx);
            }
        }

        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible && event.requires_visibility() {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().badge = true;

        animation_open_then_redraw!(self, cx, event);

        if self.block_signal_event {
            if let Event::Signal = event {
                return;
            }
        }
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            let mut actions = Vec::new();
            scroll_bars.handle_main_event(cx, event, scope, &mut actions);
            if actions.len() > 0 {
                cx.redraw_area_and_children(self.area);
            };
        }

        // If the UI tree has changed significantly (e.g. AdaptiveView varaints changed),
        // we need to clear the cache and re-query widgets.
        if cx.widget_query_invalidation_event.is_some() {
            self.find_cache.borrow_mut().clear();
        }

        match &self.event_order {
            EventOrder::Up => {
                for (id, child) in self.children.iter_mut().rev() {
                    scope.with_id(*id, |scope| {
                        child.handle_event(cx, event, scope);
                    });
                }
            }
            EventOrder::Down => {
                for (id, child) in self.children.iter_mut() {
                    scope.with_id(*id, |scope| {
                        child.handle_event(cx, event, scope);
                    });
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some((_, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                        scope.with_id(*id, |scope| {
                            child.handle_event(cx, event, scope);
                        });
                    }
                }
            }
        }

        if self.visible || self.animator.live_ptr.is_some() {
            let hit = event.hits_with_capture_overload(cx, self.area(), self.capture_overload);
            if self.disabled {
                self.handle_when_disabled(cx, event, hit);
            } else {
                self.handle_widget_event(cx, event, hit, self.area());
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
}

impl LiveHook for GBadge {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.do_before_apply_pre(cx, apply, index, nodes);
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let update_order_len = self.live_update_order.len();
        if apply.from.is_update_from_doc() {
            //livecoding
            // update/delete children list
            for (idx, id) in self.live_update_order.clone().iter().enumerate() {
                // lets remove this id from the childlist
                if let Some(pos) = self.children.iter().position(|(i, _v)| *i == *id) {
                    // alright so we have the position its in now, and the position it should be in
                    self.children.swap(idx, pos);
                }
            }
            // if we had more truncate
            self.children.truncate(update_order_len);
        }
        if crate::components::needs_draw_list(self.optimize) && self.draw_list.is_none() {
            self.draw_list = Some(DrawList2d::new(cx));
        }
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }

        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [live_id!(basic), live_id!(disabled)],
            [
                (BadgePart::Container, &ViewBasicStyle::live_props()),
                (BadgePart::Dot, &BadgeDotBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(BadgeState::Basic, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(BadgeState::Disabled, applys);
                }
                _ => {}
            },
        );
    }

    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        self.do_apply_value_instance_pre(cx, apply, index, nodes)
    }
}

impl SlotComponent<BadgeState> for GBadge {
    type Part = BadgePart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.dot.style.basic = self.style.basic.dot;
        self.dot.style.disabled = self.style.disabled.dot;
    }
}

impl Component for GBadge {
    type Error = Error;

    type State = BadgeState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.badge;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(BadgeState::Disabled);
        }
        let style = self.style.get(self.state);
        self.draw_badge.merge(&style.container);
        self.dot.render(cx)?;
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, _event: &Event, hit: Hit, area: Area) {
        match hit {
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(area);
                }
                self.active_finger_down(cx, e);
            }
            Hit::FingerMove(e) => {
                self.active_move(cx, e);
            }
            Hit::FingerLongPress(e) => {
                self.active_long_press(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                    } else {
                        self.switch_state_with_animation(cx, BadgeState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.switch_state_with_animation(cx, BadgeState::Basic);
                    hit_finger_up!(self, cx, e);
                }
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.style.get(self.state).container.cursor);
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, BadgeState::Basic);
                hit_hover_out!(self, cx, e);
            }
            Hit::FingerHoverOver(e) => {
                cx.set_cursor(self.style.get(self.state).container.cursor);
                self.active_hover_over(cx, e);
            }
            Hit::KeyDown(e) => {
                self.active_key_down(cx, e);
            }
            Hit::KeyUp(e) => {
                self.switch_state_with_animation(cx, BadgeState::Basic);
                self.active_key_up(cx, e);
            }
            _ => (),
        }
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, BadgeState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
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
        crossed_map.remove(&BadgePart::Dot).map(|map| {
            self.dot.apply_slot_map.merge_slot(map.to_slot());
            self.dot.focus_sync();
        });

        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().badge;

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
            let basic_prop = self.style.get(BadgeState::Basic).container;
            let disabled_prop = self.style.get(BadgeState::Disabled).container;
            let (mut basic_index, mut disabled_index) = (None, None);
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
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_badge = {
                    basic_index => {
                        background_color => basic_prop.background_color,
                        border_color =>basic_prop.border_color,
                        border_radius => basic_prop.border_radius,
                        border_width =>(basic_prop.border_width as f64),
                        shadow_color => basic_prop.shadow_color,
                        spread_radius => (basic_prop.spread_radius as f64),
                        blur_radius => (basic_prop.blur_radius as f64),
                        shadow_offset => basic_prop.shadow_offset,
                        background_visible => basic_prop.background_visible.to_f64()
                    },
                    disabled_index => {
                        background_color => disabled_prop.background_color,
                        border_color => disabled_prop.border_color,
                        border_radius => disabled_prop.border_radius,
                        border_width => (disabled_prop.border_width as f64),
                        shadow_color => disabled_prop.shadow_color,
                        spread_radius => (disabled_prop.spread_radius as f64),
                        blur_radius => (disabled_prop.blur_radius as f64),
                        shadow_offset => disabled_prop.shadow_offset,
                        background_visible => disabled_prop.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state).container;
            let index = match state {
                BadgeState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),

                BadgeState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_badge = {
                    index => {
                        background_color => style.background_color,
                        border_color => style.border_color,
                        border_radius => style.border_radius,
                        border_width => (style.border_width as f64),
                        shadow_color => style.shadow_color,
                        spread_radius => (style.spread_radius as f64),
                        blur_radius => (style.blur_radius as f64),
                        shadow_offset => style.shadow_offset,
                        background_visible => style.background_visible.to_f64()
                    }
                }
            }
        }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.dot.switch_state(state.into());
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl GBadge {
    do_view_livehook_pre!();
    active_event! {
        active_hover_in: BadgeEvent::HoverIn |meta: FingerHoverEvent| => BadgeHoverIn {meta},
        active_hover_over: BadgeEvent::HoverOver |meta: FingerHoverEvent| => BadgeHoverOver {meta},
        active_hover_out: BadgeEvent::HoverOut |meta: FingerHoverEvent| => BadgeHoverOut {meta},
        active_finger_down: BadgeEvent::FingerDown |meta: FingerDownEvent| => BadgeFingerDown {meta},
        active_finger_up: BadgeEvent::FingerUp |meta: FingerUpEvent| => BadgeFingerUp {meta},
        active_long_press: BadgeEvent::LongPress |meta: FingerLongPressEvent| => BadgeLongPress {meta},
        active_move: BadgeEvent::Move |meta: FingerMoveEvent| => BadgeMove {meta},
        active_key_down: BadgeEvent::KeyDown |meta: KeyEvent| => BadgeKeyDown {meta},
        active_key_up: BadgeEvent::KeyUp |meta: KeyEvent| => BadgeKeyUp {meta},
        active_clicked: BadgeEvent::Clicked |meta: FingerUpEvent| => BadgeClicked {meta}
    }
    event_option! {
        hover_in: BadgeEvent::HoverIn => BadgeHoverIn,
        hover_over: BadgeEvent::HoverOver => BadgeHoverOver,
        hover_out: BadgeEvent::HoverOut => BadgeHoverOut,
        finger_down: BadgeEvent::FingerDown => BadgeFingerDown,
        finger_up: BadgeEvent::FingerUp => BadgeFingerUp,
        long_press: BadgeEvent::LongPress => BadgeLongPress,
        finger_move: BadgeEvent::Move => BadgeMove,
        key_down: BadgeEvent::KeyDown => BadgeKeyDown,
        key_up: BadgeEvent::KeyUp => BadgeKeyUp,
        clicked: BadgeEvent::Clicked => BadgeClicked
    }
}
