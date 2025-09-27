mod prop;
mod event;

pub use prop::*;
pub use event::*;

use makepad_widgets::*;

use crate::shader::draw_color_picker::DrawColorPanel;

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;
    use link::shaders::*;

    pub GColorPanelBase = {{GColorPanel}}{
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_color_panel: { down: 0.0, hover: 0.0}
                    }
                }

                on = {
                    cursor: Arrow,
                    from: {
                        all: Forward {duration: 0.1}
                        down: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_color_panel: {
                            down: 0.0,
                            hover: [{time: 0.0, value: 1.0}],
                        }
                    }
                }

                down = {
                    cursor: Arrow,
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_color_panel: {
                            down: [{time: 0.0, value: 1.0}],
                            hover: 1.0,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct GColorPanel {
    #[live]
    pub style: ColorPanelStyle,
    #[redraw]
    #[live]
    draw_color_panel: DrawColorPanel,
    #[animator]
    animator: Animator,
    #[walk]
    walk: Walk,
    #[rust]
    pub size: f64,
    #[rust]
    picker_x: f32,
    #[rust]
    picker_y: f32,
    #[rust]
    base_color: Vec4, // 基础颜色（如红色）
    #[rust(ColorPickerDragMode::None)]
    drag_mode: ColorPickerDragMode,
    #[rust]
    pub state: ColorPanelState,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ColorPickerAction {
    Change { rgba: Vec4 },
    DoneChanging,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColorPickerDragMode {
    Wheel,
    Rect,
    None,
}

impl GColorPanel {
    pub fn handle_finger(&mut self, cx: &mut Cx, rel: DVec2, scope: &mut Scope) {
        fn clamp(x: f64, mi: f64, ma: f64) -> f64 {
            if x < mi {
                mi
            } else if x > ma {
                ma
            } else {
                x
            }
        }

        let last_picker_x = self.picker_x;
        let last_picker_y = self.picker_y;

        // 获取实际的矩形尺寸
        let area = self.draw_color_panel.area();
        let rect_size = area.rect(cx).size;

        // 在矩形区域内，直接映射坐标到选择器位置
        self.picker_x = clamp(rel.x / rect_size.x, 0.0, 1.0) as f32;
        self.picker_y = clamp(rel.y / rect_size.y, 0.0, 1.0) as f32;

        // 更新绘制参数
        let mut changed = false;

        if (last_picker_x - self.picker_x).abs() > 0.001 {
            self.draw_color_panel
                .apply_over(cx, live! {picker_x: (self.picker_x)});
            changed = true;
        }
        if (last_picker_y - self.picker_y).abs() > 0.001 {
            self.draw_color_panel
                .apply_over(cx, live! {picker_y: (self.picker_y)});
            changed = true;
        }
        if changed {
            let uid = self.widget_uid();
            cx.widget_action(
                uid,
                &scope.path,
                ColorPickerAction::Change {
                    rgba: self.to_rgba(),
                },
            );
        }
    }

    pub fn to_rgba(&self) -> Vec4 {
        // 根据选择器位置计算颜色
        let x = self.picker_x;
        let y = self.picker_y;

        // 水平插值：从白色到基础颜色（顶部），从黑色到暗化基础颜色（底部）
        let white = vec3(1.0, 1.0, 1.0);
        let black = vec3(0.0, 0.0, 0.0);
        let base_rgb = vec3(self.base_color.x, self.base_color.y, self.base_color.z);

        let top_color = white + (base_rgb - white) * x;
        let bottom_color = black + (base_rgb * 0.2 - black) * x;

        // 垂直插值：从顶部颜色到底部颜色
        let final_color = top_color + (bottom_color - top_color) * y;

        vec4(final_color.x, final_color.y, final_color.z, 1.0)
    }

    pub fn draw_color_picker(&mut self, cx: &mut Cx2d, base_color: Vec4, walk: Walk) {
        // 设置基础颜色
        self.base_color = base_color;

        // 更新绘制参数
        self.size = cx.turtle().rect().size.y.min(cx.turtle().rect().size.x);
        self.draw_color_panel.color = base_color;
        self.draw_color_panel.picker_x = self.picker_x;
        self.draw_color_panel.picker_y = self.picker_y;
        self.draw_color_panel.draw_walk(cx, walk);
    }
}

impl Widget for GColorPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.draw_color_panel.area()) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(fe) => {
                self.animator_play(cx, id!(hover.down));
                let rel = fe.abs - fe.rect.pos;
                // 对于矩形区域，任何点击都是有效的
                self.drag_mode = ColorPickerDragMode::Rect;
                return self.handle_finger(cx, rel, scope);
            }
            Hit::FingerUp(fe) => {
                if fe.is_over && fe.device.has_hovers() {
                    self.animator_play(cx, id!(hover.on));
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
                self.drag_mode = ColorPickerDragMode::None;
                let uid = self.widget_uid();
                cx.widget_action(uid, &scope.path, ColorPickerAction::DoneChanging);
            }
            Hit::FingerMove(fe) => {
                let rel = fe.abs - fe.rect.pos;
                return self.handle_finger(cx, rel, scope);
            }
            _ => (),
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_color_picker(cx, vec4(1.0, 0.0, 0.0, 1.0), walk);
        DrawStep::done()
    }
}
