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
        
        
        fn sdf_draw_star(p: vec2, r: float, teeth: float, th: float, tang: float) -> float {
            let angle = atan(p.y, p.x);
            let segment_angle = 2.0 * PI / teeth;
            let local_angle = mod(angle + PI, segment_angle) - segment_angle * 0.5;
            let dist_from_center = length(p);
                                
            // Create teeth pattern
            let tooth_height = th;
            let tooth_width = segment_angle * tang;
            let tooth_factor = smoothstep_custom(tooth_width, 0.0, abs(local_angle));
            let radius_variation = r + tooth_height * tooth_factor;
                                
            return dist_from_center - radius_variation;
        }

        // Gear-like star shape for the first SVG
        fn sdf_gear_star(p: vec2, r: float) -> float {
            return sdf_draw_star(p, r, 10.0, 0.20, 0.58);
        }
        
        // Smooth organic star for the second SVG  
        fn sdf_organic_star(p: vec2, r: float) -> float {
            return sdf_draw_star(p, r, 9.0, 0.2, 0.84);
        }

        // SDF helper functions
        fn sdf_circle(p: vec2, r: float) -> float {
            // return length(p) - r;
            return sdf_draw_star(p, r, 2.0, 0.34, 0.86);
        }

        fn sdf_rounded(p: vec2, r: float) -> float {
            return sdf_draw_star(p, r, 8.0, 0.25, 0.72);
        }

        fn sdf_hexagon(p: vec2, r: float) -> float {
            return sdf_draw_star(p, r, 5.0, 0.12, 0.72);
        }

        fn sdf_octagon(p: vec2, r: float) -> float {
            return sdf_draw_star(p, r, 4.0, 0.4, 0.76);
        }

        fn sdf_oval(p: vec2, r: vec2) -> float {
            let k = vec2(1.0, r.y / r.x);
            return (length(p * k) - r.x) / k.y;
        }
        
        fn rotate(p: vec2, angle: float) -> vec2 {
            let c = cos(angle);
            let s = sin(angle);
            return vec2(p.x * c - p.y * s, p.x * s + p.y * c);
        }
        
        fn smoothstep_custom(edge0: float, edge1: float, x: float) -> float {
            let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
            return t * t * (3.0 - 2.0 * t);
        }
        
        // Material 3 ease-in-out function for smooth transitions
        fn ease_in_out(t: float) -> float {
            if t < 0.6 {
                return 2.0 * t * t;
            } else {
                return 1.0 - 2.0 * (1.0 - t) * (1.0 - t);
            }
        }
        
        // Enhanced easing for Material 3 style
        fn material3_ease(t: float) -> float {
            // Cubic bezier approximation for Material 3 standard easing
            let t2 = t * t;
            let t3 = t2 * t;
            return 2.0 * t2 - 1.0 * t3;
        }

        fn pixel(self) -> vec4 {
            if self.loading == 0.0 {
                return self.color;
            }

            let loading_size = self.rect_size * 0.9;
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let loading_dot_size = vec2(loading_size.x * 0.2 * 0.96);
            let rotate_time = self.time;
            let center = vec2(self.rect_size.x * 0.5, self.rect_size.y * 0.5);
            match self.mode{
                LoadingMode::Circle => {
                    return self.loading_circle(self.get_color());
                }
                LoadingMode::Dot => {
                    let r = loading_dot_size.x * 0.32;

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
                    // Normalize coordinates to [-1, 1]
                    let uv = (self.pos * 2.0 - 1.0) * vec2(1.0, -1.0);
                    
                    // Material 3 style animation timing
                    // Shape transition cycle (every 14 seconds) for shape morphing
                    let shape_cycle_time = self.time * 0.6;    // Slow shape transitions
                    
                    let shape_index = mod(floor(shape_cycle_time), 7.0);
                    let raw_shape_progress = fract(shape_cycle_time);
                    
                    // Apply Material 3 easing to shape transitions
                    let eased_shape_progress = material3_ease(raw_shape_progress);
                    
                    // Calculate rotation factor based on shape's visual symmetry
                    // This ensures each shape completes exactly one VISUAL rotation cycle
                    let mut rotation_factor = 1.0;
                    if shape_index < 0.5 { // Shape 1: Gear-like star (10 teeth)
                        rotation_factor = 1.0 / 10.0; // 1/10 rotation for full visual cycle
                    } else if shape_index < 1.5 { // Shape 2: Organic star (9 bulges)
                        rotation_factor = 1.0 / 9.0; // 1/9 rotation for full visual cycle
                    } else if shape_index < 2.5 { // Shape 3: Hexagon (6 sides)
                        rotation_factor = 1.0 / 6.0; // 1/6 rotation for full visual cycle
                    } else if shape_index < 3.5 { // Shape 4: Circle (infinite symmetry)
                        rotation_factor = 1.0 / 2.0; // Full rotation needed
                    } else if shape_index < 4.5 { // Shape 5: Rounded (8 bulges)
                        rotation_factor = 1.0 / 8.0; // 1/8 rotation for full visual cycle
                    } else if shape_index < 5.5 { // Shape 6: Octagon (4 major features)
                        rotation_factor = 1.0 / 4.0; // 1/4 rotation for full visual cycle
                    } else { // Shape 7: Oval (2-fold symmetry)
                        rotation_factor = 1.0 / 2.0; // 1/2 rotation for full visual cycle
                    }
                    
                    // Rotation tied to shape progress with visual symmetry correction
                    let base_rotation = raw_shape_progress * 2.0 * PI * rotation_factor;
                    
                    // Scale factor for shapes
                    let scale = 0.64;
                    let p = uv / scale;
                    
                    // Rotate the coordinate system
                    let rotated_p = rotate(p, base_rotation);
                    
                    // Calculate current and next shape distances
                    let mut current_dist = 1000.0;
                    let mut next_dist = 1000.0;
                    let next_shape_index = mod(shape_index + 1.0, 7.0);
                    
                    // Current shape
                    if shape_index < 0.5 { // Shape 1: Gear-like star (complex star)
                        current_dist = sdf_gear_star(rotated_p, 0.8);
                    } else if shape_index < 1.5 { // Shape 2: Organic star (rounded star)
                        current_dist = sdf_organic_star(rotated_p, 0.8);
                    } else if shape_index < 2.5 { // Shape 3: Hexagon
                        current_dist = sdf_hexagon(rotated_p, 0.85);
                    } else if shape_index < 3.5 { // Shape 4: Circle
                        current_dist = sdf_circle(rotated_p, 0.8);
                    } else if shape_index < 4.5 { // Shape 5: Rounded square/cross
                        current_dist = sdf_rounded(rotated_p, 0.8);
                    } else if shape_index < 5.5 { // Shape 6: Octagon
                        current_dist = sdf_octagon(rotated_p, 0.8);
                    } else { // Shape 7: Rounded square
                        current_dist = sdf_oval(rotated_p, vec2(0.66, 0.42));
                    }
                    
                    // For next shape, calculate its rotation factor too
                    let mut next_rotation_factor = 1.0;
                    if next_shape_index < 0.5 { // Shape 1: Gear-like star
                        next_rotation_factor = 1.0 / 10.0;
                    } else if next_shape_index < 1.5 { // Shape 2: Organic star
                        next_rotation_factor = 1.0 / 9.0;
                    } else if next_shape_index < 2.5 { // Shape 3: Hexagon
                        next_rotation_factor = 1.0 / 6.0;
                    } else if next_shape_index < 3.5 { // Shape 4: Circle
                        next_rotation_factor = 1.0 / 2.0;
                    } else if next_shape_index < 4.5 { // Shape 5: Rounded
                        next_rotation_factor = 1.0 / 8.0;
                    } else if next_shape_index < 5.5 { // Shape 6: Octagon
                        next_rotation_factor = 1.0 / 4.0;
                    } else { // Shape 7: Oval
                        next_rotation_factor = 1.0 / 2.0;
                    }
                    
                    let next_base_rotation = raw_shape_progress * 2.0 * PI * next_rotation_factor;
                    let next_rotated_p = rotate(p, next_base_rotation);
                    
                    // Next shape
                    if next_shape_index < 0.5 { // Shape 1: Gear-like star
                        next_dist = sdf_gear_star(next_rotated_p, 0.8);
                    } else if next_shape_index < 1.5 { // Shape 2: Organic star
                        next_dist = sdf_organic_star(next_rotated_p, 0.8);
                    } else if next_shape_index < 2.5 { // Shape 3: Hexagon
                        next_dist = sdf_hexagon(next_rotated_p, 0.85);
                    } else if next_shape_index < 3.5 { // Shape 4: Circle
                        next_dist = sdf_circle(next_rotated_p, 0.8);
                    } else if next_shape_index < 4.5 { // Shape 5: Rounded square/cross
                        next_dist = sdf_rounded(next_rotated_p, 0.8);
                    } else if next_shape_index < 5.5 { // Shape 6: Octagon
                        next_dist = sdf_octagon(next_rotated_p, 0.8);
                    } else { // Shape 7: Rounded square
                        next_dist = sdf_oval(next_rotated_p, vec2(0.66, 0.42));
                    }
                    
                    // Material 3 style transition - starts late and ends early for longer hold times
                    let transition_start = 0.8;
                    let transition_end = 1.0;
                    let transition_progress = clamp((eased_shape_progress - transition_start) / (transition_end - transition_start), 0.0, 1.0);
                    let final_transition = ease_in_out(transition_progress);
                    
                    let final_distance = mix(current_dist, next_dist, final_transition);
                    
                    // Create smooth edges with anti-aliasing
                    let edge_smoothness = 0.015;
                    let alpha = 1.0 - smoothstep_custom(-edge_smoothness, edge_smoothness, final_distance);
                    
                    // Early return for transparent background
                    if alpha <= 0.0 {
                        return vec4(0.0, 0.0, 0.0, 0.0); // Fully transparent background
                    }
                    
                    // Material 3 color - only apply to the shape itself
                    let base_color = self.get_color();
                    let pulse_speed = 1.5;
                    let color_variation = 0.92 + 0.08 * sin(self.time * pulse_speed);
                    let color = base_color * color_variation;
                    
                    // Add subtle glow effect for Material 3 style
                    let glow_intensity = 0.06;
                    let glow = exp(-abs(final_distance) * 5.0) * glow_intensity;
                    let final_alpha = min(alpha + glow, 1.0) * self.loading;
                    
                    // Return color only where the shape exists, transparent elsewhere
                    return vec4(color.rgb, final_alpha);
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
    #[live(0.0)]
    pub loading: f32,
}

impl DrawLoading {
    pub fn merge(&mut self, prop: &LoadingBasicStyle) {
        self.color = prop.color;
    }
}