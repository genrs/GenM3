pub mod dot;
mod prop;
mod register;

pub use register::register as badge_register;
use std::cell::RefCell;

use crate::{
    animation_open_then_redraw, components::{
        dot::GBadgeDot, BasicStyle, Component, DrawState, GContainer, LifeCycle, SlotComponent, SlotStyle, Style
    }, do_container_livehook_pre, error::Error, event_option, event_option_ref, impl_view_trait_live_hook, impl_view_trait_widget_node, inherits_container_find_widgets, lifecycle, play_animation, prop::{
        manuel::{BASIC, DISABLED}, traits::ToFloat, ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, ApplyStateMap, DeferWalks, ToSlotMap
    }, pure_after_apply, set_animation, set_index, set_scope_path, shader::draw_view::DrawView, switch_state, sync, themes::conf::Conf, visible, ComponentAnInit
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

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
#[allow(dead_code)]
pub struct GBadge {
    #[deref]
    pub super_widget: GContainer,
    #[live]
    pub style: BadgeStyle,
    #[rust]
    pub state: BadgeState,
    #[rust]
    pub apply_slot_map: ApplySlotMap<BadgeState, BadgePart>,
    #[live]
    pub dot: GBadgeDot,
}

impl WidgetNode for GBadge {
    inherits_container_find_widgets!();

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
        self.draw_view.redraw(cx);
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

impl Widget for GBadge {}

impl LiveHook for GBadge {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.do_before_apply_pre(cx, apply, index, nodes);
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
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

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(BadgeState::Disabled);
        }
        let style = self.style.get(self.state).container;
        self.draw_view.merge(&style);
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
    do_container_livehook_pre!();
}
