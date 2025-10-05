#[macro_export]
macro_rules! inherits_view_widget_node {
    ($component: ty) => {
        impl WidgetNode for $component {
            fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
                self.deref_widget.uid_to_widget(uid)
            }

            fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
                self.deref_widget.find_widgets(path, cached, results);
            }

            fn walk(&mut self, cx: &mut Cx) -> Walk {
                self.deref_widget.walk(cx)
            }

            fn area(&self) -> Area {
                self.deref_widget.area()
            }

            fn redraw(&mut self, cx: &mut Cx) {
                self.deref_widget.redraw(cx);
            }
            crate::visible!();
        }
    };
}

#[macro_export]
macro_rules! inherits_view_livehook {
    () => {
        fn after_new_before_apply(&mut self, cx: &mut Cx) {
            self.deref_widget.after_new_before_apply(cx);
        }
        fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
            self.deref_widget.before_apply(cx, apply, index, nodes);
        }
        fn after_update_from_doc(&mut self, cx: &mut Cx) {
            self.deref_widget.after_update_from_doc(cx);
        }
        fn after_apply_from_doc(&mut self, cx: &mut Cx) {
            self.deref_widget.after_apply_from_doc(cx);
        }
        fn after_new_from_doc(&mut self, cx: &mut Cx) {
            self.deref_widget.after_new_from_doc(cx);
        }
        fn apply_value_instance(
            &mut self,
            cx: &mut Cx,
            apply: &mut Apply,
            index: usize,
            nodes: &[LiveNode],
        ) -> usize {
            self.deref_widget
                .apply_value_instance(cx, apply, index, nodes)
        }
    };
}
/// ## do container livehook trait fn pre-operations
/// pre-operations for some methods in the livehook trait
/// - `after_apply`
/// - `before_apply`
/// - `apply_value_instance`
/// **should use in component which need to inherits container**
/// ### usage
/// ```
/// impl XXX {
///     do_container_livehook_pre!();
/// }
/// ```
#[macro_export]
macro_rules! do_container_livehook_pre {
    ($widget_ty: ty) => {
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
        #[allow(unused_variables)]
        fn do_after_apply_pre(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
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

        fn do_before_apply_pre(
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

        fn do_apply_value_instance_pre(
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
    };
}

#[macro_export]
macro_rules! inherits_container_find_widgets {
    () => {
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
    };
}