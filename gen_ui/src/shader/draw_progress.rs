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
                    let spacing = self.sdf_rect_size.y * 0.2;
                    if self.value >= 1.0 {
                        spacing = 0.0;
                    }
                    let total_shadow_size = self.spread_radius + self.blur_radius;
                    let progress_width = self.sdf_rect_size.x * self.value;
                    let border_radius = self.count_border_radius();
                    // 使用calculated位置而不是原始rect_size
                    sdf.box_all(
                        self.sdf_rect_pos.x + progress_width,
                        self.sdf_rect_pos.y,
                        self.sdf_rect_size.x - progress_width,
                        self.sdf_rect_size.y,
                        border_radius.x,
                        border_radius.y,
                        border_radius.z,
                        border_radius.w
                    );
                    // - [border with and color if width bigger than 0] -----------------------------------
                    if self.border_width != 0.0 {
                        sdf.stroke_keep(self.get_border_color(), self.border_width);
                    }
                    // - [background color if visible] ----------------------------------------------------
                    if self.background_visible == 1.0 {
                        sdf.fill_premul(self.get_background_color());
                    }

                    // [draw a small dot in the end of the progress bar] --------------------------
                    let dot_radius = 4.0;
                    let dot_pos = vec2(
                        self.rect_size.x + self.pos.x - dot_radius * 2.0 - self.border_width,
                        self.pos.y + self.rect_size.y * 0.5
                    );
                    sdf.circle(dot_pos.x, dot_pos.y, dot_radius);
                    sdf.fill_premul(self.get_color());

                    // // [draw the progress bar] ----------------------------------------------------
                    if self.value != 0.0 {
                        sdf.box_all(
                            self.sdf_rect_pos.x,
                            self.sdf_rect_pos.y,
                            progress_width - spacing,
                            self.sdf_rect_size.y,
                            border_radius.x,
                            border_radius.y,
                            border_radius.z,
                            border_radius.w
                        );
                        if self.border_width != 0.0 {
                            sdf.stroke_keep(self.get_border_color(), self.border_width);
                        }
                        sdf.fill(self.get_color());
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
                    let spacing = self.sdf_rect_size.x * 0.2;
                    if self.value >= 1.0 {
                        spacing = 0.0;
                    }
                    let total_shadow_size = self.spread_radius + self.blur_radius;
                    let progress_height = self.sdf_rect_size.y * self.value;
                    let border_radius = self.count_border_radius();
                    // 使用calculated位置而不是原始rect_size
                    sdf.box_all(
                        self.sdf_rect_pos.x,
                        self.sdf_rect_pos.y,
                        self.sdf_rect_size.x,
                        self.sdf_rect_size.y - progress_height,
                        border_radius.x,
                        border_radius.y,
                        border_radius.z,
                        border_radius.w
                    );
                    // - [border with and color if width bigger than 0] -----------------------------------
                    if self.border_width != 0.0 {
                        sdf.stroke_keep(self.get_border_color(), self.border_width);
                    }

                    // - [background color if visible] ----------------------------------------------------
                    // keep the mask even if background invisible so stroke can draw
                    if self.background_visible == 1.0 {
                        sdf.fill_premul(self.get_background_color());
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
                    if self.value != 0.0 {
                        let progress_height = self.sdf_rect_size.y * self.value;
                        sdf.box_all(
                            self.sdf_rect_pos.x,
                            self.sdf_rect_size.y - progress_height + spacing,
                            self.sdf_rect_size.x,
                            progress_height - spacing,
                            border_radius.x,
                            border_radius.y,
                            border_radius.z,
                            border_radius.w
                        );
                        if self.border_width != 0.0 {
                            sdf.stroke_keep(self.get_border_color(), self.border_width);
                        }
                        sdf.fill(self.get_color());
                    }
                }
                ProgressMode::Circle => {
                    // [draw a ring progress bar] ------------------------------------------------
                    let center_pos = vec2(self.pos.x + self.rect_size.x * 0.5, self.pos.y + self.rect_size.y * 0.5);
                    // compute offset from border_radius (use count_border_radius to account for border)
                    let border_radius = self.count_border_radius();
                    let max_border = max(
                        max(border_radius.x, border_radius.y),
                        max(border_radius.z, border_radius.w)
                    );
                    let offset = max(max_border, min(self.sdf_rect_size.x, self.sdf_rect_size.y) * 0.2) * 0.5;
                    let ring_outer_radius = min(self.rect_size.x, self.rect_size.y) * 0.5;

                    let ring_inner_radius = ring_outer_radius - offset * 2.0;
                    let ring_arc_radius = ring_inner_radius + offset;
                    // sdf.circle(center_pos.x, center_pos.y, ring_outer_radius);
                    // sdf.circle(center_pos.x, center_pos.y, ring_inner_radius);
                    let start_deg = (self.value * PI) * one_deg;
                    if self.value != 1.0 {
                        sdf.arc_round_caps(
                            center_pos.x,
                            center_pos.y,
                            ring_arc_radius,
                            (self.value * 360.0 - 162.0) * one_deg,
                            162.0 * one_deg,
                            offset * 2.0
                        );
                    }

                    if self.border_width > 0.0 {
                        sdf.stroke_keep(self.get_border_color(), self.border_width);
                    }

                    // keep mask even when background invisible so stroke can draw
                    if self.background_visible == 1.0 {
                        sdf.fill_premul(self.get_background_color());
                    }

                    
                    // [draw the progress part] ----------------------------------------------------
                    if self.value == 1.0 {
                        sdf.arc_round_caps(
                            center_pos.x,
                            center_pos.y,
                            ring_arc_radius,
                            0.0 * one_deg,
                            360.0 * one_deg,
                            offset * 2.0
                        );
                    } else {
                        sdf.arc_round_caps(
                            center_pos.x,
                            center_pos.y,
                            ring_arc_radius,
                            198.0 * one_deg,
                            (self.value * 360.0 + 162.0) * one_deg,
                            offset * 2.0
                        );
                    }
                    sdf.fill_premul(self.get_color());
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
