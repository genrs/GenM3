use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    component_part,
    components::{
        ButtonBasicStyle, ButtonState, ViewColors,
        live_props::LiveProps,
        traits::{BasicStyle, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys,
        manuel::{BASIC, BUTTON, CONTAINER, DISABLED, HOVER, PRESSED},
        traits::NewFrom,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    NumberCtrStyle {
        basic_prop = NumberCtrBasicStyle;
        basic => BASIC, NumberCtrBasicStyle::default(), |v| (v, ButtonState::Basic).try_into(),
        hover => HOVER, NumberCtrBasicStyle::from_state(Theme::default(), ButtonState::Hover), |v| (v, ButtonState::Hover).try_into(),
        pressed => PRESSED, NumberCtrBasicStyle::from_state(Theme::default(), ButtonState::Pressed), |v| (v, ButtonState::Pressed).try_into(),
        disabled => DISABLED, NumberCtrBasicStyle::from_state(Theme::default(), ButtonState::Disabled), |v| (v, ButtonState::Disabled).try_into()
    }, "[component.number_ctr] should be a table"
}

impl Style for NumberCtrStyle {
    type State = ButtonState;

    type Basic = NumberCtrBasicStyle;

    get_get_mut! {
        ButtonState::Basic => basic,
        ButtonState::Hover => hover,
        ButtonState::Pressed => pressed,
        ButtonState::Disabled => disabled
    }

    fn len() -> usize {
        2 * NumberCtrBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            ButtonState::Basic,
            [
                (ButtonState::Disabled, &mut self.disabled),
                (ButtonState::Hover, &mut self.hover),
                (ButtonState::Pressed, &mut self.pressed),
            ],
        );
    }
}

impl SlotStyle for NumberCtrStyle {
    type Part = NumberCtrPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            ButtonState::Basic,
            [
                (ButtonState::Disabled, &mut self.disabled),
                (ButtonState::Hover, &mut self.hover),
                (ButtonState::Pressed, &mut self.pressed),
            ],
            [NumberCtrPart::Container, NumberCtrPart::Button],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct NumberCtrBasicStyle {
    #[live(NumberCtrBasicStyle::default_container(Theme::default(), ButtonState::Basic))]
    pub container: ViewBasicStyle,
    #[live(NumberCtrBasicStyle::default_button(Theme::default(), ButtonState::Basic))]
    pub button: ButtonBasicStyle,
}

impl BasicStyle for NumberCtrBasicStyle {
    type State = ButtonState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            button: Self::default_button(theme, state),
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

    fn sync(&mut self, _state: Self::State) -> () {
        ()
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(button), ButtonBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for NumberCtrBasicStyle {
    type Part = NumberCtrPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            NumberCtrPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            NumberCtrPart::Button => self.button.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            NumberCtrPart::Container => self.container.sync(state.into()),
            NumberCtrPart::Button => self.button.sync(state.into()),
        }
    }
}

impl Default for NumberCtrBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), ButtonState::Basic)
    }
}

from_prop_to_toml! {
    NumberCtrBasicStyle {
        container => CONTAINER,
        button => BUTTON
    }
}

impl TryFrom<(&InlineTable, ButtonState)> for NumberCtrBasicStyle {
    type Error = Error;

    fn try_from((inline_table, state): (&InlineTable, ButtonState)) -> Result<Self, Self::Error> {
        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(NumberCtrBasicStyle::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let button = get_from_itable(
            inline_table,
            BUTTON,
            || Ok(NumberCtrBasicStyle::default_button(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self { container, button })
    }
}

impl TryFrom<(&Value, ButtonState)> for NumberCtrBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Value, ButtonState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.number_ctr.$slot] should be an inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&Item, ButtonState)> for NumberCtrBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, ButtonState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.number_ctr.$slot] should be an inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl NumberCtrBasicStyle {
    pub fn default_container(theme: Theme, state: ButtonState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_background_visible(false);
        container.set_flow(Flow::Down);
        container.set_height(Size::Fill);
        container.set_width(Size::Fill);
        container.set_padding(Padding::from_f64(0.0));
        container.set_spacing(0.0);
        container
    }

    pub fn default_button(theme: Theme, state: ButtonState) -> ButtonBasicStyle {
        let mut button = ButtonBasicStyle::from_state(theme, state.into());
        button.set_width(Size::Fill);
        let padding = if state == ButtonState::Hover {
            Padding::from_xy(4.0, 2.0)
        } else {
            Padding::from_f64(2.0)
        };
        button.set_padding(padding);
        button.set_height(Size::Fill);
        button
    }
}

component_part! {
    NumberCtrPart {
        Container => container => CONTAINER,
        Button => button => BUTTON
    }, ButtonState
}
