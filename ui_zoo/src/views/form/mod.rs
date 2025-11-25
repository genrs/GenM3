use makepad_widgets::Cx;

pub mod radio;
pub mod checkbox;
pub mod switch;
pub mod rate;
pub mod input;
pub mod select;
pub mod verification;

pub fn register(cx: &mut Cx) {
    radio::live_design(cx);
    checkbox::live_design(cx);
    switch::live_design(cx);
    rate::live_design(cx);
    input::live_design(cx);
    select::live_design(cx);
    verification::live_design(cx);
}