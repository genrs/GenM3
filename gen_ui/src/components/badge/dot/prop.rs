use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    component_part, component_state,
    components::{
        label::{LabelBasicStyle, LabelState},
        live_props::LiveProps,
        svg::SvgBasicStyle,
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, Radius,
        manuel::{BASIC, CONTAINER, DISABLED, TEXT},
        traits::NewFrom,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    BadgeDotStyle {
        basic_prop = BadgeDotBasicStyle;
        basic => BASIC, BadgeDotBasicStyle::default(), |v| (v, BadgeDotState::Basic).try_into(),
        disabled => DISABLED, BadgeDotBasicStyle::from_state(Theme::default(), BadgeDotState::Disabled), |v| (v, BadgeDotState::Disabled).try_into()
    }, "[component.badge.dot] should be a table"
}

impl SlotStyle for BadgeDotStyle {
    type Part = BadgeDotPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            BadgeDotState::Basic,
            [(BadgeDotState::Disabled, &mut self.disabled)],
            [BadgeDotPart::Container, BadgeDotPart::Text],
        );
    }
}

impl Style for BadgeDotStyle {
    type State = BadgeDotState;

    type Basic = BadgeDotBasicStyle;

    get_get_mut! {
        BadgeDotState::Basic => basic,
        BadgeDotState::Disabled => disabled
    }

    fn len() -> usize {
        4 * BadgeDotBasicStyle::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct BadgeDotBasicStyle {
    #[live(BadgeDotBasicStyle::default_text(Theme::default(), BadgeDotState::default()))]
    pub text: LabelBasicStyle,
    #[live(BadgeDotBasicStyle::default_container(Theme::default(), BadgeDotState::default()))]
    pub container: ViewBasicStyle,
}

impl Default for BadgeDotBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), BadgeDotState::default())
    }
}

impl SlotBasicStyle for BadgeDotBasicStyle {
    type Part = BadgeDotPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            BadgeDotPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            BadgeDotPart::Text => self.text.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            BadgeDotPart::Container => self.container.sync(state.into()),
            BadgeDotPart::Text => self.text.sync(state.into()),
        }
    }
}

impl BasicStyle for BadgeDotBasicStyle {
    type State = BadgeDotState;

    type Colors = ();

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            text: Self::default_text(theme, state),
            container: Self::default_container(theme, state),
        }
    }

    fn state_colors(_theme: crate::themes::Theme, _state: Self::State) -> Self::Colors {
        ()
    }

    fn len() -> usize {
        ViewBasicStyle::len() + SvgBasicStyle::len() + LabelBasicStyle::len() + SvgBasicStyle::len()
    }

    fn set_from_str(
        &mut self,
        _key: &str,
        _value: &makepad_widgets::LiveValue,
        _state: Self::State,
    ) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.container.sync(state.into());
        self.text.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(icon), SvgBasicStyle::live_props().into()),
            (live_id!(text), LabelBasicStyle::live_props().into()),
            (live_id!(close), SvgBasicStyle::live_props().into()),
            (live_id!(container), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }

    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

from_prop_to_toml! {
    BadgeDotBasicStyle {
        text => TEXT,
        container => CONTAINER
    }
}
impl TryFrom<(&Value, BadgeDotState)> for BadgeDotBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Value, BadgeDotState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.badge.dot] should be an inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&Item, BadgeDotState)> for BadgeDotBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, BadgeDotState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.badge.dot.$slot] should be an inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&InlineTable, BadgeDotState)> for BadgeDotBasicStyle {
    type Error = Error;

    fn try_from((inline_table, state): (&InlineTable, BadgeDotState)) -> Result<Self, Self::Error> {
        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(BadgeDotBasicStyle::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(Self::default_text(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self { text, container })
    }
}

impl BadgeDotBasicStyle {
    pub fn default_container(theme: Theme, state: BadgeDotState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.height = Size::Fit;
        container.width = Size::Fit;
        container.align = Align::from_f64(0.5);
        container.background_visible = true;
        container.set_padding(Padding::from_all(4.0, 8.0, 4.0, 8.0));
        container.set_border_radius(Radius::new(5.6));
        container.set_flow(Flow::Right);
        container.set_spacing(0.0);
        container
    }

    pub fn default_text(theme: Theme, state: BadgeDotState) -> LabelBasicStyle {
        let mut text = LabelBasicStyle::from_state(theme, state.into());
        text.flow = Flow::Right;
        text.set_font_size(8.0);
        text
    }
}

component_state! {
    BadgeDotState {
        Basic => BASIC,
        Disabled => DISABLED
    },
    _ => BadgeDotState::Basic
}

impl ComponentState for BadgeDotState {
    fn is_disabled(&self) -> bool {
        matches!(self, BadgeDotState::Disabled)
    }
}

impl From<BadgeDotState> for ViewState {
    fn from(value: BadgeDotState) -> Self {
        match value {
            BadgeDotState::Basic => ViewState::Basic,
            BadgeDotState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<BadgeDotState> for LabelState {
    fn from(value: BadgeDotState) -> Self {
        match value {
            BadgeDotState::Basic => LabelState::Basic,
            BadgeDotState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    BadgeDotPart {
        Text => text => TEXT,
        Container => container => CONTAINER
    }, BadgeDotState
}
