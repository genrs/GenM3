pub mod dot;
mod prop;
mod register;

pub use register::register as badge_register;
use std::cell::RefCell;

use crate::{
    components::{
        BasicStyle, Component, DrawState, LifeCycle, SlotComponent, SlotStyle, Style,
        ViewBasicStyle, ViewTextureCache,
        dot::{BadgeDotBasicStyle, GBadgeDot},
        is_texture, needs_draw_list,
    },
    do_view_livehook_pre,
    error::Error,
    inherits_view_find_widgets, lifecycle, play_animation,
    prop::{
        ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, Position, ToSlotMap,
        manuel::{BASIC, DISABLED},
    },
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible,
};

pub use prop::*;

use makepad_widgets::*;

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
                        y: area.size.y + 0.0,
                    },
                    Position::BottomLeft => DVec2 {
                        x: 0.0,
                        y: area.size.y + 0.0,
                    },
                    Position::BottomRight => DVec2 {
                        x: area.size.x - dot_rect.size.x,
                        y: area.size.y + 0.0,
                    },
                    Position::Top => DVec2 {
                        x: -dot_rect.size.x / 2.0 + area.size.x / 2.0,
                        y: -dot_rect.size.y,
                    },
                    Position::TopLeft => DVec2 {
                        x: 0.0,
                        y: -dot_rect.size.y,
                    },
                    Position::TopRight => DVec2 {
                        x: area.size.x - dot_rect.size.x,
                        y: -dot_rect.size.y,
                    },
                    Position::Left => DVec2 {
                        x: -dot_rect.size.x,
                        y: area.size.y / 2.0 - dot_rect.size.y / 2.0,
                    },
                    Position::LeftTop => DVec2 {
                        x: -dot_rect.size.x,
                        y: 0.0,
                    },
                    Position::LeftBottom => DVec2 {
                        x: -dot_rect.size.x,
                        y: 0.0 - dot_rect.size.y + area.size.y,
                    },
                    Position::Right => DVec2 {
                        x: area.size.x + 0.0,
                        y: area.size.y / 2.0 - dot_rect.size.y / 2.0,
                    },
                    Position::RightTop => DVec2 {
                        x: area.size.x + 0.0,
                        y: 0.0,
                    },
                    Position::RightBottom => DVec2 {
                        x: area.size.x + 0.0,
                        y: 0.0 - dot_rect.size.y + area.size.y,
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

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        ()
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
        ()
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
}
