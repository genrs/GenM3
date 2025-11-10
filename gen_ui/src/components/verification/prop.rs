use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        InputState, ViewColors,
        area::InputAreaBasicStyle,
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys,
        manuel::{BASIC, CONTAINER, DISABLED, ITEM},
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    VerificationStyle {
        basic_prop = VerificationBasicStyle;
        basic => BASIC, VerificationBasicStyle::default(), |v| (v, VerificationState::Basic).try_into(),
        disabled => DISABLED, VerificationBasicStyle::from_state(Theme::default(), VerificationState::Disabled), |v| (v, VerificationState::Disabled).try_into()
    }, "[component.pagination] should be a table"
}

impl Style for VerificationStyle {
    type State = VerificationState;

    type Basic = VerificationBasicStyle;

    get_get_mut! {
        VerificationState::Basic => basic,
        VerificationState::Disabled => disabled
    }

    fn len() -> usize {
        2 * VerificationBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            VerificationState::Basic,
            [(VerificationState::Disabled, &mut self.disabled)],
        );
    }
}

impl SlotStyle for VerificationStyle {
    type Part = VerificationPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            VerificationState::Basic,
            [(VerificationState::Disabled, &mut self.disabled)],
            [VerificationPart::Container, VerificationPart::Item],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct VerificationBasicStyle {
    #[live(VerificationBasicStyle::default_container(Theme::default(), VerificationState::Basic))]
    pub container: ViewBasicStyle,
    #[live(VerificationBasicStyle::default_item(Theme::default(), VerificationState::Basic))]
    pub item: InputAreaBasicStyle,
}

impl BasicStyle for VerificationBasicStyle {
    type State = VerificationState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            item: Self::default_item(theme, state),
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
            (live_id!(item), InputAreaBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for VerificationBasicStyle {
    type Part = VerificationPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            VerificationPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            VerificationPart::Item => self.item.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            VerificationPart::Container => self.container.sync(state.into()),
            VerificationPart::Item => self.item.sync(state.into()),
        }
    }
}

impl Default for VerificationBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), VerificationState::Basic)
    }
}

from_prop_to_toml! {
    VerificationBasicStyle {
        container => CONTAINER,
        item => ITEM
    }
}

impl TryFrom<(&Item, VerificationState)> for VerificationBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, VerificationState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.pagination.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(VerificationBasicStyle::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let item = get_from_itable(
            inline_table,
            ITEM,
            || {
                Ok(VerificationBasicStyle::default_item(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self { container, item })
    }
}

impl VerificationBasicStyle {
    pub fn default_container(theme: Theme, state: VerificationState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_background_visible(false);
        container.set_flow(Flow::Right);
        container.set_height(Size::Fit);
        container.set_width(Size::Fit);
        container
    }

    pub fn default_item(theme: Theme, state: VerificationState) -> InputAreaBasicStyle {
        let mut item = InputAreaBasicStyle::from_state(theme, state.into());
        item.container.set_width(Size::Fixed(42.0));
        
        item
    }
}

component_state! {
    VerificationState {
        Basic => BASIC,
        Disabled => DISABLED
    }, _ => VerificationState::Basic
}

impl ComponentState for VerificationState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<VerificationState> for ViewState {
    fn from(value: VerificationState) -> Self {
        match value {
            VerificationState::Basic => ViewState::Basic,
            VerificationState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for VerificationState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => VerificationState::Basic,
            ViewState::Disabled => VerificationState::Disabled,
            _ => panic!("VerificationState can only be Basic or Hover"),
        }
    }
}

impl From<VerificationState> for InputState {
    fn from(value: VerificationState) -> Self {
        match value {
            VerificationState::Basic => InputState::Basic,
            VerificationState::Disabled => InputState::Disabled,
        }
    }
}

component_part! {
    VerificationPart {
        Container => container => CONTAINER,
        Item => item => ITEM
    }, VerificationState
}
