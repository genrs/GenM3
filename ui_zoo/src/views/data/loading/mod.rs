use gen_ui::{components::*, inherits_view_livehook, prop::traits::ToFloat};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub LoadingPage = {{LoadingPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                        spacing: 20.0,
                    }
                }
                <GHLayout> {
                    style: {
                        basic: {
                            height: 248.0,
                            width: Fill,
                            spacing: 12.0,
                        }
                    }
                    gloading1 = <GLoading> {mode: Circle, style: {basic: { height: 60.0, width: 60.0}}}
                    gloading2 = <GLoading> {mode: Dot, style: {basic: { height: 60.0, width: 60.0}}}
                    gloading3 = <GLoading> {
                        mode: Polygons,
                        loading: false,
                        style: {basic: { height: 60.0, width: 60.0}}
                    }

                    open_btn = <GButton> {

                    }
                }
            }
            desc = {
                text: ""
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct LoadingPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LoadingPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LoadingPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for LoadingPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut loading1 = self.gloading(id!(gloading1));
        let mut loading2 = self.gloading(id!(gloading2));
        let mut loading3 = self.gloading(id!(gloading3));
        let open_btn = self.gbutton(id!(open_btn));
        if let Some(_) = open_btn.clicked(actions) {
            for loading in [&mut loading1, &mut loading2, &mut loading3] {
                loading.toggle(cx);
            }
        }
    }
}

widget_node!(LoadingPage);
