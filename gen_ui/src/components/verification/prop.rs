use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        ButtonBasicStyle, ButtonState, ViewColors,
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys,
        manuel::{BASIC, CONTAINER, DISABLED, ITEM, PREFIX, SUFFIX},
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
            [
                VerificationPart::Container,
                VerificationPart::Prefix,
                VerificationPart::Item,
                VerificationPart::Suffix,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct VerificationBasicStyle {
    #[live(VerificationBasicStyle::default_container(Theme::default(), VerificationState::Basic))]
    pub container: ViewBasicStyle,
    #[live(VerificationBasicStyle::default_prefix(Theme::default(), VerificationState::Basic))]
    pub prefix: ButtonBasicStyle,
    #[live(VerificationBasicStyle::default_item(Theme::default(), VerificationState::Basic))]
    pub item: ButtonBasicStyle,
    #[live(VerificationBasicStyle::default_suffix(Theme::default(), VerificationState::Basic))]
    pub suffix: ButtonBasicStyle,
}

impl BasicStyle for VerificationBasicStyle {
    type State = VerificationState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            prefix: Self::default_prefix(theme, state),
            item: Self::default_item(theme, state),
            suffix: Self::default_suffix(theme, state),
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
            (live_id!(header), ViewBasicStyle::live_props().into()),
            (live_id!(body), ViewBasicStyle::live_props().into()),
            (live_id!(footer), ViewBasicStyle::live_props().into()),
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
            VerificationPart::Prefix => self.prefix.set_from_str(key, &value.into(), state.into()),
            VerificationPart::Item => self.item.set_from_str(key, &value.into(), state.into()),
            VerificationPart::Suffix => self.suffix.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            VerificationPart::Container => self.container.sync(state.into()),
            VerificationPart::Prefix => self.prefix.sync(state.into()),
            VerificationPart::Item => self.item.sync(state.into()),
            VerificationPart::Suffix => self.suffix.sync(state.into()),
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
        prefix => PREFIX,
        item => ITEM,
        suffix => SUFFIX
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

        let prefix = get_from_itable(
            inline_table,
            PREFIX,
            || {
                Ok(VerificationBasicStyle::default_prefix(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, state.into()).try_into(),
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

        let suffix = get_from_itable(
            inline_table,
            SUFFIX,
            || {
                Ok(VerificationBasicStyle::default_suffix(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self {
            container,
            prefix,
            item,
            suffix,
        })
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

    pub fn default_prefix(theme: Theme, state: VerificationState) -> ButtonBasicStyle {
        let mut prefix = ButtonBasicStyle::from_state(theme, state.into());
        prefix.set_cursor(Default::default());
        prefix
    }

    pub fn default_item(theme: Theme, state: VerificationState) -> ButtonBasicStyle {
        let mut item = ButtonBasicStyle::from_state(theme, state.into());
        item.set_cursor(Default::default());
        item
    }

    pub fn default_suffix(theme: Theme, state: VerificationState) -> ButtonBasicStyle {
        let mut suffix = ButtonBasicStyle::from_state(theme, state.into());
        suffix.set_cursor(Default::default());
        suffix
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

impl From<VerificationState> for ButtonState {
    fn from(value: VerificationState) -> Self {
        match value {
            VerificationState::Basic => ButtonState::Basic,
            VerificationState::Disabled => ButtonState::Disabled,
        }
    }
}

component_part! {
    VerificationPart {
        Container => container => CONTAINER,
        Prefix => prefix => PREFIX,
        Item => item => ITEM,
        Suffix => suffix => SUFFIX
    }, VerificationState
}
