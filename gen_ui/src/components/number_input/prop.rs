use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        ButtonBasicStyle, ButtonState, InputState, ViewColors, area::InputAreaBasicStyle, live_props::LiveProps, traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style}, view::{ViewBasicStyle, ViewState}
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys,
        manuel::{BASIC, BUTTON, CONTAINER, DISABLED, INPUT, ITEM}, traits::NewFrom,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    NumberInputStyle {
        basic_prop = NumberInputBasicStyle;
        basic => BASIC, NumberInputBasicStyle::default(), |v| (v, NumberInputState::Basic).try_into(),
        disabled => DISABLED, NumberInputBasicStyle::from_state(Theme::default(), NumberInputState::Disabled), |v| (v, NumberInputState::Disabled).try_into()
    }, "[component.number_input] should be a table"
}

impl Style for NumberInputStyle {
    type State = NumberInputState;

    type Basic = NumberInputBasicStyle;

    get_get_mut! {
        NumberInputState::Basic => basic,
        NumberInputState::Disabled => disabled
    }

    fn len() -> usize {
        2 * NumberInputBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            NumberInputState::Basic,
            [(NumberInputState::Disabled, &mut self.disabled)],
        );
    }
}

impl SlotStyle for NumberInputStyle {
    type Part = NumberInputPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            NumberInputState::Basic,
            [(NumberInputState::Disabled, &mut self.disabled)],
            [NumberInputPart::Container, NumberInputPart::Input, NumberInputPart::Button],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct NumberInputBasicStyle {
    #[live(NumberInputBasicStyle::default_container(Theme::default(), NumberInputState::Basic))]
    pub container: ViewBasicStyle,
    #[live(NumberInputBasicStyle::default_input(Theme::default(), NumberInputState::Basic))]
    pub input: InputAreaBasicStyle,
    #[live(NumberInputBasicStyle::default_button(Theme::default(), NumberInputState::Basic))]
    pub button: ButtonBasicStyle
}

impl BasicStyle for NumberInputBasicStyle {
    type State = NumberInputState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            input: Self::default_input(theme, state),
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
            (live_id!(input), InputAreaBasicStyle::live_props().into()),
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

impl SlotBasicStyle for NumberInputBasicStyle {
    type Part = NumberInputPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            NumberInputPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            NumberInputPart::Input => self.input.set_from_str(key, &value.into(), state.into()),
            NumberInputPart::Button => self.button.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            NumberInputPart::Container => self.container.sync(state.into()),
            NumberInputPart::Input => self.input.sync(state.into()),
            NumberInputPart::Button => self.button.sync(state.into()),
        }
    }
}

impl Default for NumberInputBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), NumberInputState::Basic)
    }
}

from_prop_to_toml! {
    NumberInputBasicStyle {
        container => CONTAINER,
        input => INPUT,
        button => BUTTON
    }
}

impl TryFrom<(&Item, NumberInputState)> for NumberInputBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, NumberInputState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.number_input.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(NumberInputBasicStyle::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let input = get_from_itable(
            inline_table,
            INPUT,
            || {
                Ok(NumberInputBasicStyle::default_input(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, state.into()).try_into(),
        )?;

        let button = get_from_itable(
            inline_table,
            BUTTON,
            || {
                Ok(NumberInputBasicStyle::default_button(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self { container, input, button })
    }
}

impl NumberInputBasicStyle {
    pub fn default_container(theme: Theme, state: NumberInputState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_background_visible(true);
        container.set_background_color(vec4(1.0,0.0,0.0, 1.0));
        container.set_flow(Flow::Right);
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_padding(Padding::from_f64(0.0));
        container.set_spacing(0.0);
        container
    }

    pub fn default_input(theme: Theme, state: NumberInputState) -> InputAreaBasicStyle {
        let mut item = InputAreaBasicStyle::from_state(theme, state.into());
        item.container.set_width(Size::Fill);
        item
    }

    pub fn default_button(theme: Theme, state: NumberInputState) -> ButtonBasicStyle {
        let mut button = ButtonBasicStyle::from_state(theme, state.into());
        button.set_width(Size::Fill);
        button.set_padding(Padding::from_f64(2.0));
        button.set_height(Size::Fill);
        button
    }
}

component_state! {
    NumberInputState {
        Basic => BASIC,
        Disabled => DISABLED
    }, _ => NumberInputState::Basic
}

impl ComponentState for NumberInputState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<NumberInputState> for ViewState {
    fn from(value: NumberInputState) -> Self {
        match value {
            NumberInputState::Basic => ViewState::Basic,
            NumberInputState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for NumberInputState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => NumberInputState::Basic,
            ViewState::Disabled => NumberInputState::Disabled,
            _ => panic!("NumberInputState can only be Basic or Hover"),
        }
    }
}

impl From<NumberInputState> for InputState {
    fn from(value: NumberInputState) -> Self {
        match value {
            NumberInputState::Basic => InputState::Basic,
            NumberInputState::Disabled => InputState::Disabled,
        }
    }
}

impl From<NumberInputState> for ButtonState {
    fn from(value: NumberInputState) -> Self {
        match value {
            NumberInputState::Basic => ButtonState::Basic,
            NumberInputState::Disabled => ButtonState::Disabled,
        }
    }
}

component_part! {
    NumberInputPart {
        Container => container => CONTAINER,
        Input => input => INPUT,
        Button => button => BUTTON
    }, NumberInputState
}
