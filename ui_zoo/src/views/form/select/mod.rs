use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub SelectPage = {{SelectPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                <GSelect> {
                    style: {
                        basic: {
                            container: {
                                theme: Primary
                            }
                        }
                    }
                    // open: false
                    select_options: {
                        style: {
                            basic: {
                                theme: Primary
                            }
                        }
                        <GSelectItem> {
                            style: {
                                basic: {
                                    container: {
                                        theme: Primary
                                    }
                                }
                            }
                            text: {
                                text: "Option 1"
                            }
                        }
                        <GSelectItem> {style: {
                                basic: {
                                    container: {
                                        theme: Primary
                                    }
                                }
                            },active: true, text: {text: "Option 2"} }
                    }
                    suffix: <GView>{
                        <IconRight>{
                            style: {
                                basic: {
                                    svg: {
                                        height: 16.0,
                                        width: 16.0,
                                    }
                                }
                            }
                        }
                    }
                    item: <GSelectItem> {}
                }

            }
            desc = {
                text: ""
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct SelectPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for SelectPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for SelectPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for SelectPage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(SelectPage);
