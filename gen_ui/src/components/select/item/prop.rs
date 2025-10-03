use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        LabelBasicStyle, LabelState, SlotBasicStyle, SlotStyle, SvgBasicStyle, SvgState,
        ViewColors, ViewState,
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
        view::ViewBasicStyle,
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl,
        manuel::{ACTIVE, BASIC, CONTAINER, DISABLED, HOVER, ICON, SUFFIX, TEXT},
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    SelectItemStyle {
        basic_prop = SelectItemBasicStyle;
        basic => BASIC, SelectItemBasicStyle::default(),|v| (v, SelectItemState::Basic).try_into(),
        hover => HOVER, SelectItemBasicStyle::from_state(Theme::default(), SelectItemState::Hover),|v| (v, SelectItemState::Hover).try_into(),
        active => ACTIVE, SelectItemBasicStyle::from_state(Theme::default(), SelectItemState::Active),|v| (v, SelectItemState::Active).try_into(),
        disabled => DISABLED, SelectItemBasicStyle::from_state(Theme::default(), SelectItemState::Disabled),|v| (v, SelectItemState::Disabled).try_into()
    }, "[component.select.item] should be a table"
}

impl SlotStyle for SelectItemStyle {
    type Part = SelectItemPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            SelectItemState::Basic,
            [
                (SelectItemState::Hover, &mut self.hover),
                (SelectItemState::Active, &mut self.active),
                (SelectItemState::Disabled, &mut self.disabled),
            ],
            [
                SelectItemPart::Container,
                SelectItemPart::Icon,
                SelectItemPart::Text,
                SelectItemPart::Suffix,
            ],
        );
    }
}

impl Style for SelectItemStyle {
    type State = SelectItemState;

    type Basic = SelectItemBasicStyle;

    fn len() -> usize {
        SelectItemBasicStyle::len() * 4 // basic, hover, active, disabled
    }

    get_get_mut! {
        SelectItemState::Basic => basic,
        SelectItemState::Hover => hover,
        SelectItemState::Active => active,
        SelectItemState::Disabled => disabled
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            SelectItemState::Basic,
            [
                (SelectItemState::Hover, &mut self.hover),
                (SelectItemState::Active, &mut self.active),
                (SelectItemState::Disabled, &mut self.disabled),
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct SelectItemBasicStyle {
    #[live(SelectItemBasicStyle::default_container(Theme::default(), SelectItemState::Basic))]
    pub container: ViewBasicStyle,
    #[live(SelectItemBasicStyle::default_icon(Theme::default(), SelectItemState::Basic))]
    pub icon: SvgBasicStyle,
    #[live(SelectItemBasicStyle::default_text(Theme::default(), SelectItemState::Basic))]
    pub text: LabelBasicStyle,
    #[live(SelectItemBasicStyle::default_suffix(Theme::default(), SelectItemState::Basic))]
    pub suffix: SvgBasicStyle,
}

impl BasicStyle for SelectItemBasicStyle {
    type State = SelectItemState;

    type Colors = ViewColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            icon: Self::default_icon(theme, state),
            text: Self::default_text(theme, state),
            suffix: Self::default_suffix(theme, state),
        }
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        ViewBasicStyle::state_colors(theme, state.into())
    }

    fn len() -> usize {
        4 * (2 * SvgBasicStyle::len() + LabelBasicStyle::len() + ViewBasicStyle::len())
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.icon.sync(state.into());
        self.text.sync(state.into());
        self.suffix.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(icon), SvgBasicStyle::live_props().into()),
            (live_id!(text), LabelBasicStyle::live_props().into()),
            (live_id!(suffix), SvgBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for SelectItemBasicStyle {
    type Part = SelectItemPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            SelectItemPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            SelectItemPart::Icon => self.icon.set_from_str(key, &value.into(), state.into()),
            SelectItemPart::Text => self.text.set_from_str(key, &value.into(), state.into()),
            SelectItemPart::Suffix => self.suffix.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            SelectItemPart::Container => self.container.sync(state.into()),
            SelectItemPart::Icon => self.icon.sync(state.into()),
            SelectItemPart::Text => self.text.sync(state.into()),
            SelectItemPart::Suffix => self.suffix.sync(state.into()),
        }
    }
}

impl Default for SelectItemBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), SelectItemState::Basic)
    }
}

from_prop_to_toml! {
    SelectItemBasicStyle {
        container => CONTAINER,
        icon => ICON,
        text => TEXT,
        suffix => SUFFIX
    }
}

impl TryFrom<(&Item, SelectItemState)> for SelectItemBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, SelectItemState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.select.item.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(SelectItemBasicStyle::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(SelectItemBasicStyle::default_icon(Theme::default(), state)),
            |v| (v, SvgState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(SelectItemBasicStyle::default_text(Theme::default(), state)),
            |v| (v, LabelState::from(state)).try_into(),
        )?;

        let suffix = get_from_itable(
            inline_table,
            SUFFIX,
            || {
                Ok(SelectItemBasicStyle::default_suffix(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, SvgState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            icon,
            text,
            suffix,
        })
    }
}

impl SelectItemBasicStyle {
    pub fn default_container(theme: Theme, state: SelectItemState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(MouseCursor::Hand);
        container.set_background_visible(true);
        container
    }
    pub fn default_icon(theme: Theme, state: SelectItemState) -> SvgBasicStyle {
        SvgBasicStyle::from_state(theme, state.into())
    }
    pub fn default_text(theme: Theme, state: SelectItemState) -> LabelBasicStyle {
        LabelBasicStyle::from_state(theme, state.into())
    }
    pub fn default_suffix(theme: Theme, state: SelectItemState) -> SvgBasicStyle {
        SvgBasicStyle::from_state(theme, state.into())
    }
}

component_state! {
    SelectItemState {
        Basic => BASIC,
        Hover => HOVER,
        Active => ACTIVE,
        Disabled => DISABLED
    },
    _ => SelectItemState::Basic
}

impl ComponentState for SelectItemState {
    fn is_disabled(&self) -> bool {
        matches!(self, SelectItemState::Disabled)
    }
}

impl From<SelectItemState> for ViewState {
    fn from(state: SelectItemState) -> Self {
        match state {
            SelectItemState::Basic => ViewState::Basic,
            SelectItemState::Hover => ViewState::Hover,
            SelectItemState::Active => ViewState::Pressed,
            SelectItemState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<SelectItemState> for LabelState {
    fn from(value: SelectItemState) -> Self {
        match value {
            SelectItemState::Basic | SelectItemState::Hover | SelectItemState::Active => {
                LabelState::Basic
            }
            SelectItemState::Disabled => LabelState::Disabled,
        }
    }
}

impl From<SelectItemState> for SvgState {
    fn from(value: SelectItemState) -> Self {
        match value {
            SelectItemState::Basic => SvgState::Basic,
            SelectItemState::Hover => SvgState::Hover,
            SelectItemState::Active => SvgState::Pressed,
            SelectItemState::Disabled => SvgState::Disabled,
        }
    }
}

component_part! {
    SelectItemPart {
        Container => container => CONTAINER,
        Icon => icon => ICON,
        Text => text => TEXT,
        Suffix => suffix => SUFFIX
    }, SelectItemState
}
