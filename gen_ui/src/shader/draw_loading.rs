use makepad_widgets::*;

use crate::{components::LoadingBasicStyle, prop::LoadingMode};

live_design! {
    use link::shaders::*;
    DrawLoading = {{DrawLoading}}{
        fn get_color(self) -> vec4 {
            return self.color;
        }

        fn loading_circle(self, color: vec4) -> vec4 {

            let uv = self.pos * self.rect_size;
            let center = self.rect_size * 0.5;
            let aspect = self.rect_size.x / self.rect_size.y;

            // 将UV坐标调整为以中心为原点，并考虑宽高比
            let adjusted_uv = (uv - center) / vec2(aspect, 1.0);

            let radius = min(self.rect_size.x, self.rect_size.y) * 0.4;
            let line_width = min(self.rect_size.x, self.rect_size.y) * 0.03;
            let glow_size = line_width * 3.0;

            let len = length(adjusted_uv);
            let angle = atan(adjusted_uv.y, adjusted_uv.x);

            // 计算旋转和渐变效果
            let rotation_speed = 0.5;
            let fall_off = fract(-0.5 * (angle / PI) - self.time * rotation_speed);

            // 计算圆环的形状
            let circle_shape = smoothstep(line_width, 0.0, abs(radius - len));

            // 添加发光效果
            let glow = smoothstep(glow_size * fall_off, 0.0, abs(radius - len) - line_width * 0.5) * 0.5;

            // 组合形状和发光效果
            let shape = (circle_shape + glow) * fall_off;

            // 创建颜色渐变
            let gradient_color = mix(vec4(color.rgb, 0.1), color, fall_off);

            return gradient_color * shape;
        }
        fn rotating_radial_pattern(self) -> vec4 {
            let r = self.rect_size;
            let u = (self.pos * r * 2.0 - r) / (r.y * 0.5);

            // 初始化输出颜色
            let o = vec4(0.0);

            // 创建基本形状
            let shape = pow(abs(dot(u, u) - 2.0), 18.0);
            o -= vec4(shape, shape, shape, shape);

            // 计算旋转角度
            let angle = atan(u.y, u.x) / 0.7854;
            let rotation = ceil(8.0 * self.time) - angle;
            let fract_rotation = rotation - floor(rotation);

            let f = fract_rotation - vec2(0.0, 0.0);
            let t_f = f.y;
            // 创建平滑的过渡
            let transition = smoothstep(0.0, 0.12, f.y);

            // 创建旋转效果
            let pattern = floor(rotation);

            if transition == 1.0 {
                if mod(pattern, 8.0) - 1.0 < 0.0 {
                    o += vec4(0.8);
                } else {
                    o += self.get_color();
                }
            }

            return o;
        }
        fn pixel(self) -> vec4 {
            if self.loading == 0.0 {
                return self.color;
            }

            let loading_size =  self.rect_size * 0.86;
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let loading_dot_size = vec2(loading_size.x * 0.2 * 0.96);
            let rotate_time = self.time;
            let center = vec2(self.rect_size.x * 0.5, self.rect_size.y * 0.5);
            match self.mode{
                LoadingMode::Circle => {
                    return self.loading_circle(self.get_color());
                }
                LoadingMode::Dot => {
                    let r = loading_dot_size.x * 0.5;

                    let spacing = (loading_size.x - 8.0 * r) * 0.25 + 2.0 * r;
                    // let phase = (rotate_time / 2.0 - rotate_time) / 2.0;
                    let num_dots = 5;
                    let counter = 0.0;
                    for i in 0..5 {
                        // let t = counter / 4;
                        // let offset = abs(phase - t) * loading_size.x * 0.5;
                        let offset = abs(2.0 - counter) * spacing ;
                        if counter < 2.0 {
                            let dot_pos = vec2(center.x + offset * sin(rotate_time), center.y);

                            sdf.circle(dot_pos.x, dot_pos.y, r);
                        }else{
                            let dot_pos = vec2(center.x - offset * sin(rotate_time), center.y);

                            sdf.circle(dot_pos.x, dot_pos.y, r);
                        }
                        sdf.fill(self.get_color());
                        counter += 1.0;
                    }
                }
                LoadingMode::Polygons => {
                    return self.rotating_radial_pattern();
                }
            }

            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawLoading {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live]
    pub mode: LoadingMode,
    #[live(1.0)]
    pub loading: f32,
}

impl DrawLoading {
    pub fn merge(&mut self, prop: &LoadingBasicStyle) {
        self.color = prop.color;
    }
}
