use crate::{
    basic_prop_interconvert, component_state,
    components::{BasicStyle, ComponentState, Style},
    get_get_mut,
    prop::{
        ApplyStateMapImpl,
        manuel::{
            ABS_POS, ALIGN, BASIC, CURSOR, DISABLED, FLOW, HEIGHT, HOVER, MARGIN, PADDING, PRESSED,
            SPACING, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveValue, NewFrom, ToTomlValue},
    },
    prop_interconvert,
    themes::{Theme, TomlValueTo},
};
use makepad_widgets::*;

prop_interconvert! {
    ColorPanelStyle {
        basic_prop = ColorPanelBasicStyle;
        basic => BASIC, ColorPanelBasicStyle::default(),|v| (v, ColorPanelState::Basic).try_into(),
        hover => HOVER, ColorPanelBasicStyle::from_state(Theme::default(), ColorPanelState::Hover),|v| (v, ColorPanelState::Hover).try_into(),
        pressed => PRESSED, ColorPanelBasicStyle::from_state(Theme::default(), ColorPanelState::Pressed),|v| (v, ColorPanelState::Pressed).try_into(),
        disabled => DISABLED, ColorPanelBasicStyle::from_state(Theme::default(), ColorPanelState::Disabled),|v| (v, ColorPanelState::Disabled).try_into()
    }, "[component.color_panel] should be a table"
}

impl Style for ColorPanelStyle {
    type State = ColorPanelState;

    type Basic = ColorPanelBasicStyle;

    get_get_mut! {
        ColorPanelState::Basic => basic,
        ColorPanelState::Hover => hover,
        ColorPanelState::Pressed => pressed,
        ColorPanelState::Disabled => disabled
    }

    fn len() -> usize {
        4 * ColorPanelBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            ColorPanelState::Basic,
            [
                (ColorPanelState::Hover, &mut self.hover),
                (ColorPanelState::Pressed, &mut self.pressed),
                (ColorPanelState::Disabled, &mut self.disabled),
            ],
        );
    }
}

basic_prop_interconvert! {
    ColorPanelBasicStyle {
        state = ColorPanelState;
        {};
        {
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            padding: Padding => PADDING, Padding::from_xy(9.0, 16.0), |v| v.to_padding(padding),
            flow: Flow => FLOW, Flow::Right, |v| v.to_flow(),
            align: Align => ALIGN, Align::from_f64(0.5), |v| v.to_align(align),
            height: Size => HEIGHT, Size::Fit, |v| v.to_size(),
            width: Size => WIDTH, Size::Fit, |v| v.to_size(),
            spacing: f64 => SPACING, 6.0, |v| v.to_f64(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "ColorPanelBasicStyle should be a inline table"
}

impl BasicStyle for ColorPanelBasicStyle {
    type State = ColorPanelState;

    type Colors = ();

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };

        Self {
            theme,
            cursor,
            margin: Margin::from_f64(0.0),
            padding: Padding::from_f64(0.0),
            flow: Flow::Right,
            align: Align::from_f64(0.5),
            height: Size::Fixed(160.0),
            width: Size::Fixed(220.0),
            spacing: 0.0,
            abs_pos: None,
        }
    }

    fn state_colors(_theme: crate::themes::Theme, _state: Self::State) -> Self::Colors {
        ()
    }

    fn len() -> usize {
        11
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
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
                self.padding = Padding::from_live_value(value).unwrap_or(Padding::from_f64(0.0));
            }
            FLOW => {
                self.flow = Flow::from_live_value(value).unwrap_or(Flow::Right);
            }
            ALIGN => {
                self.align = Align::from_live_value(value).unwrap_or(Align::from_f64(0.5));
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fixed(160.0));
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fixed(220.0));
            }
            SPACING => {
                self.spacing = f64::from_live_value(value).unwrap_or(0.0);
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            _ => {}
        }
    }

    fn sync(&mut self, _state: Self::State) -> () {
        ()
    }

    fn live_props() -> crate::components::LiveProps {
        vec![
            (live_id!(theme), None.into()),
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
    ColorPanelState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    },
    _ => ColorPanelState::Basic
}

impl ComponentState for ColorPanelState {
    fn is_disabled(&self) -> bool {
        matches!(self, ColorPanelState::Disabled)
    }
}
