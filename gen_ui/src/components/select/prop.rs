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
        ApplySlotMapImpl, ApplyStateMapImpl, Radius,
        manuel::{ACTIVE, BASIC, CONTAINER, DISABLED, HOVER, ICON, SUFFIX, TEXT},
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    SelectStyle {
        basic_prop = SelectBasicStyle;
        basic => BASIC, SelectBasicStyle::default(),|v| (v, SelectState::Basic).try_into(),
        hover => HOVER, SelectBasicStyle::from_state(Theme::default(), SelectState::Hover),|v| (v, SelectState::Hover).try_into(),
        active => ACTIVE, SelectBasicStyle::from_state(Theme::default(), SelectState::Active),|v| (v, SelectState::Active).try_into(),
        disabled => DISABLED, SelectBasicStyle::from_state(Theme::default(), SelectState::Disabled),|v| (v, SelectState::Disabled).try_into()
    }, "[component.select.item] should be a table"
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
                SelectPart::Icon,
                SelectPart::Text,
                SelectPart::Suffix,
            ],
        );
    }
}

impl Style for SelectStyle {
    type State = SelectState;

    type Basic = SelectBasicStyle;

    fn len() -> usize {
        SelectBasicStyle::len() * 4 // basic, hover, active, disabled
    }

    get_get_mut! {
        SelectState::Basic => basic,
        SelectState::Hover => hover,
        SelectState::Active => active,
        SelectState::Disabled => disabled
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

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct SelectBasicStyle {
    #[live(SelectBasicStyle::default_container(Theme::default(), SelectState::Basic))]
    pub container: ViewBasicStyle,
    #[live(SelectBasicStyle::default_icon(Theme::default(), SelectState::Basic))]
    pub icon: SvgBasicStyle,
    #[live(SelectBasicStyle::default_text(Theme::default(), SelectState::Basic))]
    pub text: LabelBasicStyle,
    #[live(SelectBasicStyle::default_suffix(Theme::default(), SelectState::Basic))]
    pub suffix: SvgBasicStyle,
}

impl BasicStyle for SelectBasicStyle {
    type State = SelectState;

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

impl SlotBasicStyle for SelectBasicStyle {
    type Part = SelectPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            SelectPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            SelectPart::Icon => self.icon.set_from_str(key, &value.into(), state.into()),
            SelectPart::Text => self.text.set_from_str(key, &value.into(), state.into()),
            SelectPart::Suffix => self.suffix.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            SelectPart::Container => self.container.sync(state.into()),
            SelectPart::Icon => self.icon.sync(state.into()),
            SelectPart::Text => self.text.sync(state.into()),
            SelectPart::Suffix => self.suffix.sync(state.into()),
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
        icon => ICON,
        text => TEXT,
        suffix => SUFFIX
    }
}

impl TryFrom<(&Item, SelectState)> for SelectBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, SelectState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.select.item.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(SelectBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(SelectBasicStyle::default_icon(Theme::default(), state)),
            |v| (v, SvgState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(SelectBasicStyle::default_text(Theme::default(), state)),
            |v| (v, LabelState::from(state)).try_into(),
        )?;

        let suffix = get_from_itable(
            inline_table,
            SUFFIX,
            || Ok(SelectBasicStyle::default_suffix(Theme::default(), state)),
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

impl SelectBasicStyle {
    pub fn default_container(theme: Theme, state: SelectState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(MouseCursor::Hand);
        container.set_background_visible(true);
        container.set_flow(Flow::Right);
        container.set_height(Size::Fixed(32.0));
        container.set_width(Size::Fill);
        container.set_align(Align { x: 0.0, y: 0.5 });
        container.set_spacing(12.0);
        container.set_border_radius(Radius::new(2.0));
        container
    }
    pub fn default_icon(theme: Theme, state: SelectState) -> SvgBasicStyle {
        let mut icon = SvgBasicStyle::from_state(theme, state.into());
        icon.container.height = Size::Fill;
        icon
    }
    pub fn default_text(theme: Theme, state: SelectState) -> LabelBasicStyle {
        let mut label = LabelBasicStyle::from_state(theme, state.into());
        label.width = Size::Fill;
        label
    }
    pub fn default_suffix(theme: Theme, state: SelectState) -> SvgBasicStyle {
        SvgBasicStyle::from_state(theme, state.into())
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
        Icon => icon => ICON,
        Text => text => TEXT,
        Suffix => suffix => SUFFIX
    }, SelectState
}
