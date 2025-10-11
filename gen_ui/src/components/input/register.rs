use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::input::area::live_design(cx);
    crate::components::input::live_design(cx);
}