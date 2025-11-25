use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub ProgressPage = {{ProgressPage}} {
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
                <GVLayout> {
                    style: {
                        basic: {
                            height: 600.0,
                            width: Fill,
                            spacing: 12.0,
                        }
                    }
                    <GProgress> {
                        style: {
                            basic: {theme: Primary, border_width: 2.0, border_color: #FF0000}
                        }
                        value: 80.0,
                        min: 0.,
                        max: 100.,
                    }
                    <GHLayout> {
                        style: {
                            basic: {
                                height: 300.0,
                                width: Fill,
                                spacing: 12.0,
                            }
                        }
                        <GProgress> {
                            style: {
                                basic: {theme: Primary, height: Fill, width: 32.0}
                            }
                            value: 10.0,
                            min: 0.,
                            max: 100.,
                            mode: Vertical,
                        }
                        <GProgress> {
                            style: {
                                basic: {theme: Primary, height: Fill, width: 16.0}
                            }
                            value: 50.0,
                            min: 0.,
                            max: 100.,
                            mode: Vertical,
                        }
                    }
                    <GProgress> {
                        style: {
                            basic: {theme: Primary, height: 148.0, width: 148.0}
                        }
                        value: 3.,
                        min: 0.,
                        max: 100.,
                        mode: Circle,
                    }
                    rprogress = <GProgress> {
                        style: {
                            basic: {theme: Primary, height: 48.0, width: 48.0}
                        }
                        value: 64.,
                        min: 0.,
                        max: 100.,
                        mode: Circle,
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
pub struct ProgressPage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    timer: Timer,
}

impl LiveHook for ProgressPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ProgressPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for ProgressPage {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.timer = cx.start_interval(1.0);
    }

    fn handle_timer(&mut self, cx: &mut Cx, _e: &TimerEvent) {
        // dbg!("tick");
        if let Some(mut progress) = self.gprogress(id!(rprogress)).borrow_mut() {
            if progress.value <= 100.0 {
                progress.value += 5.0;
            } else {
                progress.value = 0.0;
            }
            progress.play_animation(cx, id!(loading.on));
            progress.redraw(cx);
        }
    }
}

widget_node!(ProgressPage);
