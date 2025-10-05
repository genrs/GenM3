use std::cell::RefCell;

use makepad_widgets::*;

use crate::{
    components::{DrawState, LifeCycle, ViewTextureCache, needs_draw_list},
    shader::draw_view::DrawView,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GContainerBase = {{GContainer}} {}
}

/// # Container
/// container is a lowest component which can use to expand each view component, such as: `View`, `Badge`
/// - it can hold children widgets
/// - it has no specific `style`, `state`, etc.
/// - it use for inherits
#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
#[allow(dead_code)]
pub struct GContainer {
    // --- other props ------------
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
    find_cache: RefCell<SmallVec<[(u64, WidgetSet); 3]>>,
    #[rust]
    scroll_bars_obj: Option<Box<ScrollBars>>,
    #[rust]
    pub view_size: Option<DVec2>,
    #[rust]
    pub area: Area,
    #[rust]
    draw_list: Option<DrawList2d>,
    #[rust]
    texture_cache: Option<ViewTextureCache>,
    #[rust]
    defer_walks: SmallVec<[(LiveId, DeferWalk); 1]>,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    pub children: SmallVec<[(LiveId, WidgetRef); 2]>,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    // --- animation --------------
    #[animator]
    animator: Animator,
    #[live(false)]
    pub animation_open: bool,
    #[live(true)]
    pub animation_spread: bool,
    // --- draw -------------------
    #[live]
    pub draw_container: DrawView,
    // --- lifecycle --------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub index: usize,
    #[live(true)]
    pub sync: bool,
}

impl LiveHook for GContainer {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            //self.draw_order.clear();
            self.live_update_order.clear();
            self.find_cache.get_mut().clear();
        }
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if apply.from.is_update_from_doc() {
            //livecoding
            // update/delete children list
            for (idx, id) in self.live_update_order.iter().enumerate() {
                // lets remove this id from the childlist
                if let Some(pos) = self.children.iter().position(|(i, _v)| *i == *id) {
                    // alright so we have the position its in now, and the position it should be in
                    self.children.swap(idx, pos);
                }
            }
            // if we had more truncate
            self.children.truncate(self.live_update_order.len());
        }
        if needs_draw_list(self.optimize) && self.draw_list.is_none() {
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
                        self.children.push((id, WidgetRef::new(cx)));
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

impl WidgetNode for GContainer {
    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        Walk::default()
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
        self.area.redraw(cx);
        self.draw_container.redraw(cx);
        for (_, child) in &mut self.children {
            child.redraw(cx);
        }
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        match cached {
            WidgetCache::Yes | WidgetCache::Clear => {
                if let WidgetCache::Clear = cached {
                    self.find_cache.borrow_mut().clear();
                    if path.len() == 0 {
                        return;
                    }
                }
                let mut hash = 0u64;
                for i in 0..path.len() {
                    hash ^= path[i].0
                }
                if let Some((_, widget_set)) =
                    self.find_cache.borrow().iter().find(|(h, _v)| h == &hash)
                {
                    results.extend_from_set(widget_set);
                    return;
                }
                let mut local_results = WidgetSet::empty();
                if let Some((_, child)) = self.children.iter().find(|(id, _)| *id == path[0]) {
                    if path.len() > 1 {
                        child.find_widgets(&path[1..], WidgetCache::No, &mut local_results);
                    } else {
                        local_results.push(child.clone());
                    }
                }
                for (_, child) in &self.children {
                    child.find_widgets(path, WidgetCache::No, &mut local_results);
                }
                if !local_results.is_empty() {
                    results.extend_from_set(&local_results);
                }
                self.find_cache.borrow_mut().push((hash, local_results));
            }
            WidgetCache::No => {
                if let Some((_, child)) = self.children.iter().find(|(id, _)| *id == path[0]) {
                    if path.len() > 1 {
                        child.find_widgets(&path[1..], WidgetCache::No, results);
                    } else {
                        results.push(child.clone());
                    }
                }
                for (_, child) in &self.children {
                    child.find_widgets(path, WidgetCache::No, results);
                }
            }
        }
    }
}

impl Widget for GContainer {}

impl GContainer {
    pub fn walk_from_previous_size(&self, walk: Walk) -> Walk {
        let view_size = self.view_size.unwrap_or(DVec2::default());
        Walk {
            abs_pos: walk.abs_pos,
            width: if walk.width.is_fill() {
                walk.width
            } else {
                Size::Fixed(view_size.x)
            },
            height: if walk.height.is_fill() {
                walk.height
            } else {
                Size::Fixed(view_size.y)
            },
            margin: walk.margin,
        }
    }
}
