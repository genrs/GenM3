use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        ViewColors,
        dot::{BadgeDotBasicStyle, BadgeDotPart, BadgeDotState},
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys,
        manuel::{BASIC, CONTAINER, DISABLED, DOT},
        traits::NewFrom,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    BadgeStyle {
        basic_prop = BadgeBasicStyle;
        basic => BASIC, BadgeBasicStyle::default(), |v| (v, BadgeState::Basic).try_into(),
        disabled => DISABLED, BadgeBasicStyle::from_state(Theme::default(), BadgeState::Disabled), |v| (v, BadgeState::Disabled).try_into()
    }, "[component.badge] should be a table"
}

impl Style for BadgeStyle {
    type State = BadgeState;

    type Basic = BadgeBasicStyle;

    get_get_mut! {
        BadgeState::Basic => basic,
        BadgeState::Disabled => disabled
    }

    fn len() -> usize {
        2 * BadgeBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            BadgeState::Basic,
            [(BadgeState::Disabled, &mut self.disabled)],
        );
    }
}

impl SlotStyle for BadgeStyle {
    type Part = BadgePart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            BadgeState::Basic,
            [(BadgeState::Disabled, &mut self.disabled)],
            [BadgePart::Container, BadgePart::Dot],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct BadgeBasicStyle {
    #[live(BadgeBasicStyle::default_container(Theme::default(), BadgeState::Basic))]
    pub container: ViewBasicStyle,
    #[live(BadgeBasicStyle::default_dot(Theme::default(), BadgeState::Basic))]
    pub dot: BadgeDotBasicStyle,
}

impl BasicStyle for BadgeBasicStyle {
    type State = BadgeState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            dot: Self::default_dot(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicStyle::state_colors(theme, state.into())
    }

    fn len() -> usize {
        3 * ViewBasicStyle::len()
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.dot.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(dot), BadgeDotBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for BadgeBasicStyle {
    type Part = BadgePart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            BadgePart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            BadgePart::Dot => {
                let dot_part = BadgeDotPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.dot
                        .set_from_str_slot(key, value, state.into(), dot_part);
                }
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            BadgePart::Container => self.container.sync(state.into()),
            BadgePart::Dot => {
                self.dot.sync_slot(state.into(), BadgeDotPart::Text);
                self.dot.sync_slot(state.into(), BadgeDotPart::Container);
            }
        }
    }
}

impl Default for BadgeBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), BadgeState::Basic)
    }
}

from_prop_to_toml! {
    BadgeBasicStyle {
        container => CONTAINER,
        dot => DOT
    }
}

impl TryFrom<(&Item, BadgeState)> for BadgeBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, BadgeState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.badge.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(BadgeBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let dot = get_from_itable(
            inline_table,
            DOT,
            || Ok(BadgeBasicStyle::default_dot(Theme::default(), state)),
            |v| (v, BadgeDotState::from(state)).try_into(),
        )?;

        Ok(Self { container, dot })
    }
}

impl BadgeBasicStyle {
    pub fn default_dot(theme: Theme, state: BadgeState) -> BadgeDotBasicStyle {
        BadgeDotBasicStyle::from_state(theme, state.into())
    }
    pub fn default_container(theme: Theme, state: BadgeState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_background_visible(true);
        container.set_padding(Padding::from_f64(0.0));
        container.set_height(Size::Fit);
        container.set_width(Size::Fit);
        container
    }
}

component_state! {
    BadgeState {
        Basic => BASIC,
        Disabled => DISABLED
    }, _ => BadgeState::Basic
}

impl ComponentState for BadgeState {
    fn is_disabled(&self) -> bool {
        matches!(self, BadgeState::Disabled)
    }
}

impl From<BadgeState> for ViewState {
    fn from(value: BadgeState) -> Self {
        match value {
            BadgeState::Basic => ViewState::Basic,
            BadgeState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<BadgeState> for BadgeDotState {
    fn from(value: BadgeState) -> Self {
        match value {
            BadgeState::Basic => BadgeDotState::Basic,
            BadgeState::Disabled => BadgeDotState::Disabled,
        }
    }
}

component_part! {
    BadgePart {
        Container => container => CONTAINER,
        Dot => dot => DOT
    }, BadgeState
}
