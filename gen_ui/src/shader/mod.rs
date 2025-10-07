pub mod animation;
pub mod draw_checkbox;
pub mod draw_color_picker;
pub mod draw_image;
pub mod draw_link;
pub mod draw_loading;
pub mod draw_popup;
pub mod draw_progress;
pub mod draw_radio;
pub mod draw_slider;
pub mod draw_svg;
pub mod draw_switch;
pub mod draw_view;
pub mod draw_rate;
pub mod draw_dot;
pub mod draw_input;

use makepad_widgets::Cx;

pub fn shader_register(cx: &mut Cx) {
    draw_view::live_design(cx);
    draw_radio::live_design(cx);
    draw_checkbox::live_design(cx);
    draw_switch::live_design(cx);
    draw_svg::live_design(cx);
    draw_image::live_design(cx);
    draw_popup::live_design(cx);
    draw_link::live_design(cx);
    draw_progress::live_design(cx);
    draw_slider::live_design(cx);
    draw_color_picker::live_design(cx);
    draw_loading::live_design(cx);
    draw_rate::live_design(cx);
    draw_dot::live_design(cx);
    draw_input::live_design(cx);
    animation::live_design(cx);
}
