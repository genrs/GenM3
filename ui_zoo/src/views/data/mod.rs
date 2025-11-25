use makepad_widgets::Cx;

pub mod tag;
pub mod collapse;
pub mod progress;
pub mod slider;
pub mod loading;
pub mod color_picker;
pub mod badge;
pub mod pagination;
pub mod tree;
pub mod number_input;

pub fn register(cx: &mut Cx) {
    tag::live_design(cx);
    collapse::live_design(cx);
    progress::live_design(cx);
    loading::live_design(cx);
    slider::live_design(cx);
    color_picker::live_design(cx);
    badge::live_design(cx);
    pagination::live_design(cx);
    tree::live_design(cx);
    number_input::live_design(cx);
}