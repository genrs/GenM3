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
        // // helper: signed distance to regular n-gon of radius 'rad'
        // fn sd_reg_poly(self, p: vec2, n: float, rad: float) -> float {
        //     let an = PI * 2.0 / n;
        //     let a = atan(p.y, p.x);
        //     let ang = mod(a + an * 0.5, an) - an * 0.5;
        //     let q = vec2(cos(ang), sin(ang));
        //     return length(p - rad * q);
        // }

        // // ellipse distance helper (approximate)
        // fn sd_ellipse(self, p: vec2, rx: float, ry: float, _rad: float) -> float {
        //     let q = vec2(p.x / rx, p.y / ry);
        //     return length(q) - 1.0;
        // }

        // // produce a mask in [0,1] for different shape ids
        // fn shape_mask(self, id: float, p: vec2, base_rad: float) -> float {
        //     let edge = base_rad * 0.06;
        //     if id == 0.0 {
        //         let d = length(p) - base_rad;
        //         return smoothstep(edge, -edge, d);
        //     }
        //     else if id == 1.0 {
        //         let rx = base_rad * 1.25;
        //         let ry = base_rad * 0.7;
        //         let d = sd_ellipse(self, p, rx, ry, 1.0);
        //         return smoothstep(edge, -edge, d * base_rad);
        //     }
        //     else if id == 2.0 {
        //         let rx = base_rad * 1.25;
        //         let ry = base_rad * 0.6;
        //         let d1 = sd_ellipse(self, p, rx, ry, 1.0) * base_rad;
        //         let p2 = vec2(p.y, -p.x);
        //         let d2 = sd_ellipse(self, p2, rx, ry, 1.0) * base_rad;
        //         let m1 = smoothstep(edge, -edge, d1);
        //         let m2 = smoothstep(edge, -edge, d2);
        //         return max(m1, m2);
        //     }
        //     else if id == 3.0 {
        //         let d = sd_reg_poly(self, p, 4.0, base_rad);
        //         let corner = base_rad * 0.25;
        //         return smoothstep(edge, -edge, d - corner);
        //     }
        //     else if id == 4.0 {
        //         let d = sd_reg_poly(self, p, 5.0, base_rad);
        //         let corner = base_rad * 0.18;
        //         return smoothstep(edge, -edge, d - corner);
        //     }
        //     else if id == 5.0 {
        //         let d1 = sd_reg_poly(self, p, 4.0, base_rad);
        //         let ang = 45.0 * (PI/180.0);
        //         let rp = vec2(p.x * cos(ang) - p.y * sin(ang), p.x * sin(ang) + p.y * cos(ang));
        //         let d2 = sd_reg_poly(self, rp, 4.0, base_rad);
        //         return max(smoothstep(edge, -edge, d1), smoothstep(edge, -edge, d2));
        //     }
        //     else {
        //         // id == 6.0 -> combine 5 rotated ellipses
        //         let step = 360.0 / 5.0;
        //         let ang0 = (0.0 * step) * (PI / 180.0);
        //         let ang1 = (1.0 * step) * (PI / 180.0);
        //         let ang2 = (2.0 * step) * (PI / 180.0);
        //         let ang3 = (3.0 * step) * (PI / 180.0);
        //         let ang4 = (4.0 * step) * (PI / 180.0);
        //         let rx = base_rad * 0.6;
        //         let ry = base_rad * 0.9;
        //         let rp0 = vec2(p.x * cos(ang0) - p.y * sin(ang0), p.x * sin(ang0) + p.y * cos(ang0));
        //         let rp1 = vec2(p.x * cos(ang1) - p.y * sin(ang1), p.x * sin(ang1) + p.y * cos(ang1));
        //         let rp2 = vec2(p.x * cos(ang2) - p.y * sin(ang2), p.x * sin(ang2) + p.y * cos(ang2));
        //         let rp3 = vec2(p.x * cos(ang3) - p.y * sin(ang3), p.x * sin(ang3) + p.y * cos(ang3));
        //         let rp4 = vec2(p.x * cos(ang4) - p.y * sin(ang4), p.x * sin(ang4) + p.y * cos(ang4));
        //         let d0 = sd_ellipse(self, rp0, rx, ry, 1.0) * base_rad;
        //         let d1 = sd_ellipse(self, rp1, rx, ry, 1.0) * base_rad;
        //         let d2 = sd_ellipse(self, rp2, rx, ry, 1.0) * base_rad;
        //         let d3 = sd_ellipse(self, rp3, rx, ry, 1.0) * base_rad;
        //         let d4 = sd_ellipse(self, rp4, rx, ry, 1.0) * base_rad;
        //         let m0 = smoothstep(edge, -edge, d0);
        //         let m1 = smoothstep(edge, -edge, d1);
        //         let m2 = smoothstep(edge, -edge, d2);
        //         let m3 = smoothstep(edge, -edge, d3);
        //         let m4 = smoothstep(edge, -edge, d4);
        //         return max(max(max(m0, m1), max(m2, m3)), m4);
        //     }
        // }
        fn pixel(self) -> vec4 {
            if self.loading == 0.0 {
                return self.color;
            }

            let loading_size =  self.rect_size * 0.9;
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
                    // let r = self.rect_size;
                    // let uv = (self.pos * r - r * 0.5) / (r.y * 0.5);
                    // let shapes = 7.0;
                    // let speed = 0.6;
                    // let cycle = fract(self.time * speed);
                    // let idxf = cycle * shapes;
                    // let idx = floor(idxf);
                    // let t = smoothstep(0.0, 1.0, fract(idxf));
                    // let base_rad = min(r.x, r.y) * 0.35;
                    // let cur = shape_mask(self, idx, uv, base_rad);
                    // let next = shape_mask(self, mod(idx + 1.0, shapes), uv, base_rad);
                    // let mask = mix(cur, next, t);
                    // let col = self.get_color();
                    // return vec4(col.rgb, mask * col.a);
                    sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                    sdf.stroke(self.get_color(), 2.0);
                    let start_pos = (self.rect_size - loading_size) * vec2(0.5, 0.5);

                    // sdf.box_all(
                    //     start_pos.x,
                    //     start_pos.y,
                    //     loading_size.x,
                    //     loading_size.y,
                    //     loading_size.x * 0.1,
                    //     loading_size.x * 0.1,
                    //     loading_size.x * 0.1,
                    //     loading_size.x * 0.1
                    // );
                    sdf.box_all(
                        start_pos.x,
                        start_pos.y,
                        loading_size.x,
                        loading_size.y * 0.2,
                        loading_size.x * 0.1,
                        loading_size.x * 0.1,
                        loading_size.x * 0.1,
                        loading_size.x * 0.1
                    );
                    sdf.fill(self.get_color());
                }
                LoadingMode::Classic => {
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
