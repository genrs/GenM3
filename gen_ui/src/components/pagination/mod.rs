mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{
        BasicStyle, ButtonState, Component, GButton, GLabelWidgetRefExt, LifeCycle, Style,
    }, error::Error, event_option, lifecycle, prop::{
        ApplyStateMap,
        manuel::{BASIC, DISABLED},
        traits::ToFloat,
    }, set_index, set_scope_path, shader::draw_view::DrawView, switch_state, sync, themes::conf::Conf, visible
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
    #[live(false)]
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
    #[rust]
    pub display_pages: Vec<String>,
    #[live(true)]
    pub event_open: bool,
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
        for ((_id, btn), text) in self.item.iter_mut().zip(self.display_pages.iter()) {
            let walk = btn.walk(cx);
            btn.set_text(cx, &text);
            // 如果current等于按钮的页码，则设置为选中状态
            if self.current.to_string().eq(text) {
                btn.switch_state_with_animation(cx, ButtonState::Pressed);
            } else {
                btn.switch_state_with_animation(cx, ButtonState::Basic);
            }
            let _ = btn.draw_walk(cx, scope, walk);
        }

        if self.suffix.visible {
            let walk = self.suffix.walk(cx);
            let _ = self.suffix.draw_walk(cx, scope, walk);
        }
        self.draw_pagination.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }
        // if self.disabled {
        //     let area = self.area();
        //     let hit = event.hits(cx, area);
        //     self.handle_when_disabled(cx, event, hit);
        //     return;
        // }
        // 事件捕捉 ----------------------------------------------------------------------------------------------------

        self.match_event(cx, event);
        // 点击前缀按钮会让current - 1， 点击后缀按钮会让current + 1, 如果 current - 1 或 + 1 超过范围则直接设置为边界值
        self.prefix.handle_event(cx, event, scope);
        self.suffix.handle_event(cx, event, scope);
        // 点击页码按钮会让 current 变为对应的页码，页码中的省略号按钮，前省略号会让 current - 5，后省略号会让 current + 5，超过范围则设置为边界值
        for (_id, item) in self.item.iter_mut() {
            // 如果current等于按钮的页码，则跳过
            if self.current.to_string() == item.slot.as_glabel().get_text() {
                continue;
            }
            item.handle_event(cx, event, scope);
        }
    }
}

impl MatchEvent for GPagination {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut meta = None;
        if let Some(e) = self.prefix.clicked(actions) {
            if self.current > 1 {
                self.current -= 1;
                meta = Some(e.meta);
            }
        }

        if let Some(e) = self.suffix.clicked(actions) {
            if self.current < self.total {
                self.current += 1;
                meta = Some(e.meta);
            }
        }

        for (id, item) in self.item.iter_mut() {
            if let Some(e) = item.clicked(actions) {
                let text = item.slot.as_glabel().get_text();
                if *id == live_id!(prefix_ellipsis) {
                    // 前省略号
                    if self.current > 5 {
                        self.current -= 5;
                    } else {
                        self.current = 1;
                    }
                } else if *id == live_id!(suffix_ellipsis) {
                    // 后省略号
                    if self.current + 5 < self.total {
                        self.current += 5;
                    } else {
                        self.current = self.total;
                    }
                } else {
                    // 普通页码按钮
                    if let Ok(page) = text.parse::<usize>() {
                        self.current = page;
                    }
                }
                meta = Some(e.meta);
            }
        }

        if let Some(meta) = meta {
            self.active_changed(cx, Some(meta));
        }
    }
}

impl LiveHook for GPagination {
    // pure_after_apply!();
    #[allow(unused_variables)]
    #[cfg(feature = "dev")]
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        self.sync();
        self.render_after_apply(cx);
    }

    #[cfg(feature = "dev")]
    fn after_update_from_doc(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
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

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            apply.from,
            nodes,
            index,
            &PaginationBasicStyle::live_props(),
            [live_id!(basic), live_id!(disabled)],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_state_map
                        .insert(PaginationState::Basic, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(PaginationState::Disabled, applys);
                }
                _ => {}
            },
        );

        self.item.clear();
        // 根据 current 和 page_size 计算出当前页的按钮范围
        let (start_page, end_page, show_prefix_ellipsis, show_suffix_ellipsis) =
            self.count_display_pages();
        // 确定 item 数量
        let display_count = end_page - start_page
            + 1
            + (show_prefix_ellipsis.to_f32() as usize)
            + (show_suffix_ellipsis.to_f32() as usize);
        for i in 0..display_count {
            // 第一个按钮永远是1这个按钮
            if i == 0 {
                self.item
                    .push((live_id!(page_1), GButton::new_from_ptr(cx, self.btn)));
                self.display_pages.push("1".to_string());
                continue;
            } else if i == display_count - 1 {
                // 最后一个按钮永远是total这个按钮
                self.item
                    .push((live_id!(page_total), GButton::new_from_ptr(cx, self.btn)));
                self.display_pages.push(self.total.to_string());
                continue;
            }
            // 显示前缀省略号
            if show_prefix_ellipsis && i == 1 {
                self.item.push((
                    live_id!(prefix_ellipsis),
                    GButton::new_from_ptr(cx, self.btn),
                ));
                self.display_pages.push("...".to_string());
                continue;
            }
            if show_suffix_ellipsis && i == display_count - 2 {
                // 显示后缀省略号
                self.item.push((
                    live_id!(suffix_ellipsis),
                    GButton::new_from_ptr(cx, self.btn),
                ));
                self.display_pages.push("...".to_string());
                continue;
            }

            // 中间的页码按钮
            let page_number = start_page + i - (if show_prefix_ellipsis { 1 } else { 0 });
            self.item
                .push((live_id!(page_number), GButton::new_from_ptr(cx, self.btn)));
            self.display_pages.push(page_number.to_string());
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

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
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

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn switch_state_with_animation(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }
    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
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
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    PaginationEvent::Changed(PaginationChanged {
                        meta,
                        current: self.current,
                        page_size: self.page_size as usize,
                    }),
                );
            });
        }
    }
    event_option! {
        changed: PaginationEvent::Changed => PaginationChanged
    }
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
