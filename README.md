# GenM3

- version: `v0.1.0`
- author: [Will-YiFei Sheng](syf20020816@outlook.com)

```
uProgressMode::Circle => {
                    // [draw a ring progress bar] ------------------------------------------------
                    let center_pos = vec2(self.pos.x + self.rect_size.x * 0.5, self.pos.y + self.rect_size.y * 0.5);
                    let offset = max(
                        max(self.border_radius.x, max(self.border_radius.y, max(self.border_radius.z, self.border_radius.w))),
                    32.0) * 0.25;
                    // 0.8 is a magic offset which can make circle look better
                    let ring_outer_radius = min(self.rect_size.x, self.rect_size.y) * 0.5 - self.border_width * 2.0 - offset ;
                    sdf.circle(center_pos.x, center_pos.y, min(self.rect_size.x, self.rect_size.y) * 0.5);
                    sdf.fill_premul(#f00);
                    sdf.circle(center_pos.x, center_pos.y, min(self.rect_size.x, self.rect_size.y) * 0.5 - offset * 4.0);
                    sdf.fill_premul(#ff0);
                    let ring_inner_radius = ring_outer_radius - offset * 2.0;
                    let ring_arc_radius = ring_outer_radius;
                    sdf.circle(center_pos.x, center_pos.y, ring_outer_radius);
                    sdf.circle(center_pos.x, center_pos.y, ring_inner_radius);
                    sdf.subtract();
                    if self.background_visible == 1.0 {
                        sdf.fill_premul(self.get_background_color());
                    }
                    if self.border_width > 0.0 {
                        sdf.stroke_keep(self.get_border_color(), self.border_width);
                    }
                    
                    // [draw the progress part] ----------------------------------------------------
                    if self.in_progress == 1.0 {
                        // 圆环波浪进度条参数
                        let wave_count = 12.0; // 圆环上的波浪数量
                        let wave_amplitude = offset * 2.0; // 波浪振幅
                        let wave_thickness = offset * 2.0; // 波浪线条厚度
                        let x = 0.0;
                        for i in 0..12 {
                            if x > 3.0 {break;}
                            if i < int(self.value * wave_count) {
                                // 计算每个波浪在圆环上的角度位置
                                // 强行将起点设置在坐标轴1号区域(右上区域)
                                let wave_angle = (x / wave_count) * 360.0 * PI / 180.0 + 180.0 * PI / 180.0;

                                // 检查是否在进度范围内
                                if wave_angle <= (self.value * 360.0) * PI / 180.0 + 180.0 * PI / 180.0{
                                    // 计算波浪中心位置
                                    let wave_offset = ((abs(self.value * 10.0 - float(int(self.value * 10.0))) + 1.0) * offset * 0.5);
                                    let wave_center_x = center_pos.x - cos(wave_angle) * ring_arc_radius;
                                    let wave_center_y = center_pos.y + sin(wave_angle) * ring_arc_radius;
                                    
                                    if mod(x, 2.0) == 0.0 {
                                        // 外波浪 - 向外的弧线
                                        let start_deg = 25.0 ;
                                        let end_deg = start_deg + 90.0 * one_deg;
                                        
                                        if x == 0.0 {
                                            sdf.arc_round_caps(
                                                wave_center_x - wave_offset * 0.25,
                                                wave_center_y - wave_thickness * 1.25 - wave_offset,
                                                wave_amplitude * 1.25,
                                                25.0 * one_deg,
                                                115.0 * one_deg,
                                                wave_thickness
                                            );
                                        }
                                        if x == 2.0 {
                                            sdf.arc_round_caps(
                                                wave_center_x - wave_offset * 2.5,
                                                wave_center_y - wave_thickness * 0.25 - wave_offset,
                                                wave_amplitude * 1.25,
                                                -25.0 * one_deg,
                                                65.0 * one_deg,
                                                wave_thickness
                                            );
                                        }
                                    } else {
                                        // 内波浪 - 向内的弧线
                                        sdf.arc_round_caps(
                                            wave_center_x - wave_amplitude * 1.25 - wave_offset,
                                            wave_center_y - wave_offset * 0.25,
                                            wave_amplitude * 1.25,
                                            180.0 * one_deg,
                                            270.0 * one_deg,
                                            wave_thickness 
                                        );
                                    }
                                    sdf.fill_premul(self.get_color());
                                }
                            }
                            x += 1.0;
                        }
                    } else {
                        sdf.arc_round_caps(
                            center_pos.x,
                            center_pos.y,
                            ring_arc_radius,
                            180.0 * one_deg,
                            (self.value * 360.0 + 180.0) * one_deg,
                            offset
                        );
                        sdf.fill_premul(self.get_color());
                    }
                    sdf.rect(0.0, center_pos.y, self.sdf_rect_size.x, 2.0);
                    sdf.rect(center_pos.x, 0.0, 2.0, self.sdf_rect_size.y);
                    sdf.fill(#000);
                }
            }
```

fn atan2(y: float, x: float) -> float {
            if x > 0.0 { return atan(y / x); }
            else if x < 0.0 {
                if y >= 0.0 { return atan(y / x) + PI; }
                else { return atan(y / x) - PI; }
            } else { // x == 0
                if y > 0.0 { return PI / 2.0; }
                else if y < 0.0 { return -PI / 2.0; }
                else { return 0.0; } // (0,0) 无定义，返回0
            }
        }

        fn circle_wave(self) -> vec4 {
            let uv = self.pos * 2.0 - 1.0; // [-1, 1] 坐标系
            let len = length(uv);
            let angle = atan2(uv.y, uv.x); // [-π, π]

            // 转换到 [0, 1] 角度范围
            let normalized_angle = (angle + PI) / (PI * 2.0);

            // 波浪扰动：半径随角度变化
            let wave_freq = 8.0; // 波浪数量
            let wave_amp = 0.15; // 波浪幅度（相对于半径）
            let radius_offset = sin(angle * wave_freq + self.phase) * wave_amp;

            // 基础半径（归一化后为 0.9，留边距）
            let base_radius = 0.9;
            let wave_radius = base_radius + radius_offset;

            // 是否在波浪环内？
            let in_ring = abs(len - wave_radius) < 0.03; // 环宽度

            // 是否在进度范围内？
            let show_angle = normalized_angle < self.value;

            // 抗锯齿边缘
            let aa = 0.01;
            let edge_fade = smoothstep(self.value - aa, self.value, normalized_angle);
            let mask = 1.0 - edge_fade;

            // 最终颜色
            let color = #7b68ee;
            let alpha = float(in_ring) * float(show_angle) * mask;
            // let alpha = 0.0;

            return vec4(color.rgb, alpha);
        }