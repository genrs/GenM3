mod prop;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    components::{
        PopupContainerBasicStyle,
        item::GSelectItem,
        lifecycle::LifeCycle,
        popup::PopupState,
        traits::{BasicStyle, PopupComponent, Style},
    },
    error::Error,
    lifecycle,
    prop::{ApplyStateMap, Position, manuel::BASIC},
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    themes::conf::Conf,
};

live_design! {
    link genui_basic;

    pub GSelectOptionsBase = {{GSelectOptions}}{}
}

/// ## This is a container for the popup component.
#[derive(Live, LiveRegister)]
pub struct GSelectOptions {
    #[live]
    pub style: SelectOptionsStyle,
    #[rust]
    pub children: Vec<(LiveId, GSelectItem)>,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    #[live]
    pub draw_options: DrawView,
    #[live]
    draw_list: DrawList2d,
    #[live(true)]
    pub visible: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<PopupState>,
    #[rust]
    pub index: usize,
    #[rust(true)]
    pub sync: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub state: PopupState,
}

impl LiveHook for GSelectOptions {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &PopupContainerBasicStyle::live_props(),
            [live_id!(basic)],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(PopupState::Basic, applys);
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
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::Animate | ApplyFrom::Over => {
                let node_id = nodes[index].id;
                if let Some((_, component)) =
                    self.children.iter_mut().find(|(id, _)| *id == node_id)
                {
                    component.apply(cx, apply, index, nodes)
                } else {
                    nodes.skip_node(index)
                }
            }
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if nodes[index].is_instance_prop() {
                    if apply.from.is_update_from_doc() {
                        //livecoding
                        self.live_update_order.push(id);
                    }
                    //self.draw_order.push(id);
                    if let Some((_, node)) = self.children.iter_mut().find(|(id2, _)| *id2 == id) {
                        node.apply(cx, apply, index, nodes)
                    } else {
                        self.children.push((id, GSelectItem::new(cx)));
                        self.children
                            .last_mut()
                            .unwrap()
                            .1
                            .apply(cx, apply, index, nodes)
                    }
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                    nodes.skip_node(index)
                }
            }
            _ => nodes.skip_node(index),
        }
    }
}

impl PopupComponent for GSelectOptions {
    type Error = Error;

    type State = PopupState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.select_options;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let style = self.style.get(self.current_state());
        self.draw_options.merge(&(*style).into());
        Ok(())
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        self.focus_sync();
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn current_state(&self) -> Self::State {
        self.state
    }

    fn walk(&self) -> Walk {
        self.style.get(self.current_state()).walk()
    }

    fn begin(&mut self, cx: &mut Cx2d, walk: Walk) -> () {
        self.draw_list.begin_overlay_reuse(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());
        let style = self.style.get(self.current_state());
        self.draw_options.begin(cx, walk, style.layout());
    }

    fn end(&mut self, cx: &mut Cx2d, _scope: &mut Scope, shift_area: Area, shift: DVec2) -> () {
        self.draw_options.end(cx);
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
    }

    fn redraw(&mut self, cx: &mut Cx) -> () {
        self.draw_list.redraw(cx);
        for (_id, child) in self.children.iter_mut() {
            let _ = child.redraw(cx);
        }
    }

    fn draw_popup(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        _position: Option<Position>,
        _angle_offset: f32,
        _redraw: &mut bool,
    ) -> () {
        for (_id, child) in self.children.iter_mut() {
            let walk = child.walk(cx);
            let _ = child.draw_walk(cx, scope, walk);
        }
    }

    set_index!();
    lifecycle!();
    set_scope_path!();
}

impl GSelectOptions {
    pub fn area(&self) -> Area {
        // self.popup.area
        self.draw_options.area
    }

    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        for (_id, child) in self.children.iter_mut() {
            let _ = child.handle_event_with(cx, event, scope, sweep_area);
        }
    }

    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.draw_options.area().clipped_rect(cx).contains(pos)
    }
}
