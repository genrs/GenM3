use makepad_widgets::*;

use crate::{
    basic_prop_interconvert, component_color, component_state,
    components::{BasicStyle, ComponentState, Style},
    get_get_mut,
    prop::{
        ApplyStateMapImpl,
        manuel::{
            ABS_POS, ALIGN, BASIC, COLOR, CURSOR, DISABLED, FLOW, HEIGHT, LOADING, MARGIN, PADDING,
            SPACING, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
    },
    prop_interconvert, state_color,
    themes::{Theme, TomlValueTo},
};

prop_interconvert! {
    LoadingStyle {
        basic_prop = LoadingBasicStyle;
        basic => BASIC, LoadingBasicStyle::default(),|v| (v, LoadingState::Basic).try_into(),
        loading => LOADING, LoadingBasicStyle::from_state(Theme::default(), LoadingState::Loading),|v| (v, LoadingState::Loading).try_into(),
        disabled => DISABLED, LoadingBasicStyle::from_state(Theme::default(), LoadingState::Disabled),|v| (v, LoadingState::Disabled).try_into()
    }, "[component.Loading] should be a table"
}

impl Style for LoadingStyle {
    type State = LoadingState;

    type Basic = LoadingBasicStyle;

    get_get_mut! {
        LoadingState::Basic => basic,
        LoadingState::Loading => loading,
        LoadingState::Disabled => disabled
    }

    fn len() -> usize {
        3 * LoadingBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            LoadingState::Basic,
            [
                (LoadingState::Loading, &mut self.loading),
                (LoadingState::Disabled, &mut self.disabled),
            ],
        );
    }
}

basic_prop_interconvert! {
    LoadingBasicStyle {
        state = LoadingState;
        {
            color => COLOR, |v| v.try_into()
        };
        {
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            padding: Padding => PADDING, Padding::from_xy(10.0, 16.0), |v| v.to_padding(padding),
            flow: Flow => FLOW, Flow::Right, |v| v.to_flow(),
            align: Align => ALIGN, Align::from_f64(0.5), |v| v.to_align(align),
            height: Size => HEIGHT, Size::Fixed(32.0), |v| v.to_size(),
            width: Size => WIDTH, Size::Fixed(32.0), |v| v.to_size(),
            spacing: f64 => SPACING, 6.0, |v| v.to_f64(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "LoadingBasicStyle should be a inline table"
}

component_color! {
    LoadingColors {
        colors = (Color);
        color
    }
}

impl BasicStyle for LoadingBasicStyle {
    type State = LoadingState;

    type Colors = LoadingColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let LoadingColors { color } = Self::state_colors(theme, state);

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
            padding: Padding::from_xy(10.0, 16.0),
            flow: Flow::Right,
            align: Align::from_f64(0.5),
            height: Size::Fixed(32.0),
            width: Size::Fixed(32.0),
            spacing: 6.0,
            abs_pos: None,
        }
    }

    state_color! {
        (color),
        LoadingState::Basic => (600),
        LoadingState::Loading => (600),
        LoadingState::Disabled => (500)
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
            COLOR => {
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
        let LoadingColors { color } = Self::state_colors(self.theme, state);

        self.color = color.into();
    }

    fn live_props() -> crate::components::LiveProps {
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
    LoadingState {
        Basic => BASIC,
        Loading => LOADING,
        Disabled => DISABLED
    },
    _ => LoadingState::Basic
}

impl ComponentState for LoadingState {
    fn is_disabled(&self) -> bool {
        matches!(self, LoadingState::Disabled)
    }
}
