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
    PaginationStyle {
        basic_prop = PaginationBasicStyle;
        basic => BASIC, PaginationBasicStyle::default(), |v| (v, PaginationState::Basic).try_into(),
        disabled => DISABLED, PaginationBasicStyle::from_state(Theme::default(), PaginationState::Disabled), |v| (v, PaginationState::Disabled).try_into()
    }, "[component.pagination] should be a table"
}

impl Style for PaginationStyle {
    type State = PaginationState;

    type Basic = PaginationBasicStyle;

    get_get_mut! {
        PaginationState::Basic => basic,
        PaginationState::Disabled => disabled
    }

    fn len() -> usize {
        2 * PaginationBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            PaginationState::Basic,
            [(PaginationState::Disabled, &mut self.disabled)],
        );
    }
}

impl SlotStyle for PaginationStyle {
    type Part = PaginationPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            PaginationState::Basic,
            [(PaginationState::Disabled, &mut self.disabled)],
            [
                PaginationPart::Container,
                PaginationPart::Prefix,
                PaginationPart::Item,
                PaginationPart::Suffix,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct PaginationBasicStyle {
    #[live(PaginationBasicStyle::default_container(Theme::default(), PaginationState::Basic))]
    pub container: ViewBasicStyle,
    #[live(PaginationBasicStyle::default_prefix(Theme::default(), PaginationState::Basic))]
    pub prefix: ButtonBasicStyle,
    #[live(PaginationBasicStyle::default_item(Theme::default(), PaginationState::Basic))]
    pub item: ButtonBasicStyle,
    #[live(PaginationBasicStyle::default_suffix(Theme::default(), PaginationState::Basic))]
    pub suffix: ButtonBasicStyle,
}

impl BasicStyle for PaginationBasicStyle {
    type State = PaginationState;

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

impl SlotBasicStyle for PaginationBasicStyle {
    type Part = PaginationPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            PaginationPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            PaginationPart::Prefix => self.prefix.set_from_str(key, &value.into(), state.into()),
            PaginationPart::Item => self.item.set_from_str(key, &value.into(), state.into()),
            PaginationPart::Suffix => self.suffix.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            PaginationPart::Container => self.container.sync(state.into()),
            PaginationPart::Prefix => self.prefix.sync(state.into()),
            PaginationPart::Item => self.item.sync(state.into()),
            PaginationPart::Suffix => self.suffix.sync(state.into()),
        }
    }
}

impl Default for PaginationBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), PaginationState::Basic)
    }
}

from_prop_to_toml! {
    PaginationBasicStyle {
        container => CONTAINER,
        prefix => PREFIX,
        item => ITEM,
        suffix => SUFFIX
    }
}

impl TryFrom<(&Item, PaginationState)> for PaginationBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, PaginationState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.pagination.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(PaginationBasicStyle::default_container(
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
                Ok(PaginationBasicStyle::default_prefix(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, state.into()).try_into(),
        )?;

        let item = get_from_itable(
            inline_table,
            ITEM,
            || Ok(PaginationBasicStyle::default_item(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        let suffix = get_from_itable(
            inline_table,
            SUFFIX,
            || {
                Ok(PaginationBasicStyle::default_suffix(
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

impl PaginationBasicStyle {
    pub fn default_container(theme: Theme, state: PaginationState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_background_visible(false);
        container.set_flow(Flow::Right);
        container.set_height(Size::Fit);
        container.set_width(Size::Fit);
        container
    }

    pub fn default_prefix(theme: Theme, state: PaginationState) -> ButtonBasicStyle {
        let mut prefix = ButtonBasicStyle::from_state(theme, state.into());
        prefix.set_cursor(Default::default());
        prefix
    }

    pub fn default_item(theme: Theme, state: PaginationState) -> ButtonBasicStyle {
        let mut item = ButtonBasicStyle::from_state(theme, state.into());
        item.set_cursor(Default::default());
        item
    }

    pub fn default_suffix(theme: Theme, state: PaginationState) -> ButtonBasicStyle {
        let mut suffix = ButtonBasicStyle::from_state(theme, state.into());
        suffix.set_cursor(Default::default());
        suffix
    }
}

component_state! {
    PaginationState {
        Basic => BASIC,
        Disabled => DISABLED
    }, _ => PaginationState::Basic
}

impl ComponentState for PaginationState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<PaginationState> for ViewState {
    fn from(value: PaginationState) -> Self {
        match value {
            PaginationState::Basic => ViewState::Basic,
            PaginationState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for PaginationState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => PaginationState::Basic,
            ViewState::Disabled => PaginationState::Disabled,
            _ => panic!("PaginationState can only be Basic or Hover"),
        }
    }
}

impl From<PaginationState> for ButtonState {
    fn from(value: PaginationState) -> Self {
        match value {
            PaginationState::Basic => ButtonState::Basic,
            PaginationState::Disabled => ButtonState::Disabled,
        }
    }
}

component_part! {
    PaginationPart {
        Container => container => CONTAINER,
        Prefix => prefix => PREFIX,
        Item => item => ITEM,
        Suffix => suffix => SUFFIX
    }, PaginationState
}
