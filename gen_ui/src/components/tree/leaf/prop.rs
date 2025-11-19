use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        ViewColors,
        label::{LabelBasicStyle, LabelState},
        live_props::LiveProps,
        svg::{SvgBasicStyle, SvgPart, SvgState},
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, Applys,
        manuel::{ACTIVE, BASIC, CONTAINER, DISABLED, HOVER, ICON, TEXT},
        traits::NewFrom,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    LeafStyle {
        basic_prop = LeafBasicStyle;
        basic => BASIC, LeafBasicStyle::default(), |v| (v, LeafState::Basic).try_into(),
        hover => HOVER, LeafBasicStyle::from_state(Theme::default(), LeafState::Hover), |v| (v, LeafState::Hover).try_into(),
        active => ACTIVE, LeafBasicStyle::from_state(Theme::default(), LeafState::Active), |v| (v, LeafState::Active).try_into(),
        disabled => DISABLED, LeafBasicStyle::from_state(Theme::default(), LeafState::Disabled), |v| (v, LeafState::Disabled).try_into()
    }, "[component.menu_item] should be a table"
}

impl Style for LeafStyle {
    type State = LeafState;

    type Basic = LeafBasicStyle;

    get_get_mut! {
        LeafState::Basic => basic,
        LeafState::Hover => hover,
        LeafState::Active => active,
        LeafState::Disabled => disabled
    }

    fn len() -> usize {
        4 * LeafBasicStyle::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

impl SlotStyle for LeafStyle {
    type Part = LeafPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            LeafState::Basic,
            [
                (LeafState::Hover, &mut self.hover),
                (LeafState::Active, &mut self.active),
                (LeafState::Disabled, &mut self.disabled),
            ],
            [LeafPart::Container, LeafPart::Icon, LeafPart::Text],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct LeafBasicStyle {
    #[live(LeafBasicStyle::default_container(Theme::default(), LeafState::Basic))]
    pub container: ViewBasicStyle,
    #[live(LeafBasicStyle::default_icon(Theme::default(), LeafState::Basic))]
    pub icon: SvgBasicStyle,
    #[live(LeafBasicStyle::default_text(Theme::default(), LeafState::Basic))]
    pub text: LabelBasicStyle,
}

from_prop_to_toml! {
    LeafBasicStyle {
        container => CONTAINER,
        icon => ICON,
        text => TEXT
    }
}

impl BasicStyle for LeafBasicStyle {
    type State = LeafState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            icon: Self::default_icon(theme, state),
            text: Self::default_text(theme, state),
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
        self.icon.sync(state.into());
        self.text.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(icon), SvgBasicStyle::live_props().into()),
            (live_id!(text), LabelBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for LeafBasicStyle {
    type Part = LeafPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            LeafPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            LeafPart::Icon => {
                let icon_part = SvgPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.icon
                        .set_from_str_slot(key, value, state.into(), icon_part);
                }
            }
            LeafPart::Text => {
                self.text.set_from_str(key, &value.into(), state.into());
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            LeafPart::Container => self.container.sync(state.into()),
            LeafPart::Icon => {
                self.icon.sync_slot(state.into(), SvgPart::Svg);
                self.icon.sync_slot(state.into(), SvgPart::Container);
            }
            LeafPart::Text => self.text.sync(state.into()),
        }
    }
}

impl Default for LeafBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), LeafState::Basic)
    }
}

impl TryFrom<(&Item, LeafState)> for LeafBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, LeafState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(LeafBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(LeafBasicStyle::default_icon(Theme::default(), state)),
            |v| (v, SvgState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(LeafBasicStyle::default_text(Theme::default(), state)),
            |v| (v, LabelState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            icon,
            text,
        })
    }
}

impl LeafBasicStyle {
    pub fn default_container(theme: Theme, state: LeafState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_background_visible(false);
        container.set_flow(Flow::Right);
        container.set_cursor(MouseCursor::Hand);
        container.set_margin(Margin::from_f64(0.0));
        container.set_padding(Padding::from_f64(0.0));
        container
    }
    pub fn default_icon(theme: Theme, state: LeafState) -> SvgBasicStyle {
        let icon = SvgBasicStyle::from_state(theme, state.into());
        icon
    }
    pub fn default_text(theme: Theme, state: LeafState) -> LabelBasicStyle {
        LabelBasicStyle::from_state(theme, state.into())
    }
    pub fn default_extra(theme: Theme, state: LeafState) -> ViewBasicStyle {
        let mut extra = ViewBasicStyle::from_state(theme, state.into());
        extra.set_height(Size::Fill);
        extra.set_width(Size::Fit);
        extra
    }
}

component_state! {
    LeafState {
        Basic => BASIC,
        Hover => HOVER,
        Active => ACTIVE,
        Disabled => DISABLED
    }, _ => LeafState::Basic
}

impl ComponentState for LeafState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<LeafState> for ViewState {
    fn from(value: LeafState) -> Self {
        match value {
            LeafState::Basic => ViewState::Basic,
            LeafState::Hover => ViewState::Hover,
            LeafState::Active => ViewState::Pressed,
            LeafState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for LeafState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => LeafState::Basic,
            ViewState::Hover => LeafState::Hover,
            ViewState::Pressed => LeafState::Active,
            ViewState::Disabled => LeafState::Disabled,
        }
    }
}

impl From<LeafState> for SvgState {
    fn from(value: LeafState) -> Self {
        match value {
            LeafState::Basic => SvgState::Basic,
            LeafState::Hover => SvgState::Hover,
            LeafState::Active => SvgState::Pressed,
            LeafState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<LeafState> for LabelState {
    fn from(value: LeafState) -> Self {
        match value {
            LeafState::Basic | LeafState::Hover | LeafState::Active => LabelState::Basic,
            LeafState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    LeafPart {
        Container => container => CONTAINER,
        Icon => icon => ICON,
        Text => text => TEXT
    }, LeafState
}
