use makepad_widgets::*;

use crate::{
    basic_prop_interconvert, component_colors, component_state,
    components::{BasicStyle, ComponentState, Style},
    get_get_mut,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR, BORDER_RADIUS, BORDER_WIDTH, COLOR, CURSOR, DISABLED, DRAGGING, FLOW, HEIGHT, HOVER, MARGIN, PADDING, SHADOW_COLOR, SHADOW_OFFSET, SPACING, SPREAD_RADIUS, THEME, WIDTH
        }, traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue}, ApplyStateMapImpl, Radius
    },
    prop_interconvert, state_colors,
    themes::{Color, Theme, TomlValueTo},
};

prop_interconvert! {
    SliderStyle {
        basic_prop = SliderBasicStyle;
        basic => BASIC, SliderBasicStyle::default(),|v| (v, SliderState::Basic).try_into(),
        hover => HOVER, SliderBasicStyle::from_state(Theme::default(), SliderState::Hover),|v| (v, SliderState::Hover).try_into(),
        loading => DRAGGING, SliderBasicStyle::from_state(Theme::default(), SliderState::Dragging),|v| (v, SliderState::Dragging).try_into(),
        disabled => DISABLED, SliderBasicStyle::from_state(Theme::default(), SliderState::Disabled),|v| (v, SliderState::Disabled).try_into()
    }, "[component.Slider] should be a table"
}

impl Style for SliderStyle {
    type State = SliderState;

    type Basic = SliderBasicStyle;

    get_get_mut! {
        SliderState::Basic => basic,
        SliderState::Hover => hover,
        SliderState::Dragging => loading,
        SliderState::Disabled => disabled
    }

    fn len() -> usize {
        3 * SliderBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            SliderState::Basic,
            [
                (SliderState::Hover, &mut self.hover),
                (SliderState::Dragging, &mut self.loading),
                (SliderState::Disabled, &mut self.disabled),
            ],
        );
    }
}

basic_prop_interconvert! {
    SliderBasicStyle {
        state = SliderState;
        {
            background_color => BACKGROUND_COLOR, |v| v.try_into(),
            shadow_color => SHADOW_COLOR, |v| v.try_into(),
            border_color => BORDER_COLOR, |v| v.try_into(),
            color => COLOR, |v| v.try_into()
        };
        {
            background_visible: bool => BACKGROUND_VISIBLE, true, |v| v.to_bool(),
            spread_radius: f32 => SPREAD_RADIUS, 0.0, |v| v.to_f32(),
            blur_radius: f32 => BLUR_RADIUS, 0.0, |v| v.to_f32(),
            shadow_offset: Vec2 => SHADOW_OFFSET, vec2(0.0, 0.0), |v| v.to_vec2(shadow_offset),
            border_width: f32 => BORDER_WIDTH, 0.0, |v| v.to_f32(),
            border_radius: Radius => BORDER_RADIUS, Radius::new(8.0), |v| v.try_into(),
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            padding: Padding => PADDING, Padding::from_xy(10.0, 16.0), |v| v.to_padding(padding),
            flow: Flow => FLOW, Flow::Right, |v| v.to_flow(),
            align: Align => ALIGN, Align::from_f64(0.5), |v| v.to_align(align),
            height: Size => HEIGHT, Size::Fixed(32.0), |v| v.to_size(),
            width: Size => WIDTH, Size::Fill, |v| v.to_size(),
            spacing: f64 => SPACING, 6.0, |v| v.to_f64(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "SliderBasicStyle should be a inline table"
}

component_colors! {
    SliderColors {
        colors = (Color, Color, Color, Color);
        background_color, border_color, shadow_color, color
    }
}

impl BasicStyle for SliderBasicStyle {
    type State = SliderState;

    type Colors = SliderColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let SliderColors {
            background_color,
            border_color,
            shadow_color,
            color,
        } = Self::state_colors(theme, state);

        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };

        Self {
            theme,
            color: color.into(),
            background_color: background_color.into(),
            background_visible: true,
            shadow_color: shadow_color.into(),
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: vec2(0.0, 0.0),
            border_width: 0.0,
            border_color: border_color.into(),
            border_radius: Radius::new(8.0),
            cursor,
            margin: Margin::from_f64(0.0),
            padding: Padding::from_xy(10.0, 16.0),
            flow: Flow::Right,
            align: Align::from_f64(0.5),
            height: Size::Fixed(32.0),
            width: Size::Fill,
            spacing: 6.0,
            abs_pos: None,
        }
    }

    state_colors! {
        (bg_level, border_level, shadow_level, color_level),
        SliderState::Basic => (100, 500, 400, 600),
        SliderState::Hover => (100, 500, 400, 600),
        SliderState::Dragging => (100, 500, 400, 600),
        SliderState::Disabled => (100, 300, 200, 500)
    }

    fn len() -> usize {
        20
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.color.into());
            }
            BACKGROUND_COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(color.background_color.into());
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            SHADOW_COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.shadow_color =
                    Vec4::from_live_color(value).unwrap_or(color.shadow_color.into());
            }
            SPREAD_RADIUS => {
                self.spread_radius = f32::from_live_value(value).unwrap_or(0.0);
            }
            BLUR_RADIUS => {
                self.blur_radius = f32::from_live_value(value).unwrap_or(0.0);
            }
            SHADOW_OFFSET => {
                self.shadow_offset = Vec2::from_live_value(value).unwrap_or(vec2(0.0, 0.0));
            }
            BORDER_WIDTH => {
                self.border_width = f32::from_live_value(value).unwrap_or(0.0);
            }
            BORDER_COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.border_color =
                    Vec4::from_live_color(value).unwrap_or(color.border_color.into());
            }
            BORDER_RADIUS => {
                self.border_radius = Radius::from_live_value(value).unwrap_or(Radius::new(8.0));
            }
            CURSOR => {
                let cursor = if state.is_disabled() {
                    MouseCursor::NotAllowed
                } else {
                    MouseCursor::Hand
                };
                self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
            }
            PADDING => {
                self.padding =
                    Padding::from_live_value(value).unwrap_or(Padding::from_xy(10.0, 16.0));
            }
            FLOW => {
                self.flow = Flow::from_live_value(value).unwrap_or(Flow::Right);
            }
            ALIGN => {
                self.align = Align::from_live_value(value).unwrap_or(Align::from_f64(0.5));
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            SPACING => {
                self.spacing = f64::from_live_value(value).unwrap_or(6.0);
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let SliderColors {
            background_color,
            border_color,
            shadow_color,
            color,
        } = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
        self.border_color = border_color.into();
        self.shadow_color = shadow_color.into();
        self.color = color.into();
    }

    fn live_props() -> crate::components::LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(color), None.into()),
            (live_id!(background_color), None.into()),
            (live_id!(border_color), None.into()),
            (
                live_id!(border_radius),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (live_id!(border_width), None.into()),
            (live_id!(shadow_color), None.into()),
            (live_id!(spread_radius), None.into()),
            (live_id!(blur_radius), None.into()),
            (live_id!(shadow_offset), None.into()),
            (live_id!(background_visible), None.into()),
            (live_id!(cursor), None.into()),
            (live_id!(width), None.into()),
            (live_id!(height), None.into()),
            (
                live_id!(margin),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (
                live_id!(padding),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (live_id!(align), Some(vec![live_id!(x), live_id!(y)]).into()),
            (live_id!(flow), None.into()),
            (live_id!(spacing), None.into()),
            (live_id!(abs_pos), None.into()),
        ]
    }

    fn walk(&self) -> Walk {
        Walk {
            abs_pos: self.abs_pos,
            margin: self.margin,
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self) -> Layout {
        Layout {
            clip_x: false,
            clip_y: false,
            padding: self.padding,
            align: self.align,
            flow: self.flow,
            spacing: self.spacing,
            ..Default::default()
        }
    }
}

component_state! {
    SliderState {
        Basic => BASIC,
        Hover => HOVER,
        Dragging => DRAGGING,
        Disabled => DISABLED
    },
    _ => SliderState::Basic
}

impl ComponentState for SliderState {
    fn is_disabled(&self) -> bool {
        matches!(self, SliderState::Disabled)
    }
}
