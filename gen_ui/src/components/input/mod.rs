pub mod area;
mod event;
mod prop;
mod register;
mod rely;

pub use event::*;
pub use prop::*;
pub use register::register as input_register;
use rely::*;
use makepad_widgets::*;
use crate::{components::{area::GInputArea, Component, GView, LifeCycle, SlotComponent}, error::Error, prop::ApplySlotMap, shader::draw_view::DrawView};

live_design! {
    link genui_basic;
    use link::theme::*;
    use link::genui_animation_prop::*;

    pub GInputBase = {{GInput}} {}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GInput {
    #[live]
    pub draw_input: DrawView,
    #[live]
    pub input: GInputArea,
    #[live]
    pub suffix: GView,
    #[live]
    pub prefix: GView,
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
    pub state: InputState,
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
    pub apply_slot_map: ApplySlotMap<InputState, InputPart>,
}

impl WidgetNode for GInput {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        todo!()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        todo!()
    }

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

impl Widget for GInput {}

impl LiveHook for GInput {
    
}

impl SlotComponent<InputState> for GInput {
    type Part = InputPart;

    fn merge_prop_to_slot(&mut self) -> () {
        todo!()
    }
}

impl Component for GInput {
    type Error = Error;

    type State = InputState;

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