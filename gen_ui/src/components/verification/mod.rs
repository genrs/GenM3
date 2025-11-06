// mod event;
// mod prop;

// pub use event::*;
// pub use prop::*;

// use makepad_widgets::*;

// use crate::{
//     components::{BasicStyle, Component, GInput, LifeCycle, Style}, error::Error, lifecycle, prop::ApplyStateMap, set_index, set_scope_path, shader::draw_view::DrawView, switch_state, sync, visible
// };

// live_design! {
//     link genui_basic;
//     use link::genui_animation_prop::*;

//     pub VerificationBase = {{GVerification}} {

//     }
// }

// /// # Verification Code
// /// can display multiple boxes for input verification code
// /// ## Display
// /// ```
// ///  ┌───┐ ┌───┐ ┌───┐ ┌───┐
// ///  │ x │ │ x │ │ x │ │   │
// ///  └───┘ └───┘ └───┘ └───┘
// /// ```
// #[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
// pub struct GVerification {
//     #[live]
//     pub style: VerificationStyle,
//     #[live]
//     pub input: Option<LivePtr>,
//     #[rust]
//     pub item: Vec<(LiveId, GInput)>,
//     #[rust]
//     live_update_order: SmallVec<[LiveId; 1]>,
//     #[live]
//     pub draw_verification: DrawView,
//     // --- visible -------------------
//     #[live(true)]
//     pub visible: bool,
//     // --- others -------------------
//     #[live]
//     pub disabled: bool,
//     #[live]
//     pub grab_key_focus: bool,
//     #[live(true)]
//     pub event_open: bool,
//     #[rust]
//     pub scope_path: Option<HeapLiveIdPath>,
//     #[rust]
//     pub apply_state_map: ApplyStateMap<VerificationState>,
//     #[rust]
//     pub index: usize,
//     #[rust(true)]
//     pub sync: bool,
//     #[live(true)]
//     pub animation_open: bool,
//     #[live(true)]
//     pub animation_spread: bool,
//     #[rust]
//     pub lifecycle: LifeCycle,
//     #[rust]
//     pub state: VerificationState,
    
// }

// impl WidgetNode for GVerification {
//     fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
//         WidgetRef::empty()
//     }

//     fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
//         ()
//     }

//     fn walk(&mut self, _cx: &mut Cx) -> Walk {
//         let style = self.style.get(self.state);
//         style.walk()
//     }

//     fn area(&self) -> Area {
//         self.draw_verification.area
//     }

//     fn redraw(&mut self, cx: &mut Cx) {
//         let _ = self.render(cx);
//         self.draw_verification.redraw(cx);
//         for (_, item) in self.item.iter_mut() {
//             if item.visible {
//                 item.redraw(cx);
//             }
//         }
//     }

//     fn state(&self) -> String {
//         self.state.to_string()
//     }

//     fn animation_spread(&self) -> bool {
//         self.animation_spread
//     }

//     visible!();
// }

// impl Widget for GVerification {
//     fn draw_walk(&mut self, _cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {}

//     fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
// }

// impl LiveHook for GVerification {}

// impl Component for GVerification {
//     type Error = Error;

//     type State = VerificationState;

//     fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
//          let style = &cx.global::<Conf>().components.pagination;
//         self.style = style.clone();
//     }

//     fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
//         if self.disabled {
//             self.switch_state(PaginationState::Disabled);
//         }
//         let style = self.style.get(self.state).container;
//         self.draw_verification.merge(&style);
//         Ok(())
//     }


//     fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
//         todo!()
//     }

//     fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
//         ()
//     }

//     fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
//       if !self.animation_open {
//             return;
//         }
//         self.switch_state(state);
//         self.set_animation(cx);
//         self.redraw(cx);
//     }

//     fn focus_sync(&mut self) -> () {
//           self.style.sync(&self.apply_state_map);
//     }

//     fn set_animation(&mut self, cx: &mut Cx) -> () {
//         ()
//     }

//     sync!();
//     set_scope_path!();
//     set_index!();
//     lifecycle!();
//     switch_state!();
// }
