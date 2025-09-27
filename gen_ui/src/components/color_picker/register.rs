use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::color_picker::panel::live_design(cx);
}
