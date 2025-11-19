mod branch;
mod event;
mod leaf;
mod prop;
mod register;

pub use branch::*;
pub use event::*;
pub use leaf::*;
pub use prop::*;
pub use register::register as tree_register;

use crate::{
    ComponentAnInit, active_event, area, area_ref,
    components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, SlotComponent, SlotStyle, Style},
        view::{GView, ViewBasicStyle},
    },
    error::Error,
    event_option, event_option_ref, getter_setter_ref, lifecycle,
    prop::{ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ToStateMap, TreeItemMode, manuel::BASIC},
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible,
};
use makepad_widgets::*;

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GTreeBase = {{GTree}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GTree {
    #[live]
    pub style: TreeStyle,
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
    pub apply_slot_map: ApplySlotMap<TreeState, TreePart>,
    // --- draw ----------------------
    #[live]
    pub draw_menu: DrawView,
    #[live]
    pub body: GView,
    /// active node values
    /// - leaf
    /// - branch: if all leaves under branch are active, branch is active
    #[live]
    pub actives: Vec<String>,
    #[rust]
    pub item_modes: Vec<TreeItemMode>,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub state: TreeState,
    #[live(true)]
    pub animation_spread: bool,
}

impl WidgetNode for GTree {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for slot in [&self.body] {
            for (_, child) in &slot.children {
                let x = child.uid_to_widget(uid);
                if !x.is_empty() {
                    return x;
                }
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for slot in [&self.body] {
            for (_, child) in &slot.children {
                child.find_widgets(path, cached, results);
            }
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_menu.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_menu.redraw(cx);
        if self.body.visible {
            self.body.redraw(cx);
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

impl Widget for GTree {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let state = self.state;
        let style = self.style.get(state);

        let _ = self.draw_menu.begin(
            cx,
            walk,
            Layout {
                clip_x: false,
                clip_y: false,
                padding: style.container.padding,
                align: style.container.align,
                flow: style.container.flow,
                spacing: style.container.spacing,
                ..Default::default()
            },
        );

        if self.body.visible {
            let walk = self.style.get(self.state).body.walk();
            let _ = self.body.draw_walk(cx, scope, walk);
        }

        self.draw_menu.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().menu = true;

        if self.body.visible {
            let actions = cx.capture_actions(|cx| self.body.handle_event(cx, event, scope));
            // 在body的children中如果激活了某个菜单项，需要给出事件
            if !actions.is_empty() {
                let mut active = None;
                self.item_modes.clear();
                for (index, (_id, child)) in self.body.children.iter().enumerate() {
                    let index_chain = vec![index];
                    nested_action(
                        child,
                        cx,
                        &mut self.item_modes,
                        &mut active,
                        &index_chain,
                        &actions,
                    );
                }

                if let Some(e_type) = active {
                    let (meta, value, active) = match e_type {
                        TreeActionType::Branch(BranchChanged {
                            value,
                            meta,
                            active,
                            ..
                        }) => (meta, value, active),
                        TreeActionType::Leaf(LeafClicked {
                            meta,
                            value,
                            active,
                        }) => (meta, value, active),
                    };
                    // 更新actives，如果是branch被激活，则需要将branch下的所有leaf加入actives，如果是leaf被激活，则只加入leaf
                    if active {
                        // 添加value到self.actives
                        if !self.actives.contains(&value) {
                            self.actives.push(value.clone());
                        }
                    } else {
                        // 从self.actives中移除value
                        self.actives.retain(|v| v != &value);
                    }

                    self.active_changed(cx, meta);
                    self.set_target_active(cx);
                    // self.set_actives(cx, self.actives.clone());
                }
            }
        }

        let area = self.area();
        let hit = event.hits(cx, area);
        self.handle_widget_event(cx, event, hit, area);
    }
}

impl LiveHook for GTree {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let live_props = ViewBasicStyle::live_props();
        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [live_id!(basic)],
            [
                (TreePart::Container, &live_props),
                (TreePart::Body, &live_props),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(TreeState::Basic, applys);
                }
                _ => {}
            },
        );
        // if active is some -> set active or find
        if self.actives.is_empty() {
            self.find_actives();
        } else {
            self.set_actives(cx, self.actives.clone());
        }
    }
}

impl Component for GTree {
    type Error = Error;

    type State = TreeState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.tree;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.state;
        let style = self.style.get(state);
        self.draw_menu.merge(&style.container);
        Ok(())
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.body.switch_state(state.into());
    }

    fn switch_state_with_animation(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }

    fn focus_sync(&mut self) -> () {
        let mut crossed_map = self.apply_slot_map.cross();

        crossed_map.remove(&TreePart::Body).map(|map| {
            self.body.apply_state_map.merge(map.to_state());
            self.body.focus_sync();
        });

        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
        ()
    }

    sync!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl SlotComponent<TreeState> for GTree {
    type Part = TreePart;

    fn merge_prop_to_slot(&mut self) -> () {
        self.body.style.basic = self.style.basic.body;
    }
}

impl GTree {
    active_event! {
        active_hover_in: TreeEvent::HoverIn |meta: FingerHoverEvent| => TreeHoverIn { meta },
        active_hover_out: TreeEvent::HoverOut |meta: FingerHoverEvent| => TreeHoverOut { meta }
    }
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    TreeEvent::Changed(TreeChanged {
                        meta,
                        actives: self.actives.clone(),
                    }),
                );
            });
        }
    }
    event_option! {
        hover_in: TreeEvent::HoverIn => TreeHoverIn,
        hover_out: TreeEvent::HoverOut => TreeHoverOut,
        changed: TreeEvent::Changed => TreeChanged
    }
    area! {
        area_body, body
    }
    /// 从body.children中查找激活的菜单项(TreeItem)
    /// 1. 在body的children中只能存在GSubTree和GTreeItem，出现其他类型panic!
    /// 2. 激活的菜单项只能是GTreeItem
    /// 3. 如果没有激活的菜单项，返回None，也无需强制指定，将设置权交给使用者/GRouter
    /// 4. 如果有多个激活的菜单项，只会返回第一个
    /// 5. find时确定self.item_mode结构
    /// 6. 若使用者没有制定node的value，则按照索引进行指定，例如第3个subTree中的第2个TreeItem，则value为"2_1" 2: 3的索引，1: 2的索引
    /// 7. 需要将menu的theme向下同步
    pub fn find_actives(&mut self) {
        fn nested_find(
            child: &WidgetRef,
            item_modes: &mut Vec<TreeItemMode>,
            actives: &mut Vec<String>,
            index_chain: &Vec<usize>,
        ) -> () {
            if let Some(mut child) = child.as_gbranch().borrow_mut() {
                if child.value.is_empty() {
                    child.generate_value(&index_chain);
                }
                let mut sub_menu_mode = vec![];
                if child.active {
                    actives.push(child.value.to_string());
                }
                // 递归查找子菜单项
                for (sub_index, (_id, sub_child)) in child.body.children.iter().enumerate() {
                    let mut index_chain = index_chain.clone();
                    index_chain.push(sub_index);
                    nested_find(sub_child, &mut sub_menu_mode, actives, &index_chain);
                }
                item_modes.push(TreeItemMode::Branch {
                    active: child.active,
                    value: child.value.to_string(),
                    items: sub_menu_mode,
                });
            } else if let Some(mut child) = child.as_gleaf().borrow_mut() {
                if child.value.is_empty() {
                    child.generate_value(&index_chain);
                }
                item_modes.push(TreeItemMode::Leaf {
                    value: child.value.to_string(),
                    active: child.active,
                });
                if child.active {
                    actives.push(child.value.to_string());
                }
            } else {
                panic!("GTree only allows GTreeItem or GSubTree as child!");
            }
        }

        let mut actives = vec![];
        self.item_modes.clear();
        if self.body.visible {
            for (index, (_id, child)) in self.body.children.iter().enumerate() {
                let index_chain = vec![index];
                nested_find(child, &mut self.item_modes, &mut actives, &index_chain);
            }
        }
    }

    /// 设置激活状态
    pub fn set_actives(&mut self, cx: &mut Cx, actives: Vec<String>) {
        self.item_modes.clear();
        // update children
        for (index, (_id, child)) in self.body.children.iter().enumerate() {
            let index_chain = vec![index];
            handle_nested(
                cx,
                child,
                &actives,
                &index_chain,
                &mut self.item_modes,
                false,
            );
        }

        // cover active
        self.actives = actives;
    }
    // 在设置过self.active后，调用此函数，确保self.active的菜单项被激活
    pub fn set_target_active(&mut self, cx: &mut Cx) {
        for (index, (_id, child)) in self.body.children.iter().enumerate() {
            let index_chain = vec![index];
            // nested_set(child, cx, &self.active, &index_chain);
            handle_nested(cx, child, &self.actives, &index_chain, &mut vec![], true);
        }
    }
}

impl GTreeRef {
    event_option_ref! {
        hover_in => TreeHoverIn,
        hover_out => TreeHoverOut,
        changed => TreeChanged
    }
    area_ref! {
        area_body
    }
    getter_setter_ref! {}
}

fn handle_nested(
    cx: &mut Cx,
    child: &WidgetRef,
    actives: &Vec<String>,
    index_chain: &Vec<usize>,
    item_modes: &mut Vec<TreeItemMode>,
    is_action: bool,
) -> bool {
    if let Some(mut child) = child.as_gbranch().borrow_mut() {
        if child.value.is_empty() {
            child.generate_value(&index_chain);
        }
        if !is_action {
            let value = child.value.to_string();
            let _ = child.set_active(cx, actives.contains(&value));
        }
        let mut sub_menu_mode = vec![];
        let mut active_sub = false;
        for (sub_index, (_id, sub_child)) in child.body.children.iter().enumerate() {
            let mut index_chain = index_chain.clone();
            index_chain.push(sub_index);
            let is_active = handle_nested(
                cx,
                sub_child,
                actives,
                &index_chain,
                &mut sub_menu_mode,
                is_action,
            );
            active_sub |= is_active;
        }
        child.active = active_sub;
        if !is_action {
            item_modes.push(TreeItemMode::Branch {
                active: child.active,
                value: child.value.to_string(),
                items: sub_menu_mode,
            });
        }
        return child.active;
    } else if let Some(mut child) = child.as_gleaf().borrow_mut() {
        if child.value.is_empty() {
            child.generate_value(&index_chain);
        }
        let value = child.value.to_string();
        let _ = child.set_active(cx, actives.contains(&value));

        if !is_action {
            item_modes.push(TreeItemMode::Leaf {
                value: child.value.to_string(),
                active: child.active,
            });
        }
        return child.active;
    } else {
        panic!("GTree only allows GTreeItem or GSubTree as child!");
    }
}

fn nested_action(
    child: &WidgetRef,
    cx: &mut Cx,
    item_modes: &mut Vec<TreeItemMode>,
    active: &mut Option<TreeActionType>,
    index_chain: &Vec<usize>,
    actions: &Vec<Box<dyn ActionTrait>>,
) {
    if let Some(mut child) = child.as_gbranch().borrow_mut() {
        if child.value.is_empty() {
            child.generate_value(&index_chain);
        }

        if let Some(e) = child.changed(actions) {
            // 替换当前的active
            active.replace(TreeActionType::Branch(e));
        }
        // 无论是否发生changed事件，如果father_active为true，那么branch下的所有leaf都要激活
        let mut sub_menu_mode = vec![];
        // 递归查找子菜单项
        for (sub_index, (_id, sub_child)) in child.body.children.iter().enumerate() {
            let mut index_chain = index_chain.clone();
            index_chain.push(sub_index);
            nested_action(
                sub_child,
                cx,
                &mut sub_menu_mode,
                active,
                &index_chain,
                actions,
            );
        }

        item_modes.push(TreeItemMode::Branch {
            active: child.active,
            value: child.value.to_string(),
            items: sub_menu_mode,
        });
    } else if let Some(mut child) = child.as_gleaf().borrow_mut() {
        if child.value.is_empty() {
            child.generate_value(&index_chain);
        }
        item_modes.push(TreeItemMode::Leaf {
            value: child.value.to_string(),
            active: child.active,
        });
        if active.is_none() {
            if let Some(e) = child.clicked(actions) {
                active.replace(TreeActionType::Leaf(e));
            }
        }
    } else {
        panic!("GTree only allows GTreeItem or GSubTree as child!");
    }
}
