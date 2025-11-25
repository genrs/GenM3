use gen_ui::{components::*, inherits_view_livehook, prop::traits::ToFloat};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub PaginationPage = {{PaginationPage}} {
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
                <GPagination>{
                    style: {
                        basic: {
                            prefix: {
                                background_color: #FF0000,
                            }
                            suffix: {
                                background_color: #00FF00,
                            }
                            item: {
                                theme: Primary
                            }
                        }
                    }
                    current: 2,
                    total: 4,
                }
                <GPagination>{
                    current: 5,
                    total: 5,
                }
                <GPagination>{
                    current: 5,
                    total: 7,
                }
                <GPagination>{
                    current: 6,
                    total: 10,
                }
            }
            desc = {
                text: ""
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct PaginationPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for PaginationPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for PaginationPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for PaginationPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {}
}

widget_node!(PaginationPage);
