mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{BasicStyle, Component, GButton, LifeCycle, Style},
    error::Error,
    lifecycle, play_animation,
    prop::{ApplyStateMap, traits::ToFloat},
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    switch_state, sync,
    themes::conf::Conf,
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GPaginationBase = {{GPagination}} {}
}

/// # Pagination
///
/// ## Display
/// ```md
/// -------------------------------
/// | < | item | item | item | > |  extra
/// -------------------------------
/// - < : prefix button
/// - > : suffix button
/// - item : page button
/// - extra : can be used to show total pages info // TODO
/// ```
#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GPagination {
    #[live]
    pub style: PaginationStyle,
    #[live]
    pub prefix: GButton,
    #[live]
    pub suffix: GButton,
    /// pagination btn template
    #[live]
    pub btn: Option<LivePtr>,
    #[rust]
    pub item: Vec<(LiveId, GButton)>,
    // #[live] TODO
    // pub extra: GView,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    #[live]
    pub draw_pagination: DrawView,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub disabled: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<PaginationState>,
    #[rust]
    pub index: usize,
    #[rust(true)]
    pub sync: bool,
    #[live(true)]
    pub animation_spread: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub state: PaginationState,
    // --- pagination
    #[live]
    pub total: usize,
    #[live]
    pub current: usize,
    #[live(5)]
    pub page_size: i32,
}

impl WidgetNode for GPagination {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_pagination.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_pagination.redraw(cx);
        for button in [&mut self.prefix, &mut self.suffix] {
            if button.visible {
                button.redraw(cx);
            }
        }
        for (_, item) in self.item.iter_mut() {
            if item.visible {
                item.redraw(cx);
            }
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

impl Widget for GPagination {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_pagination.begin(cx, walk, style.layout());
        if self.prefix.visible {
            let walk = self.prefix.walk(cx);
            let _ = self.prefix.draw_walk(cx, scope, walk);
        }
        // for (_id, item) in self.item.iter_mut() {
        //     if item.visible {
        //         let walk = item.walk(cx);
        //         let _ = item.draw_walk(cx, scope, walk);
        //     }
        // }

        // 根据 current 和 page_size 计算出当前页的按钮范围
        let (start_page, end_page, show_prefix_ellipsis, show_suffix_ellipsis) =
            self.count_display_pages();
        // 确定 item 数量
        let display_count = end_page - start_page
            + 1
            + (show_prefix_ellipsis.to_f32() as usize)
            + (show_suffix_ellipsis.to_f32() as usize);
        for i in 0..display_count {
            let mut display = None;
            // 第一个按钮永远是1这个按钮
            if i == 0 {
                self.item
                    .push((live_id!(page_1), GButton::new_from_ptr(cx, self.btn)));
                display = Some("1".to_string());
            } else if i == display_count - 1 {
                // 最后一个按钮永远是total这个按钮
                self.item
                    .push((live_id!(page_total), GButton::new_from_ptr(cx, self.btn)));
                display = Some(self.total.to_string());
            }
            // 显示前缀省略号
            if show_prefix_ellipsis && display.is_none() && i == 1 {
                self.item.push((
                    live_id!(prefix_ellipsis),
                    GButton::new_from_ptr(cx, self.btn),
                ));
                display = Some("...".to_string());
            }
            if show_suffix_ellipsis && display.is_none() && i == display_count - 2 {
                // 显示后缀省略号
                self.item.push((
                    live_id!(suffix_ellipsis),
                    GButton::new_from_ptr(cx, self.btn),
                ));
                display = Some("...".to_string());
            }

            // 中间的页码按钮
            if display.is_none() {
                let page_number = start_page + i - (if show_prefix_ellipsis { 1 } else { 0 });
                self.item
                    .push((live_id!(page_number), GButton::new_from_ptr(cx, self.btn)));
                display = Some(page_number.to_string());
            }
            // 对按钮进行绘制
            if let Some(text) = display {
                let btn = &mut self.item.last_mut().unwrap().1;
                let walk = btn.walk(cx);
                btn.set_text(cx, &text);
                let _ = btn.draw_walk(cx, scope, walk);
                continue;
            }
        }

        if self.suffix.visible {
            let walk = self.suffix.walk(cx);
            let _ = self.suffix.draw_walk(cx, scope, walk);
        }
        self.draw_pagination.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl LiveHook for GPagination {
    pure_after_apply!();

    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if nodes[index].is_instance_prop() {
                    if apply.from.is_update_from_doc() {
                        self.live_update_order.push(id);
                    }

                    if let Some((_, node)) = self.item.iter_mut().find(|(id2, _)| *id2 == id) {
                        node.apply(cx, apply, index, nodes)
                    } else {
                        self.item.push((id, GButton::new(cx)));
                        self.item
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

impl Component for GPagination {
    type Error = Error;

    type State = PaginationState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.pagination;
        self.style = style.clone();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(PaginationState::Disabled);
        }
        let style = self.style.get(self.state).container;
        self.draw_pagination.merge(&style);

        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, PaginationState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        ()
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }
    fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
        ()
    }

    sync!();
    // play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
    switch_state!();
}

impl GPagination {
    /// count display pages by current page and page size
    /// return (start_page, end_page, show_prefix_ellipsis, show_suffix_ellipsis)
    /// - total <= 5: (1, total, false, false) `1 2 3 4 5`
    /// - total > 5:
    ///    - current < 5: (1, 6, false, true) `1 2 3 4 5 6 ... total`
    ///    - current >= 5: (current - 2, current + 2, true, true) `1 ... 3 4 5 6 7 ... total`
    ///    - current > 5: (total - 5, total, true, false) `1 ... total-5 total-4 total-3 total-2 total-1 total`
    fn count_display_pages(&self) -> (usize, usize, bool, bool) {
        let total = self.total;
        let current = self.current;
        if total <= 5 {
            (1, total, false, false)
        } else {
            if current < 5 {
                (1, 6, false, true)
            } else if current >= 5 && current <= total - 4 {
                (current - 2, current + 2, true, true)
            } else {
                (total - 5, total, true, false)
            }
        }
    }
}
