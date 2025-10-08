use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    DrawCursor = {{DrawCursor}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(
                0.0,
                0.0,
                self.rect_size.x,
                self.rect_size.y,
                self.border_radius
            );
            sdf.fill(mix(vec4(0.0), self.color, self.blink));
            return sdf.result;
        }
    }

    DrawSelection = {{DrawSelection}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(
                0.0,
                0.0,
                self.rect_size.x,
                self.rect_size.y,
                self.border_radius
            );
            sdf.fill(vec4(self.color.x, self.color.y, self.color.z, 0.6));
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawCursor {
    #[deref]
    pub draw_super: DrawQuad,
    #[live(vec4(1.0, 1.0, 0.0, 1.0))]
    pub color: Vec4,
    #[live(0.5)]
    pub border_radius: f32,
    #[live]
    pub blink: f32,
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawSelection {
    #[deref]
    pub draw_super: DrawQuad,
    #[live(vec4(1.0, 1.0, 0.0, 1.0))]
    pub color: Vec4,
    #[live(0.5)]
    pub border_radius: f32,
}
