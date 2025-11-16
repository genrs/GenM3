use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::tree::leaf::live_design(cx);
    crate::components::tree::branch::live_design(cx);
}