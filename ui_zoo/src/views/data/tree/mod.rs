use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub TreePage = {{TreePage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fill,
                        width: Fill,
                        flow: Down,
                        spacing: 20.0,
                    }
                }
                <GTree> {
                    body: {
                        <GBranch> {
                            text: {
                                text: "0"
                            }
                            body: {
                                <GLeaf>{
                                    text: <GLabel> {
                                        text: "0-0"
                                    }
                                }
                                <GBranch> {
                                    text: {
                                        text: "0-1"
                                    }
                                    body: {
                                        <GLeaf>{
                                            text: <GLabel> {
                                                text: "0-1-0"
                                            }
                                        }
                                    }
                                }
                                <GBranch> {
                                    text: {
                                        text: "0-2"
                                    }
                                }
                            }
                        }
                        <GBranch> {
                            text: {
                                text: "1"
                            }
                            body: {
                                <GLeaf>{
                                    text: <GLabel> {
                                        text: "1-0"
                                    }
                                }
                                <GBranch> {
                                    text: {
                                        text: "1-1"
                                    }
                                    body: {
                                        <GLeaf>{
                                            text: <GLabel> {
                                                text: "1-1-0"
                                            }
                                        }
                                        <GLeaf>{
                                            text: <GLabel> {
                                                text: "1-1-1"
                                            }
                                        }
                                        <GLeaf>{
                                            text: <GLabel> {
                                                text: "1-1-2"
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
pub struct TreePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TreePage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TreePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for TreePage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(TreePage);
