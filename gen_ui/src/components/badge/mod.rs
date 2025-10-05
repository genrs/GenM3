pub mod dot;
mod prop;
mod register;

pub use register::register as badge_register;
use std::cell::RefCell;

use crate::{
    animation_open_then_redraw, components::{BasicStyle, Component, DrawState, GContainer, LifeCycle, Style}, error::Error, event_option, event_option_ref, impl_view_trait_live_hook, impl_view_trait_widget_node, inherits_container_find_widgets, lifecycle, play_animation, prop::{
        manuel::{BASIC, DISABLED}, traits::ToFloat, ApplySlotMap, ApplyStateMap, DeferWalks
    }, pure_after_apply, set_animation, set_index, set_scope_path, shader::draw_view::DrawView, sync, themes::conf::Conf, ComponentAnInit
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
}

impl WidgetNode for GBadge {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        todo!()
    }

    inherits_container_find_widgets!();

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        todo!()
    }

    fn area(&self) -> Area {
        todo!()
    }

    fn redraw(&mut self, _cx: &mut Cx) {
        todo!()
    }
}

impl Widget for GBadge {}

impl LiveHook for GBadge {}

impl Component for GBadge {
    type Error = Error;

    type State = BadgeState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_scope_path(&mut self, path: &HeapLiveIdPath) -> () {
        todo!()
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
        todo!()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        todo!()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        todo!()
    }

    fn sync(&mut self) -> () {
        todo!()
    }

    fn focus_sync(&mut self) -> () {
        todo!()
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn lifecycle(&self) -> LifeCycle {
        todo!()
    }

    fn set_index(&mut self, index: usize) -> () {
        todo!()
    }
}
