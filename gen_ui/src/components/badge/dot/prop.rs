use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        label::{LabelBasicStyle, LabelState},
        live_props::LiveProps,
        svg::{SvgBasicStyle, SvgPart, SvgState},
        traits::{BasicStyle, ComponentState, Part, Style, SlotBasicStyle, SlotStyle},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        manuel::{BASIC, CLOSE, CONTAINER, DISABLED, HOVER, ICON, PRESSED, TEXT},
        traits::NewFrom,
        ApplySlotMapImpl, Radius,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    BadgeDotProp {
        basic_prop = BadgeDotBasicStyle;
        basic => BASIC, BadgeDotBasicStyle::default(), |v| (v, BadgeDotState::Basic).try_into(),
        hover => HOVER, BadgeDotBasicStyle::from_state(Theme::default(), BadgeDotState::Hover), |v| (v, BadgeDotState::Hover).try_into(),
        pressed => PRESSED, BadgeDotBasicStyle::from_state(Theme::default(), BadgeDotState::Pressed), |v| (v, BadgeDotState::Pressed).try_into(),
        disabled => DISABLED, BadgeDotBasicStyle::from_state(Theme::default(), BadgeDotState::Disabled), |v| (v, BadgeDotState::Disabled).try_into()
    }, "[component.tag] should be a table"
}

impl SlotStyle for BadgeDotProp {
    type Part = BadgeDotPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            BadgeDotState::Basic,
            [
                (BadgeDotState::Hover, &mut self.hover),
                (BadgeDotState::Pressed, &mut self.pressed),
                (BadgeDotState::Disabled, &mut self.disabled),
            ],
            [
                BadgeDotPart::Container,
                BadgeDotPart::Icon,
                BadgeDotPart::Text,
                BadgeDotPart::Close,
            ],
        );
    }
}

impl Style for BadgeDotProp {
    type State = BadgeDotState;

    type Basic = BadgeDotBasicStyle;

    get_get_mut! {
        BadgeDotState::Basic => basic,
        BadgeDotState::Hover => hover,
        BadgeDotState::Pressed => pressed,
        BadgeDotState::Disabled => disabled
    }

    fn len() -> usize {
        4 * BadgeDotBasicStyle::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct BadgeDotBasicStyle {
    #[live(BadgeDotBasicStyle::default_icon(Theme::default(), BadgeDotState::default()))]
    pub icon: SvgBasicStyle,
    #[live(BadgeDotBasicStyle::default_text(Theme::default(), BadgeDotState::default()))]
    pub text: LabelBasicStyle,
    #[live(BadgeDotBasicStyle::default_close(Theme::default(), BadgeDotState::default()))]
    pub close: SvgBasicStyle,
    #[live(BadgeDotBasicStyle::default_container(Theme::default(), BadgeDotState::default()))]
    pub container: ViewBasicStyle,
}

impl Default for BadgeDotBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), BadgeDotState::default())
    }
}

impl SlotBasicStyle for BadgeDotBasicStyle {
    type Part = BadgeDotPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            BadgeDotPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            BadgeDotPart::Icon => {
                // if is slot, key is part, value is key + value
                let icon_part = SvgPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.icon
                        .set_from_str_slot(key, value, state.into(), icon_part);
                }
            }
            BadgeDotPart::Text => self.text.set_from_str(key, &value.into(), state.into()),
            BadgeDotPart::Close => {
                let close_part = SvgPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.close
                        .set_from_str_slot(key, value, state.into(), close_part);
                }
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            BadgeDotPart::Container => self.container.sync(state.into()),
            BadgeDotPart::Icon => {
                self.icon.sync_slot(state.into(), SvgPart::Svg);
                self.icon.sync_slot(state.into(), SvgPart::Container);
            }
            BadgeDotPart::Text => self.text.sync(state.into()),
            BadgeDotPart::Close => {
                self.close.sync_slot(state.into(), SvgPart::Svg);
                self.close.sync_slot(state.into(), SvgPart::Container);
            }
        }
    }
}

impl BasicStyle for BadgeDotBasicStyle {
    type State = BadgeDotState;

    type Colors = ();

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            icon: SvgBasicStyle::from_state(theme, state.into()),
            text: Self::default_text(theme, state),
            close: Self::default_close(theme, state),
            container: Self::default_container(theme, state),
        }
    }

    fn state_colors(_theme: crate::themes::Theme, _state: Self::State) -> Self::Colors {
        ()
    }

    fn len() -> usize {
        ViewBasicStyle::len() + SvgBasicStyle::len() + LabelBasicStyle::len() + SvgBasicStyle::len()
    }

    fn set_from_str(
        &mut self,
        _key: &str,
        _value: &makepad_widgets::LiveValue,
        _state: Self::State,
    ) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.container.sync(state.into());
        self.icon.sync(state.into());
        self.text.sync(state.into());
        self.close.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(icon), SvgBasicStyle::live_props().into()),
            (live_id!(text), LabelBasicStyle::live_props().into()),
            (live_id!(close), SvgBasicStyle::live_props().into()),
            (live_id!(container), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }

    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

from_prop_to_toml! {
    BadgeDotBasicStyle {
        icon => ICON,
        text => TEXT,
        close => CLOSE,
        container => CONTAINER
    }
}

impl TryFrom<(&Item, BadgeDotState)> for BadgeDotBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, BadgeDotState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.tag.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(BadgeDotBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(SvgBasicStyle::default()),
            |v| (v, state.into()).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(Self::default_text(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        let close = get_from_itable(
            inline_table,
            CLOSE,
            || Ok(Self::default_close(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self {
            icon,
            text,
            close,
            container,
        })
    }
}

impl BadgeDotBasicStyle {
    pub fn default_container(theme: Theme, state: BadgeDotState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.height = Size::Fit;
        container.width = Size::Fit;
        container.align = Align::from_f64(0.5);
        container.background_visible = true;
        container.set_padding(Padding::from_all(4.0, 8.0, 4.0, 8.0));
        container.set_border_radius(Radius::new(2.0));
        container.set_flow(Flow::Right);
        container.set_spacing(4.0);
        container
    }

    pub fn default_text(theme: Theme, state: BadgeDotState) -> LabelBasicStyle {
        let mut text = LabelBasicStyle::from_state(theme, state.into());
        text.flow = Flow::Right;
        text.set_font_size(10.0);
        text
    }

    pub fn default_close(theme: Theme, state: BadgeDotState) -> SvgBasicStyle {
        SvgBasicStyle::from_state(theme, state.into())
    }

    pub fn default_icon(theme: Theme, state: BadgeDotState) -> SvgBasicStyle {
        SvgBasicStyle::from_state(theme, state.into())
    }
}

component_state! {
    BadgeDotState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    },
    _ => BadgeDotState::Basic
}

impl ComponentState for BadgeDotState {
    fn is_disabled(&self) -> bool {
        matches!(self, BadgeDotState::Disabled)
    }
}

impl From<BadgeDotState> for ViewState {
    fn from(value: BadgeDotState) -> Self {
        match value {
            BadgeDotState::Basic => ViewState::Basic,
            BadgeDotState::Hover => ViewState::Hover,
            BadgeDotState::Pressed => ViewState::Pressed,
            BadgeDotState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for BadgeDotState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => BadgeDotState::Basic,
            ViewState::Hover => BadgeDotState::Hover,
            ViewState::Pressed => BadgeDotState::Pressed,
            ViewState::Disabled => BadgeDotState::Disabled,
        }
    }
}

impl From<BadgeDotState> for SvgState {
    fn from(value: BadgeDotState) -> Self {
        match value {
            BadgeDotState::Basic => SvgState::Basic,
            BadgeDotState::Hover => SvgState::Hover,
            BadgeDotState::Pressed => SvgState::Pressed,
            BadgeDotState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<BadgeDotState> for LabelState {
    fn from(value: BadgeDotState) -> Self {
        match value {
            BadgeDotState::Basic | BadgeDotState::Hover | BadgeDotState::Pressed => LabelState::Basic,
            BadgeDotState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    BadgeDotPart {
        Icon => icon => ICON,
        Text => text => TEXT,
        Close => close => CLOSE,
        Container => container => CONTAINER
    }, BadgeDotState
}
