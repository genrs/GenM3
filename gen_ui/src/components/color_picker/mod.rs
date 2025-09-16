use makepad_widgets::*;

live_design!{
    link genui_basic;
    use link::genui_animation_prop::*;
    use link::shaders::*;
    DrawColorBox= {{DrawColorBox}} {
        instance hover: float
        instance down: float
        // instance picker_x: float
        // instance picker_y: float
        
        fn pixel(self) -> vec4 {
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            
            // 计算渐变颜色
            // 左上角: 白色 (1,1,1)
            // 右上角: 纯色 (self.color.rgb)  
            // 左下角: 黑色 (0,0,0)
            // 右下角: 暗化的纯色
            
            let x = self.pos.x; // 0.0 到 1.0
            let y = self.pos.y; // 0.0 到 1.0
            
            // 水平插值：从白色到纯色（顶部），从黑色到暗化纯色（底部）
            let top_color = mix(vec3(1.0), self.color.xyz, x);
            let bottom_color = mix(vec3(0.0), self.color.xyz * 0.2, x);
            
            // 垂直插值：从顶部颜色到底部颜色
            let final_color = mix(top_color, bottom_color, y);
            
            // 绘制背景渐变
            sdf.rect(0.0, 0.0, w, h);
            sdf.fill(vec4(final_color, 1.0));
            
            // 绘制圆形选择器
            let picker_pos = vec2(self.picker_x * w, self.picker_y * h);
            let mark_size = 8.0;
            
            // 选择器外圈（白色边框）
            sdf.circle(picker_pos.x, picker_pos.y, mark_size + 1.0);
            sdf.fill(mix(mix(#FFF, #EEE, self.hover), #DDD, self.down));
            
            // 选择器内圈（黑色边框）
            // sdf.circle(picker_pos.x, picker_pos.y, mark_size);
            // sdf.fill(#000);
            
            // 选择器中心（透明，显示底层颜色）
            sdf.circle(picker_pos.x, picker_pos.y, mark_size - 2.0);
            sdf.fill(vec4(final_color, 1.0));
            
            return sdf.result;
        }
    }
    pub GColorPickerBase = {{GColorPicker}}{
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_box: { down: 0.0, hover: 0.0}
                    }
                }
                
                on = {
                    cursor: Arrow,
                    from: {
                        all: Forward {duration: 0.1}
                        down: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_box: {
                            down: 0.0,
                            hover: [{time: 0.0, value: 1.0}],
                        }
                    }
                }
                
                down = {
                    cursor: Arrow,
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_box: {
                            down: [{time: 0.0, value: 1.0}],
                            hover: 1.0,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawColorBox {
    #[deref] draw_super: DrawQuad,
    #[live] color: Vec4, // 选择器选择的颜色
    #[live] picker_x: f32, // 选择器X位置 (0.0-1.0)
    #[live] picker_y: f32, // 选择器Y位置 (0.0-1.0)
}


#[derive(Live, LiveHook, Widget)]
pub struct GColorPicker {
   #[redraw] #[live] draw_box: DrawColorBox,
    
    #[animator] animator: Animator,
    
    #[walk] walk: Walk,
        
    #[rust] pub size: f64,
    #[rust] picker_x: f32,
    #[rust] picker_y: f32,
    #[rust] base_color: Vec4, // 基础颜色（如红色）
    #[rust(ColorPickerDragMode::None)] drag_mode: ColorPickerDragMode
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ColorPickerAction {
    Change {rgba: Vec4},
    DoneChanging,
    None
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColorPickerDragMode {
    Wheel,
    Rect,
    None
}

impl GColorPicker {
    
    pub fn handle_finger(&mut self, cx: &mut Cx, rel: DVec2, scope:&mut Scope) {
        
        fn clamp(x: f64, mi: f64, ma: f64) -> f64 {if x < mi {mi} else if x > ma {ma} else {x}}
        
        let last_picker_x = self.picker_x;
        let last_picker_y = self.picker_y;
        
        // 获取实际的矩形尺寸
        let area = self.draw_box.area();
        let rect_size = area.rect(cx).size;
        
        // 在矩形区域内，直接映射坐标到选择器位置
        self.picker_x = clamp(rel.x / rect_size.x, 0.0, 1.0) as f32;
        self.picker_y = clamp(rel.y / rect_size.y, 0.0, 1.0) as f32;
        
        // 更新绘制参数
        let mut changed = false;
        
        if (last_picker_x - self.picker_x).abs() > 0.001 {
            self.draw_box.apply_over(cx, live!{picker_x: (self.picker_x)});
            changed = true;
        }
        if (last_picker_y - self.picker_y).abs() > 0.001 {
            self.draw_box.apply_over(cx, live!{picker_y: (self.picker_y)});
            changed = true;
        }
        if changed {
            let uid = self.widget_uid();
            cx.widget_action(uid, &scope.path, ColorPickerAction::Change {rgba: self.to_rgba()});
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
        self.draw_box.color = base_color;
        self.draw_box.picker_x = self.picker_x;
        self.draw_box.picker_y = self.picker_y;
        self.draw_box.draw_walk(cx, walk);
    }
}


impl Widget for GColorPicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);
                
        match event.hits(cx, self.draw_box.area()) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            },
            Hit::FingerDown(fe) => {
                self.animator_play(cx, id!(hover.down));
                let rel = fe.abs - fe.rect.pos;
                // 对于矩形区域，任何点击都是有效的
                self.drag_mode = ColorPickerDragMode::Rect;
                return self.handle_finger(cx, rel, scope);
            },
            Hit::FingerUp(fe) => {
                if fe.is_over && fe.device.has_hovers() {
                    self.animator_play(cx, id!(hover.on));
                }
                else {
                    self.animator_play(cx, id!(hover.off));
                }
                self.drag_mode = ColorPickerDragMode::None;
                let uid = self.widget_uid();
                cx.widget_action(uid, &scope.path, ColorPickerAction::DoneChanging);
            }
            Hit::FingerMove(fe) => {
                let rel = fe.abs - fe.rect.pos;
                return self.handle_finger(cx, rel, scope)
                                
            },
            _ => ()
        }
    }
        
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_color_picker(cx, vec4(1.0,0.0,0.0,1.0), walk);
        DrawStep::done()
    }
}