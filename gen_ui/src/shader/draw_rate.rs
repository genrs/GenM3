use makepad_widgets::*;

use crate::components::RateBasicStyle;

live_design! {
    use link::shaders::*;
    DrawRate = {{DrawRate}}{

        fn get_color(self) -> vec4 { return self.color; }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);

            // Calculate star size and spacing
            let available_width = self.rect_size.x - self.spacing * (self.count - 1.0);
            let star_size = available_width / self.count;
            let star_radius = min(star_size * 0.4, self.rect_size.y * 0.4);
            let inner_radius = star_radius * 0.5;

            // Calculate starting position to center the stars
            let total_width = self.count * star_size + self.spacing * (self.count - 1.0);
            let start_x = (self.rect_size.x - total_width) * 0.5 + star_size * 0.5;
            let center_y = self.rect_size.y * 0.5;

            let angle_outer = -PI * 0.5; // Start from top
            let angle_step = PI * 0.4; // 72 degrees between outer points

            // Draw multiple stars - each star independently to avoid accumulation
            for i in 0..5 {
                let star_center_x = start_x + float(i) * (star_size + self.spacing);
                let star_center = vec2(star_center_x, center_y);

                // Calculate all 10 points of the star
                let p0 = star_center + vec2(cos(angle_outer) * star_radius, sin(angle_outer) * star_radius);
                let p1 = star_center + vec2(cos(angle_outer + angle_step * 0.5) * inner_radius, sin(angle_outer + angle_step * 0.5) * inner_radius);
                let p2 = star_center + vec2(cos(angle_outer + angle_step) * star_radius, sin(angle_outer + angle_step) * star_radius);
                let p3 = star_center + vec2(cos(angle_outer + angle_step * 1.5) * inner_radius, sin(angle_outer + angle_step * 1.5) * inner_radius);
                let p4 = star_center + vec2(cos(angle_outer + angle_step * 2.0) * star_radius, sin(angle_outer + angle_step * 2.0) * star_radius);
                let p5 = star_center + vec2(cos(angle_outer + angle_step * 2.5) * inner_radius, sin(angle_outer + angle_step * 2.5) * inner_radius);
                let p6 = star_center + vec2(cos(angle_outer + angle_step * 3.0) * star_radius, sin(angle_outer + angle_step * 3.0) * star_radius);
                let p7 = star_center + vec2(cos(angle_outer + angle_step * 3.5) * inner_radius, sin(angle_outer + angle_step * 3.5) * inner_radius);
                let p8 = star_center + vec2(cos(angle_outer + angle_step * 4.0) * star_radius, sin(angle_outer + angle_step * 4.0) * star_radius);
                let p9 = star_center + vec2(cos(angle_outer + angle_step * 4.5) * inner_radius, sin(angle_outer + angle_step * 4.5) * inner_radius);

                // Create a fresh SDF path for each star to avoid accumulation effects
                sdf.move_to(p0.x, p0.y);
                sdf.line_to(p1.x, p1.y);
                sdf.line_to(p2.x, p2.y);
                sdf.line_to(p3.x, p3.y);
                sdf.line_to(p4.x, p4.y);
                sdf.line_to(p5.x, p5.y);
                sdf.line_to(p6.x, p6.y);
                sdf.line_to(p7.x, p7.y);
                sdf.line_to(p8.x, p8.y);
                sdf.line_to(p9.x, p9.y);
                sdf.close_path();

                // Use stroke (without _keep) to render this star independently
                sdf.stroke(self.color, 1.0);
            }

            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawRate {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live]
    pub spacing: f32,
    #[live(5.0)]
    pub count: f32,
}

impl DrawRate {
    pub fn merge(&mut self, prop: &RateBasicStyle) {
        self.color = prop.color;
        self.spacing = prop.spacing as f32;
    }
}
