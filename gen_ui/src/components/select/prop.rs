use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    basic_prop_interconvert, component_color, component_part, component_state,
    components::{
        LabelState, SvgState, ViewColors,
        item::{SelectItemBasicStyle, SelectItemPart},
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys, Radius,
        manuel::{
            ACTIVE, BASIC, COLOR, CONTAINER, DISABLED, EMPTY, FOCUS, HOVER, INPUT, PREFIX, SUFFIX,
            THEME,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
    },
    prop_interconvert, state_color,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    SelectStyle {
        basic_prop = SelectBasicStyle;
        basic => BASIC, SelectBasicStyle::default(), |v| (v, SelectState::Basic).try_into(),
        hover => HOVER, SelectBasicStyle::from_state(Theme::default(), SelectState::Hover), |v| (v, SelectState::Hover).try_into(),
        active => ACTIVE, SelectBasicStyle::from_state(Theme::default(), SelectState::Active), |v| (v, SelectState::Active).try_into(),
        disabled => DISABLED, SelectBasicStyle::from_state(Theme::default(), SelectState::Disabled), |v| (v, SelectState::Disabled).try_into()
    }, "[component.item] should be a table"
}

impl Style for SelectStyle {
    type State = SelectState;

    type Basic = SelectBasicStyle;

    get_get_mut! {
        SelectState::Basic => basic,

        SelectState::Hover => hover,
        SelectState::Active => active,
        SelectState::Disabled => disabled
    }

    fn len() -> usize {
        5 * SelectBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            SelectState::Basic,
            [
                (SelectState::Hover, &mut self.hover),
                (SelectState::Active, &mut self.active),
                (SelectState::Disabled, &mut self.disabled),
            ],
        );
    }
}

impl SlotStyle for SelectStyle {
    type Part = SelectPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            SelectState::Basic,
            [
                (SelectState::Hover, &mut self.hover),
                (SelectState::Active, &mut self.active),
                (SelectState::Disabled, &mut self.disabled),
            ],
            [
                SelectPart::Container,
                SelectPart::Select,
                SelectPart::Prefix,
                SelectPart::Suffix,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct SelectBasicStyle {
    #[live(SelectBasicStyle::default_container(Theme::default(), SelectState::Basic))]
    pub container: ViewBasicStyle,
    #[live(SelectBasicStyle::default_select(Theme::default(), SelectState::Basic))]
    pub item: SelectItemBasicStyle,
    #[live(SelectBasicStyle::default_prefix(Theme::default(), SelectState::Basic))]
    pub prefix: ViewBasicStyle,
    #[live(SelectBasicStyle::default_suffix(Theme::default(), SelectState::Basic))]
    pub suffix: ViewBasicStyle,
}

impl BasicStyle for SelectBasicStyle {
    type State = SelectState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            prefix: Self::default_prefix(theme, state),
            suffix: Self::default_suffix(theme, state),
            item: Self::default_select(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicStyle::state_colors(theme, state.into())
    }

    fn len() -> usize {
        3 * ViewBasicStyle::len() + SelectItemBasicStyle::len()
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.container.sync(state.into());
        self.item.sync(state);
        self.prefix.sync(state.into());
        self.suffix.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(item), SelectItemBasicStyle::live_props().into()),
            (live_id!(prefix), ViewBasicStyle::live_props().into()),
            (live_id!(suffix), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for SelectBasicStyle {
    type Part = SelectPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            SelectPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            SelectPart::Prefix => self.prefix.set_from_str(key, &value.into(), state.into()),
            SelectPart::Suffix => self.suffix.set_from_str(key, &value.into(), state.into()),
            SelectPart::Select => {
                let item_part = SelectItemPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.item
                        .set_from_str_slot(key, value, state.into(), item_part);
                }
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            SelectPart::Container => self.container.sync(state.into()),
            SelectPart::Prefix => self.prefix.sync(state.into()),
            SelectPart::Suffix => self.suffix.sync(state.into()),
            SelectPart::Select => {
                self.item.sync_slot(state, SelectItemPart::Container);
                self.item.sync_slot(state, SelectItemPart::Text);
                self.item.sync_slot(state, SelectItemPart::Icon);
                self.item.sync_slot(state, SelectItemPart::Suffix);
            }
        }
    }
}

impl Default for SelectBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), SelectState::Basic)
    }
}

from_prop_to_toml! {
    SelectBasicStyle {
        container => CONTAINER,
        prefix => PREFIX,
        suffix => SUFFIX
    }
}

impl TryFrom<(&Item, SelectState)> for SelectBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, SelectState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.item.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(SelectBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let item = get_from_itable(
            inline_table,
            INPUT,
            || Ok(SelectBasicStyle::default_select(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;

        let prefix = get_from_itable(
            inline_table,
            PREFIX,
            || Ok(SelectBasicStyle::default_prefix(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let suffix = get_from_itable(
            inline_table,
            SUFFIX,
            || Ok(SelectBasicStyle::default_suffix(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            item,
            prefix,
            suffix,
        })
    }
}

impl SelectBasicStyle {
    pub fn default_container(theme: Theme, state: SelectState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(MouseCursor::Text);
        container.set_background_visible(true);
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_border_radius(Radius::new(2.0));
        container.set_flow(Flow::Right);
        container.set_padding(Padding::from_f64(0.0));
        container.set_spacing(0.0);
        container
    }

    pub fn default_prefix(theme: Theme, state: SelectState) -> ViewBasicStyle {
        let mut prefix = ViewBasicStyle::from_state(theme, state.into());
        prefix.set_background_visible(true);
        prefix.set_padding(Padding::from_f64(0.0));
        prefix.set_height(Size::Fill);
        prefix.set_width(Size::Fit);
        prefix.set_align(Align::from_f64(0.5));
        prefix.set_border_radius(Radius::from_all(2.0, 0.0, 0.0, 2.0));
        prefix
    }

    pub fn default_suffix(theme: Theme, state: SelectState) -> ViewBasicStyle {
        let mut suffix = Self::default_prefix(theme, state);
        suffix.set_border_radius(Radius::from_all(0.0, 2.0, 2.0, 0.0));
        suffix
    }

    pub fn default_select(theme: Theme, state: SelectState) -> SelectItemBasicStyle {
        let mut item = SelectItemBasicStyle::from_state(theme, state);
        item.container.set_border_radius(Radius::new(0.0));
        item
    }
}

component_state! {
    SelectState {
        Basic => BASIC,
        Hover => HOVER,
        Active => ACTIVE,
        Disabled => DISABLED
    },
    _ => SelectState::Basic
}

impl ComponentState for SelectState {
    fn is_disabled(&self) -> bool {
        matches!(self, SelectState::Disabled)
    }
}

impl From<SelectState> for ViewState {
    fn from(state: SelectState) -> Self {
        match state {
            SelectState::Basic => ViewState::Basic,
            SelectState::Hover => ViewState::Hover,
            SelectState::Active => ViewState::Pressed,
            SelectState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<SelectState> for LabelState {
    fn from(value: SelectState) -> Self {
        match value {
            SelectState::Basic | SelectState::Hover | SelectState::Active => LabelState::Basic,
            SelectState::Disabled => LabelState::Disabled,
        }
    }
}

impl From<SelectState> for SvgState {
    fn from(value: SelectState) -> Self {
        match value {
            SelectState::Basic => SvgState::Basic,
            SelectState::Hover => SvgState::Hover,
            SelectState::Active => SvgState::Pressed,
            SelectState::Disabled => SvgState::Disabled,
        }
    }
}

component_part! {
    SelectPart {
        Container => container => CONTAINER,
        Select => item => INPUT,
        Prefix => prefix => PREFIX,
        Suffix => suffix => SUFFIX
    }, SelectState
}
