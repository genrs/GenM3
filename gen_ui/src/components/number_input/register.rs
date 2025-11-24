use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::number_input::controller::live_design(cx);
    crate::components::number_input::live_design(cx);
}