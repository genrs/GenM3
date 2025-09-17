use makepad_widgets::*;

use crate::{
    components::ProgressBasicStyle,
    prop::{ProgressMode, traits::ToFloat},
    shader::draw_view::DrawView,
};

live_design! {
    use link::shaders::*;
    DrawProgress = {{DrawProgress}}{
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size3);
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

            // 使用calculated位置而不是原始rect_size
            sdf.box_all(
                self.sdf_rect_pos.x,
                self.sdf_rect_pos.y,
                self.sdf_rect_size.x,
                self.sdf_rect_size.y,
                max(self.border_radius.x, 1.0),
                max(self.border_radius.y, 1.0),
                max(self.border_radius.z, 1.0),
                max(self.border_radius.w, 1.0)
            );

            // - [background color if visible] ----------------------------------------------------
            if self.background_visible == 1.0 {
                sdf.fill_keep(self.get_background_color());
            }

            // - [border with and color if width bigger than 0] -----------------------------------
            if border_width > 0.0 {
                sdf.stroke(self.get_border_color(), border_width);
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
