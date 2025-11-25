use std::env::current_dir;

use gen_ui::{
    components::*,
    prop::traits::{ToColor, ToFloat},
    themes::Theme,
};
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::genui::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::an::*;
    use crate::views::home::*;
    use crate::views::test_home::*;
    use crate::views::basic::view::*;

    Post = <View> {
        width: Fill, height: Fit,
        padding: { top: 10., bottom: 10.}

        body = <RoundedView> {
            width: Fill, height: Fit
            content = <View> {
                width: Fill, height: Fit
                text = <P> { text: "" }
            }
        }
    }

    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                body = <View>{
                    // flow: Down,
                    flow: Right,
                    spacing:30,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    show_bg: true,
                    draw_bg: {
                        color: #140D2A,
                    }
                    padding: {left: 0., right: 0., top: 0., bottom: 0.}
                    // padding: {left: 10., right: 10., top: 0., bottom: 0.}
                    body = <HomePage> {}

                    // <GColorPanel>{
                    //     color: #348b8f
                    // } 
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    counter: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::an::live_design(cx);
        crate::gen_ui::live_design(cx, Option::<&str>::None);
        // crate::gen_ui::live_design(cx, Some(current_dir().unwrap()));
        crate::views::register(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {}
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::XrUpdate(_e) = event {
            //log!("{:?}", e.now.left.trigger.analog);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
