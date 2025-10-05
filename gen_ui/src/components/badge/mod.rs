mod prop;
pub mod dot;
mod register;


pub use register::register as badge_register;
use std::cell::RefCell;

use crate::{
    ComponentAnInit, animation_open_then_redraw,
    components::{BasicStyle, Component, DrawState, LifeCycle, Style},
    error::Error,
    event_option, event_option_ref, impl_view_trait_live_hook, impl_view_trait_widget_node,
    lifecycle, play_animation,
    prop::{
        ApplyStateMap, DeferWalks,
        manuel::{BASIC, DISABLED},
        traits::ToFloat,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
};

pub use prop::*;


use makepad_widgets::*;

// live_design! {
//     link genui_basic;
//     use link::genui_animation_prop::*;

//     pub GBadgeBase = {{GBadge}} {
//         animator: {
//             hover = {
//                 default: off,

//                 off = {
//                     from: {all: Forward {duration: (AN_DURATION)}},
//                     ease: InOutQuad,
//                     apply: {
//                         draw_badge: <AN_DRAW_VIEW> {}
//                     }
//                 }

//                 disabled = {
//                     from: {all: Forward {duration: (AN_DURATION)}},
//                     ease: InOutQuad,
//                     apply: {
//                         draw_badge: <AN_DRAW_VIEW> {}
//                     }
//                 }
//             }
//         }
//     }
// }

// #[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
// pub struct GBadge {
//     #[live]

// }