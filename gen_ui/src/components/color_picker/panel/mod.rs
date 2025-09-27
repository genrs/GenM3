mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{BasicStyle, Component, LifeCycle, Style},
    error::Error,
    lifecycle,
    prop::{
        ApplyStateMap,
        manuel::{BASIC, DISABLED, HOVER, PRESSED},
        traits::ToColor,
    },
    pure_after_apply, set_index, set_scope_path,
    shader::draw_color_picker::DrawColorPanel,
    switch_state, sync,
    themes::conf::Conf,
    visible,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;
    use link::shaders::*;

    pub GColorPanelBase = {{GColorPanel}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GColorPanel {
    #[live]
    pub style: ColorPanelStyle,
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live]
    pub disabled: bool,
    #[live]
    pub draw_color_panel: DrawColorPanel,
    #[animator]
    animator: Animator,
    #[rust]
    pub size: f64,
    #[rust]
    pub picker_x: f32,
    #[rust]
    pub picker_y: f32,
    #[live]
    pub color: Vec4,
    #[rust]
    pub base_color: Vec4,
    #[rust(ColorPickerDragMode::None)]
    drag_mode: ColorPickerDragMode,
    #[live(true)]
    pub animation_spread: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<ColorPanelState>,
    #[live(true)]
    pub event_open: bool,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub state: ColorPanelState,
}

impl WidgetNode for GColorPanel {
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
        self.draw_color_panel.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_color_panel.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GColorPanel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible() {
            return DrawStep::done();
        }

        // 更新绘制参数
        self.size = cx.turtle().rect().size.y.min(cx.turtle().rect().size.x);
        let style = self.style.get(self.state);
        self.draw_color_panel.begin(cx, walk, style.layout());
        self.draw_color_panel.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl LiveHook for GColorPanel {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &ColorPanelBasicStyle::live_props(),
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(pressed),
                live_id!(disabled),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_state_map
                        .insert(ColorPanelState::Basic, applys);
                }
                HOVER => {
                    component
                        .apply_state_map
                        .insert(ColorPanelState::Hover, applys);
                }
                PRESSED => {
                    component
                        .apply_state_map
                        .insert(ColorPanelState::Pressed, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(ColorPanelState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GColorPanel {
    type Error = Error;

    type State = ColorPanelState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.color_panel;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(ColorPanelState::Disabled);
        }
        self.fix_picker_pos();
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, _area: Area) {
        self.animator_handle_event(cx, event);

        match hit {
            Hit::FingerDown(fe) => {
                let rel = fe.abs - fe.rect.pos;
                self.drag_mode = ColorPickerDragMode::Rect;
                self.set_picker_pos(cx, rel);
            }
            Hit::FingerUp(fe) => {
                self.drag_mode = ColorPickerDragMode::None;
                self.active_changed(cx, Some(fe));
            }
            Hit::FingerMove(fe) => {
                let rel = fe.abs - fe.rect.pos;
                self.set_picker_pos(cx, rel);
            }
            _ => (),
        }
    }

    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
        ()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        self.switch_state(state);
        self.set_animation(cx);
        self.redraw(cx);
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    sync!();
    set_scope_path!();
    set_index!();
    lifecycle!();
    switch_state!();
}

impl GColorPanel {
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    ColorPanelEvent::Changed(ColorPanelChanged {
                        meta,
                        color: self.get_picker_color().to_hex_string(),
                    }),
                );
            });
        }
    }

    pub fn set_picker_pos(&mut self, cx: &mut Cx, rel: DVec2) {
        let last_picker_x = self.picker_x;
        let last_picker_y = self.picker_y;

        // 获取实际的矩形尺寸
        let area = self.draw_color_panel.area();
        let rect_size = area.rect(cx).size;

        // 在矩形区域内，直接映射坐标到选择器位置
        self.picker_x = (rel.x / rect_size.x).clamp(0.0, 1.0) as f32;
        self.picker_y = (rel.y / rect_size.y).clamp(0.0, 1.0) as f32;
        let mut change = false;
        if (last_picker_x - self.picker_x).abs() > 0.001 {
            self.draw_color_panel.picker_x = self.picker_x;
            change = true;
        }
        if (last_picker_y - self.picker_y).abs() > 0.001 {
            self.draw_color_panel.picker_y = self.picker_y;
            change = true;
        }
        if change {
            self.draw_color_panel.color = self.base_color;
            self.redraw(cx);
        }
    }
    /// 修复选择器位置
    /// 从当前用户传入的颜色计算出基础颜色的色值，并将传入的颜色映射到picker位置
    /// 例如:
    /// color = #ff1190 => base_color = #ff0088
    pub fn fix_picker_pos(&mut self) -> () {
        let target_rgb = vec3(self.color.x, self.color.y, self.color.z);
        // 第一步：从目标颜色计算出基础颜色(右上角的纯色)
        let base_color = self.calculate_base_color_from_target(target_rgb);
        if self.base_color != base_color {
            self.base_color = base_color;

            // 根据颜色面板的渐变逻辑反推基础颜色
            let base_rgb = vec3(self.base_color.x, self.base_color.y, self.base_color.z);

            // 第二步：在颜色面板中搜索最接近目标颜色的位置
            let mut best_x = 0.5;
            let mut best_y = 0.5;
            let mut min_distance = f32::MAX;

            // 使用较粗的网格进行快速搜索
            for i in 0..=20 {
                for j in 0..=20 {
                    let test_x = i as f32 / 20.0;
                    let test_y = j as f32 / 20.0;

                    // 计算这个位置对应的颜色
                    let white = vec3(1.0, 1.0, 1.0);
                    let black = vec3(0.0, 0.0, 0.0);

                    // 与draw_color_picker.rs中相同的颜色计算逻辑
                    let top_color = white + (base_rgb - white) * test_x;
                    let bottom_color = black + (base_rgb * 0.2 - black) * test_x;
                    let test_color = top_color + (bottom_color - top_color) * test_y;

                    // 计算颜色距离
                    let diff = target_rgb - test_color;
                    let distance = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;

                    if distance < min_distance {
                        min_distance = distance;
                        best_x = test_x;
                        best_y = test_y;
                    }
                }
            }

            // 在最佳位置附近进行精细搜索
            let search_range = 0.05; // 搜索范围
            let search_step = 0.005; // 搜索步长

            let start_x = (best_x - search_range).max(0.0);
            let end_x = (best_x + search_range).min(1.0);
            let start_y = (best_y - search_range).max(0.0);
            let end_y = (best_y + search_range).min(1.0);

            let mut x = start_x;
            while x <= end_x {
                let mut y = start_y;
                while y <= end_y {
                    let white = vec3(1.0, 1.0, 1.0);
                    let black = vec3(0.0, 0.0, 0.0);

                    let top_color = white + (base_rgb - white) * x;
                    let bottom_color = black + (base_rgb * 0.2 - black) * x;
                    let test_color = top_color + (bottom_color - top_color) * y;

                    let diff = target_rgb - test_color;
                    let distance = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;

                    if distance < min_distance {
                        min_distance = distance;
                        best_x = x;
                        best_y = y;
                    }

                    y += search_step;
                }
                x += search_step;
            }

            // 更新picker位置
            self.picker_x = best_x;
            self.picker_y = best_y;
        }
        // 更新着色器参数
        self.draw_color_panel.color = self.base_color;
        self.draw_color_panel.picker_x = self.picker_x;
        self.draw_color_panel.picker_y = self.picker_y;
    }

    /// 从目标颜色计算基础颜色（右上角的纯色）
    /// 参考Ant Design颜色选择器的逻辑
    fn calculate_base_color_from_target(&self, target_rgb: Vec3) -> Vec4 {
        // 如果目标颜色接近白色、黑色或灰色，返回红色作为默认基础颜色
        let max_component = target_rgb.x.max(target_rgb.y.max(target_rgb.z));
        let min_component = target_rgb.x.min(target_rgb.y.min(target_rgb.z));
        let saturation = if max_component > 0.001 {
            (max_component - min_component) / max_component
        } else {
            0.0
        };

        // 如果饱和度太低（接近灰色），返回默认红色
        if saturation < 0.1 {
            return vec4(1.0, 0.0, 0.0, 1.0);
        }

        // 找出主色调（RGB中的最大值）
        let mut base_rgb = vec3(0.0, 0.0, 0.0);

        // 确定主要颜色通道
        if target_rgb.x >= target_rgb.y && target_rgb.x >= target_rgb.z {
            // 红色为主
            base_rgb.x = 1.0;
            if target_rgb.y > target_rgb.z {
                // 红-绿混合，偏向黄色
                base_rgb.y = (target_rgb.y / target_rgb.x).clamp(0.0, 1.0);
            } else {
                // 红-蓝混合，偏向品红
                base_rgb.z = (target_rgb.z / target_rgb.x).clamp(0.0, 1.0);
            }
        } else if target_rgb.y >= target_rgb.x && target_rgb.y >= target_rgb.z {
            // 绿色为主
            base_rgb.y = 1.0;
            if target_rgb.x > target_rgb.z {
                // 绿-红混合，偏向黄色
                base_rgb.x = (target_rgb.x / target_rgb.y).clamp(0.0, 1.0);
            } else {
                // 绿-蓝混合，偏向青色
                base_rgb.z = (target_rgb.z / target_rgb.y).clamp(0.0, 1.0);
            }
        } else {
            // 蓝色为主
            base_rgb.z = 1.0;
            if target_rgb.x > target_rgb.y {
                // 蓝-红混合，偏向品红
                base_rgb.x = (target_rgb.x / target_rgb.z).clamp(0.0, 1.0);
            } else {
                // 蓝-绿混合，偏向青色
                base_rgb.y = (target_rgb.y / target_rgb.z).clamp(0.0, 1.0);
            }
        }

        // 归一化基础颜色，确保至少有一个通道为1.0
        let max_base = base_rgb.x.max(base_rgb.y.max(base_rgb.z));
        if max_base > 0.001 {
            base_rgb = base_rgb / max_base;
        }

        vec4(base_rgb.x, base_rgb.y, base_rgb.z, 1.0)
    }

    pub fn get_picker_color(&self) -> Vec4 {
        // 根据选择器位置计算颜色
        let x = self.picker_x;
        let y = self.picker_y;

        // 使用base_color而不是self.color来计算，这样更准确
        // let base_rgb = vec3(self.base_color.x, self.base_color.y, self.base_color.z);
        let base_rgb = vec3(self.color.x, self.color.y, self.color.z);
        let white = vec3(1.0, 1.0, 1.0);
        let black = vec3(0.0, 0.0, 0.0);

        // 与draw_color_picker.rs中完全相同的颜色计算逻辑
        let top_color = white + (base_rgb - white) * x;
        let bottom_color = black + (base_rgb * 0.2 - black) * x;

        // 垂直插值：从顶部颜色到底部颜色
        let final_color = top_color + (bottom_color - top_color) * y;

        vec4(final_color.x, final_color.y, final_color.z, 1.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColorPickerDragMode {
    Wheel,
    Rect,
    None,
}
