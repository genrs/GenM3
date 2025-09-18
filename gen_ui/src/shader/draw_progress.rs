use makepad_widgets::*;

use crate::{
    components::ProgressBasicStyle,
    prop::{ProgressMode, traits::ToFloat},
    shader::draw_view::DrawView,
};

live_design! {
    use link::shaders::*;
    DrawProgress = {{DrawProgress}}{
        fn get_color(self) -> vec4 { return self.color; }

        // each wave_r should add `0.4` to make the wave look better (magic number)
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size3);
            let one_deg = PI / 180.0;
            // - [draw progress bar] --------------------------------------------------------------
            match self.mode {
                ProgressMode::Horizontal => {
                    // - [draw shadow and blur] -----------------------------------------------------------
                    if sdf.shape > -1.0 {
                        if self.spread_radius > 0.0 || self.blur_radius > 0.0 {
                            let shadow_offset = self.shadow_offset + self.rect_shift;
                            let total_shadow_size = self.spread_radius + self.blur_radius;
                            let shadow_lower = vec2(total_shadow_size) + shadow_offset;
                            let shadow_upper = self.rect_size + vec2(self.spread_radius * 2.0) + shadow_offset;
                            if self.border_radius.x != 0.0 || self.border_radius.y != 0.0 ||
                                self.border_radius.z != 0.0 || self.border_radius.w != 0.0 {
                                let max_border_radius = max(
                                    max(
                                        max(self.border_radius.x, self.border_radius.y),
                                        max(self.border_radius.z, self.border_radius.w)
                                    ), 1.0
                                );
                                let v = GaussShadow::rounded_box_shadow(
                                    shadow_lower,
                                    shadow_upper,
                                    self.pos * self.rect_size3,
                                    self.blur_radius,
                                    max_border_radius
                                );
                                let shadow_color = vec4(self.get_shadow_color().rgb, self.get_shadow_color().a * v);
                                sdf.clear(shadow_color);
                            } else {
                                let v = GaussShadow::box_shadow(
                                    shadow_lower,
                                    shadow_upper,
                                    self.pos * self.rect_size3,
                                    self.blur_radius
                                );
                                let shadow_color = vec4(self.get_shadow_color().rgb, self.get_shadow_color().a * v);
                                sdf.clear(shadow_color);
                            }
                        }
                    }

                    // - [basic sdf for draw a view] ------------------------------------------------------
                    let border_width = self.border_width;
                    let total_shadow_size = self.spread_radius + self.blur_radius;
                    let wave_offset = self.sdf_rect_size.y * 0.25;
                    let progress_width = self.sdf_rect_size.x * self.value;
                    // 使用calculated位置而不是原始rect_size
                    sdf.box_all(
                        self.sdf_rect_pos.x + progress_width,
                        self.sdf_rect_pos.y + wave_offset,
                        self.sdf_rect_size.x - progress_width,
                        self.sdf_rect_size.y * 0.5,
                        max(self.border_radius.x * 0.5, 1.0),
                        max(self.border_radius.y * 0.5, 1.0),
                        max(self.border_radius.z * 0.5, 1.0),
                        max(self.border_radius.w * 0.5, 1.0)
                    );

                    // - [background color if visible] ----------------------------------------------------
                    if self.background_visible == 1.0 {
                        sdf.fill_premul(self.get_background_color());
                    }

                    // - [border with and color if width bigger than 0] -----------------------------------
                    if border_width > 0.0 {
                        sdf.stroke_keep(self.get_border_color(), border_width);
                    }
                    // [draw a small dot in the end of the progress bar] --------------------------
                    let dot_radius = 4.0;
                    let dot_pos = vec2(
                        self.rect_size.x + self.pos.x - dot_radius * 2.0 - self.border_width,
                        self.pos.y + self.rect_size.y * 0.5
                    );
                    sdf.circle(dot_pos.x, dot_pos.y, dot_radius);
                    sdf.fill_premul(self.get_color());

                    // [draw the progress bar] ----------------------------------------------------
                    if self.in_progress == 1.0 {
                        // 波浪进度条参数
                        let center_y = self.pos.y + self.rect_size.y * 0.5;
                        // 使用更少的段数但更大的弧度来创建更连续的波浪
                        let mut x = 0.0;
                        for i in 0..20 {
                            if i < int(self.value * 20.0) {
                                let wave_r = self.sdf_rect_size.y;
                                // 因为最后一个波浪永远需要是完整的，所以需要在这里做一个偏移
                                let wave_x = (self.sdf_rect_pos.x + (2.0 * x + 1.0) * wave_r - 2.5 * x * wave_offset) - (abs(self.value * 10.0 - float(int(self.value * 10.0))) * wave_offset * 0.5);
                                if wave_x > (self.sdf_rect_pos.x + self.sdf_rect_size.x * self.value) {
                                    break;
                                }

                                if mod(x, 2.0) == 0.0 {
                                    // 下波浪 - 弧线向下，使用更大的弧度范围
                                    sdf.arc_round_caps(
                                        wave_x,
                                        0.0 - wave_offset * 1.0,
                                        wave_r + 0.4,
                                        -45.0 * one_deg,
                                        45.0 * one_deg,
                                        wave_offset * 2.0
                                    );
                                } else {
                                    // 上波浪 - 弧线向上，使用更大的弧度范围
                                    sdf.arc_round_caps(
                                        wave_x,
                                        self.sdf_rect_size.y + wave_offset * 1.0,
                                        wave_r + 0.4,
                                        135.0 * one_deg,
                                        225.0 * one_deg,
                                        wave_offset * 2.0
                                    );
                                }
                                sdf.fill_keep(self.get_color());
                            }
                            x += 1.0;
                        }

                    }else{
                        sdf.box_all(
                        self.sdf_rect_pos.x,
                            self.sdf_rect_pos.y,
                            progress_width,
                            self.sdf_rect_size.y,
                            max(self.border_radius.x, 1.0),
                            max(self.border_radius.y, 1.0),
                            max(self.border_radius.z, 1.0),
                            max(self.border_radius.w, 1.0)
                        );
                        sdf.fill_keep(self.get_color());
                    }

                }
                ProgressMode::Vertical => {
                    // - [draw shadow and blur] -----------------------------------------------------------
                    if sdf.shape > -1.0 {
                        if self.spread_radius > 0.0 || self.blur_radius > 0.0 {
                            let shadow_offset = self.shadow_offset + self.rect_shift;
                            let total_shadow_size = self.spread_radius + self.blur_radius;
                            let shadow_lower = vec2(total_shadow_size) + shadow_offset;
                            let shadow_upper = self.rect_size + vec2(self.spread_radius * 2.0) + shadow_offset;
                            if self.border_radius.x != 0.0 || self.border_radius.y != 0.0 ||
                                self.border_radius.z != 0.0 || self.border_radius.w != 0.0 {
                                let max_border_radius = max(
                                    max(
                                        max(self.border_radius.x, self.border_radius.y),
                                        max(self.border_radius.z, self.border_radius.w)
                                    ), 1.0
                                );
                                let v = GaussShadow::rounded_box_shadow(
                                    shadow_lower,
                                    shadow_upper,
                                    self.pos * self.rect_size3,
                                    self.blur_radius,
                                    max_border_radius
                                );
                                let shadow_color = vec4(self.get_shadow_color().rgb, self.get_shadow_color().a * v);
                                sdf.clear(shadow_color);
                            } else {
                                let v = GaussShadow::box_shadow(
                                    shadow_lower,
                                    shadow_upper,
                                    self.pos * self.rect_size3,
                                    self.blur_radius
                                );
                                let shadow_color = vec4(self.get_shadow_color().rgb, self.get_shadow_color().a * v);
                                sdf.clear(shadow_color);
                            }
                        }
                    }

                    // - [basic sdf for draw a view] ------------------------------------------------------
                    let border_width = self.border_width;
                    let total_shadow_size = self.spread_radius + self.blur_radius;
                    let progress_height = self.sdf_rect_size.y * self.value;
                    let wave_offset = self.sdf_rect_size.x * 0.25;
                    // 使用calculated位置而不是原始rect_size
                    sdf.box_all(
                        self.sdf_rect_pos.x + wave_offset,
                        self.sdf_rect_pos.y,
                        self.sdf_rect_size.x * 0.5,
                        self.sdf_rect_size.y - progress_height - wave_offset,
                        max(self.border_radius.x * 0.5, 1.0),
                        max(self.border_radius.y * 0.5, 1.0),
                        max(self.border_radius.z * 0.5, 1.0),
                        max(self.border_radius.w * 0.5, 1.0)
                    );

                    // - [background color if visible] ----------------------------------------------------
                    if self.background_visible == 1.0 {
                        sdf.fill_premul(self.get_background_color());
                    }

                    // - [border with and color if width bigger than 0] -----------------------------------
                    if border_width > 0.0 {
                        sdf.stroke_keep(self.get_border_color(), border_width);
                    }
                    // [draw a small dot in the end of the progress bar] --------------------------
                    let dot_radius = 4.0;
                    let dot_pos = vec2(
                        self.pos.x + self.rect_size.x * 0.5 - self.border_width,
                        self.pos.y + dot_radius * 2.0
                    );
                    sdf.circle(dot_pos.x, dot_pos.y, dot_radius);
                    sdf.fill_premul(self.get_color());

                    // [draw the progress bar] ----------------------------------------------------
                    if self.in_progress == 1.0 {
                        // 纵向波浪进度条参数
                        let center_x = self.pos.x + self.rect_size.x * 0.5;
                        let wave_offset = self.sdf_rect_size.x * 0.25;

                        let y = 0.0;
                        for i in 0..20 {
                            if i < int(self.value * 20.0) {
                                let wave_r = self.sdf_rect_size.x;
                                // 从底部开始向上绘制波浪，类似横向的逻辑
                                let wave_y = (self.sdf_rect_pos.y + self.sdf_rect_size.y - (2.0 * y + 1.0) * wave_r + 2.5 * y * wave_offset) + ((abs(self.value * 10.0 - float(int(self.value * 10.0))) + 1.0) * wave_offset * 0.5);
                                // 检查波浪是否在进度范围内
                                if wave_y < (self.sdf_rect_pos.y + self.sdf_rect_size.y - self.sdf_rect_size.y * self.value) {
                                    break;
                                }

                                if mod(y, 2.0) == 0.0 {
                                    // 左波浪 - 弧线向左
                                    sdf.arc_round_caps(
                                        self.sdf_rect_size.x + wave_offset * 1.0,
                                        wave_y,
                                        wave_r + 0.4,
                                        45.0 * one_deg,
                                        135.0 * one_deg,
                                        wave_offset * 2.0
                                    );
                                } else {
                                    // 右波浪 - 弧线向右
                                    sdf.arc_round_caps(
                                        0.0 - wave_offset * 1.0,
                                        wave_y,
                                        wave_r + 0.4,
                                        225.0 * one_deg,
                                        315.0 * one_deg,
                                        wave_offset * 2.0
                                    );
                                }
                                sdf.fill_keep(self.get_color());
                            }
                            y += 1.0;
                        }
                    } else {
                        let progress_height = self.sdf_rect_size.y * self.value;
                        sdf.box_all(
                            self.sdf_rect_pos.x,
                            self.sdf_rect_pos.y + self.sdf_rect_size.y - progress_height,
                            self.sdf_rect_size.x,
                            progress_height,
                            max(self.border_radius.x, 1.0),
                            max(self.border_radius.y, 1.0),
                            max(self.border_radius.z, 1.0),
                            max(self.border_radius.w, 1.0)
                        );
                        sdf.fill_keep(self.get_color());
                    }
                }
                ProgressMode::Circle => {
                    // [draw a ring progress bar] ------------------------------------------------
                    let center_pos = vec2(self.pos.x + self.rect_size.x * 0.5, self.pos.y + self.rect_size.y * 0.5);
                    let offset = max(
                        max(self.border_radius.x, max(self.border_radius.y, max(self.border_radius.z, self.border_radius.w))),
                    32.0) * 0.25;
                    // 0.8 is a magic offset which can make circle look better
                    let ring_outer_radius = min(self.rect_size.x, self.rect_size.y) * 0.5 - self.border_width * 2.0 - offset ;
                    sdf.circle(center_pos.x, center_pos.y, min(self.rect_size.x, self.rect_size.y) * 0.5);
                    sdf.fill_premul(#f00);
                    let ring_inner_radius = ring_outer_radius - offset * 2.0;
                    let ring_arc_radius = ring_outer_radius - offset;
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
                        let wave_count = 20.0; // 圆环上的波浪数量
                        let wave_amplitude = offset * 2.0; // 波浪振幅
                        let wave_thickness = offset * 2.0; // 波浪线条厚度

                        for i in 0..15 {
                            if i < int(self.value * wave_count) {
                                // 计算每个波浪在圆环上的角度位置
                                // let one_deg = PI / 180.0;
                                let wave_angle = (float(i) / wave_count) * 360.0 * PI / 180.0 + 180.0 * PI / 180.0;

                                // 检查是否在进度范围内
                                if wave_angle <= (self.value * 360.0 + 180.0) * PI / 180.0 {
                                    // 计算波浪中心位置
                                    let wave_center_x = center_pos.x - cos(wave_angle) * ring_arc_radius;
                                    let wave_center_y = center_pos.y + sin(wave_angle) * ring_arc_radius;

                                    if mod(float(i), 2.0) == 0.0 {
                                        // 外波浪 - 向外的弧线
                                        sdf.arc_round_caps(
                                            wave_center_x,
                                            wave_center_y,
                                            wave_amplitude,
                                            wave_angle - 45.0 * one_deg,
                                            wave_angle + 45.0 * one_deg,
                                            wave_thickness
                                        );
                                        // sdf.circle(wave_center_x, wave_center_y, wave_amplitude);
                                    } else {
                                        // 内波浪 - 向内的弧线
                                        // sdf.arc_round_caps(
                                        //     wave_center_x,
                                        //     wave_center_y,
                                        //     wave_amplitude,
                                        //     wave_angle + 135.0 * one_deg,
                                        //     wave_angle + 225.0 * one_deg,
                                        //     wave_thickness
                                        // );
                                        // sdf.circle(wave_center_x, wave_center_y, wave_amplitude);
                                    }
                                    sdf.fill_keep(self.get_color());
                                }
                            }
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
                        sdf.fill_keep(self.get_color());
                    }
                }
            }


            return sdf.result
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawProgress {
    #[deref]
    pub draw_super: DrawView,
    #[live]
    pub mode: ProgressMode,
    #[live]
    pub color: Vec4,
    /// 归一化的进度值，范围0.0到1.0
    #[live]
    pub value: f32,
    /// is in progress state
    /// when in_progress is 1.0, the progress bar should be act as wave
    /// when in_progress is 0.0, the progress bar should be act as normal
    #[live(1.0)]
    pub in_progress: f32,
}

impl DrawProgress {
    pub fn apply_type(&mut self, mode: ProgressMode) {
        self.mode = mode;
    }
    pub fn merge(&mut self, prop: &ProgressBasicStyle) {
        self.background_color = prop.background_color;
        self.border_color = prop.border_color;
        self.border_width = prop.border_width;
        self.border_radius = prop.border_radius.into();
        self.shadow_color = prop.shadow_color.into();
        self.spread_radius = prop.spread_radius;
        self.blur_radius = prop.blur_radius;
        self.shadow_offset = prop.shadow_offset;
        self.background_visible = prop.background_visible.to_f32();
        self.color = prop.color.into();
    }
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }
}
