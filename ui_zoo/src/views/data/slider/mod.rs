use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use link::shaders::*;
    use crate::views::cbox::*;

    pub SliderPage = {{SliderPage}} {
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
                            height: 248.0,
                            width: Fill,
                            spacing: 12.0,
                        }
                    }
                    <GSlider>{
                        value: 32.0,
                        min: 0.0,
                        max: 100.0,
                        step: 2.0
                    }
                    <GSlider> {
                        draw_slider: {
                            // 设置红橙黄绿青蓝紫的渐变色
                            fn get_background_color(self) -> vec4 {
                                // 使用HSV颜色空间实现彩虹渐变
                                // 色相从0到300度(避免回到红色)
                                let hue = self.pos.x * 300.0;
                                let saturation = 1.0;
                                let value = 1.0;
                                
                                // HSV转RGB算法
                                let c = value * saturation;
                                let x = c * (1.0 - abs(mod(hue / 60.0, 2.0) - 1.0));
                                let m = value - c;
                                
                                let rgb = vec3(0.0);
                                if hue < 60.0 {
                                    rgb = vec3(c, x, 0.0);
                                } else if hue < 120.0 {
                                    rgb = vec3(x, c, 0.0);
                                } else if hue < 180.0 {
                                    rgb = vec3(0.0, c, x);
                                } else if hue < 240.0 {
                                    rgb = vec3(0.0, x, c);
                                } else if hue < 300.0 {
                                    rgb = vec3(x, 0.0, c);
                                } else {
                                    rgb = vec3(c, 0.0, x);
                                }
                                
                                return vec4(rgb.x + m, rgb.y + m, rgb.z + m, 1.0);
                            }
                            fn get_color(self) -> vec4 {
                                // 滑块颜色为白色
                                return self.get_background_color();
                            }
                        }
                        value: 60.0,
                        min: 0.0,
                        max: 100.0,
                    }  
                    
                }
                <GHLayout> {
                    style: {
                        basic: {
                            height: 300.0,
                            width: Fill,
                            spacing: 12.0,
                        }
                    }
                    <GSlider>{
                        style: {
                            basic: {theme: Primary, height: Fill, width: 64.0}
                        },
                        value: 32.0,
                        min: 0.0,
                        max: 100.0,
                        step: 1.0,
                        proportion: 0.9,
                        mode: Vertical,
                    }
                    <GSlider> {
                        style: {
                            basic: {theme: Primary, height: 164.0, width: 164.0}
                        }
                        value: 33.,
                        min: 0.,
                        max: 100.,
                        step: 0.5,
                        proportion: 0.48, // less than 0.66 display better
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
pub struct SliderPage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    timer: Timer,
}

impl LiveHook for SliderPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for SliderPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for SliderPage {
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
            progress.play_animation(cx, id!(in_progress.on));
            progress.redraw(cx);
        }
    }
}

widget_node!(SliderPage);
