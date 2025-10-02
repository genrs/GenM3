use makepad_widgets::*;

use crate::{
    basic_prop_interconvert, component_color, component_state, components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
    }, get_get_mut, prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BASIC, COLOR, CURSOR, DISABLED, FLOW, HEIGHT, HOVER,
            MARGIN, PADDING, PRESSED, SPACING, THEME, WIDTH,
        }, traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue}, ApplyStateMapImpl
    }, prop_interconvert, state_color,  themes::{Theme, TomlValueTo}
};

prop_interconvert! {
    RateStyle {
        basic_prop = RateBasicStyle;
        basic => BASIC, RateBasicStyle::default(),|v| (v, RateState::Basic).try_into(),
        hover => HOVER, RateBasicStyle::from_state(Theme::default(), RateState::Hover),|v| (v, RateState::Hover).try_into(),
        pressed => PRESSED, RateBasicStyle::from_state(Theme::default(), RateState::Pressed),|v| (v, RateState::Pressed).try_into(),
        disabled => DISABLED, RateBasicStyle::from_state(Theme::default(), RateState::Disabled),|v| (v, RateState::Disabled).try_into()
    }, "[component.rate] should be a table"
}

impl Style for RateStyle {
    type State = RateState;

    type Basic = RateBasicStyle;

    fn len() -> usize {
        RateBasicStyle::len() * 4 // basic, hover, pressed, disabled
    }

    get_get_mut! {
        RateState::Basic => basic,
        RateState::Hover => hover,
        RateState::Pressed => pressed,
        RateState::Disabled => disabled
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            RateState::Basic,
            [
                (RateState::Hover, &mut self.hover),
                (RateState::Pressed, &mut self.pressed),
                (RateState::Disabled, &mut self.disabled),
            ],
        );
    }
}

basic_prop_interconvert! {
    RateBasicStyle {
        state = RateState;
        {
            color => COLOR, |v| v.try_into()
        };
        {
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            padding: Padding => PADDING, Padding::from_f64(0.0), |v| v.to_padding(padding),
            flow: Flow => FLOW, Flow::Right, |v| v.to_flow(),
            align: Align => ALIGN, Align::from_f64(0.5), |v| v.to_align(align),
            height: Size => HEIGHT, Size::Fixed(18.0), |v| v.to_size(),
            width: Size => WIDTH, Size::Fit, |v| v.to_size(),
            spacing: f64 => SPACING, 4.0, |v| v.to_f64(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "RateBasicStyle should be a inline table"
}

component_color! {
    RateColors {
        colors = (Color);
        color
    }
}

impl BasicStyle for RateBasicStyle {
    type State = RateState;

    type Colors = RateColors;

    fn len() -> usize {
        11
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            BACKGROUND_COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.color.into());
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
                    Padding::from_live_value(value).unwrap_or(Padding::from_f64(0.0));
            }
            FLOW => {
                self.flow = Flow::from_live_value(value).unwrap_or(Flow::Right);
            }
            ALIGN => {
                self.align = Align::from_live_value(value).unwrap_or(Align::from_f64(0.5));
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fixed(18.0));
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            SPACING => {
                self.spacing = f64::from_live_value(value).unwrap_or(4.0);
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let RateColors { color } = Self::state_colors(self.theme, state);
        self.color = color.into();
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let RateColors { color } = Self::state_colors(theme, state);

        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };

        Self {
            theme,
            color: color.into(),
            cursor,
            margin: Margin::from_f64(0.0),
            padding: Padding::from_f64(0.0),
            flow: Flow::Right,
            align: Align::from_f64(0.5),
            height: Size::Fixed(18.0),
            width: Size::Fit,
            spacing: 4.0,
            abs_pos: None,
        }
    }

    state_color! {
        (color_level),
        RateState::Basic => (500),
        RateState::Hover => (400),
        RateState::Pressed => (500),
        RateState::Disabled => (400)
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(color), None.into()),
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
    RateState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    },
    _ => RateState::Basic
}

impl ComponentState for RateState {
    fn is_disabled(&self) -> bool {
        matches!(self, RateState::Disabled)
    }
}
