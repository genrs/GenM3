use makepad_widgets::*;

live_design! {
    use link::shaders::*;
    
    DrawColorPanel = {{DrawColorPanel}} {
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
            let bottom_color = mix(vec3(0.0), self.color.xyz * 0.1, x);

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
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawColorPanel {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4, // 选择器选择的颜色
    #[live]
    pub picker_x: f32, // 选择器X位置 (0.0-1.0)
    #[live]
    pub picker_y: f32, // 选择器Y位置 (0.0-1.0)
}
