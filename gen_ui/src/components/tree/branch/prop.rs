use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        LabelBasicStyle, SvgBasicStyle, ViewColors,
        label::LabelState,
        live_props::LiveProps,
        svg::SvgState,
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, Applys,
        manuel::{ACTIVE, BASIC, BODY, CONTAINER, DISABLED, HEADER, ICON, TEXT},
        traits::NewFrom,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    BranchStyle {
        basic_prop = BranchBasicStyle;
        basic => BASIC, BranchBasicStyle::default(), |v| (v, BranchState::Basic).try_into(),
        active => ACTIVE, BranchBasicStyle::from_state(Theme::default(), BranchState::Active), |v| (v, BranchState::Active).try_into(),
        disabled => DISABLED, BranchBasicStyle::from_state(Theme::default(), BranchState::Disabled), |v| (v, BranchState::Disabled).try_into()
    }, "[component.tree.branch] should be a table"
}

impl Style for BranchStyle {
    type State = BranchState;

    type Basic = BranchBasicStyle;

    get_get_mut! {
        BranchState::Basic => basic,
        BranchState::Active => active,
        BranchState::Disabled => disabled
    }

    fn len() -> usize {
        4 * BranchBasicStyle::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

impl SlotStyle for BranchStyle {
    type Part = BranchPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            BranchState::Basic,
            [
                (BranchState::Active, &mut self.active),
                (BranchState::Disabled, &mut self.disabled),
            ],
            [
                BranchPart::Container,
                BranchPart::Icon,
                BranchPart::Text,
                BranchPart::Body,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct BranchBasicStyle {
    #[live(BranchBasicStyle::default_container(Theme::default(), BranchState::Basic))]
    pub container: ViewBasicStyle,
    #[live(BranchBasicStyle::default_icon(Theme::default(), BranchState::Basic))]
    pub icon: SvgBasicStyle,
    #[live(BranchBasicStyle::default_text(Theme::default(), BranchState::Basic))]
    pub text: LabelBasicStyle,
    #[live(BranchBasicStyle::default_body(Theme::default(), BranchState::Basic))]
    pub body: ViewBasicStyle,
}

from_prop_to_toml! {
    BranchBasicStyle {
        container => CONTAINER,
        icon => ICON,
        text => TEXT,
        body => BODY
    }
}

impl BasicStyle for BranchBasicStyle {
    type State = BranchState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            icon: Self::default_icon(theme, state),
            text: Self::default_text(theme, state),
            body: Self::default_body(theme, state),
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
        self.container.sync(state.into());
        self.icon.sync(state.into());
        self.text.sync(state.into());
        self.body.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(icon), SvgBasicStyle::live_props().into()),
            (live_id!(text), LabelBasicStyle::live_props().into()),
            (live_id!(body), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for BranchBasicStyle {
    type Part = BranchPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            BranchPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            BranchPart::Icon => self.icon.set_from_str(key, &value.into(), state.into()),
            BranchPart::Text => self.text.set_from_str(key, &value.into(), state.into()),
            BranchPart::Body => {
                self.body.set_from_str(key, &value.into(), state.into());
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            BranchPart::Container => self.container.sync(state.into()),
            BranchPart::Icon => self.icon.sync(state.into()),
            BranchPart::Text => self.text.sync(state.into()),
            BranchPart::Body => self.body.sync(state.into()),
        }
    }
}

impl Default for BranchBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), BranchState::Basic)
    }
}

impl TryFrom<(&Item, BranchState)> for BranchBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, BranchState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(BranchBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(BranchBasicStyle::default_icon(Theme::default(), state)),
            |v| (v, SvgState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(BranchBasicStyle::default_text(Theme::default(), state)),
            |v| (v, LabelState::from(state)).try_into(),
        )?;

        let body = get_from_itable(
            inline_table,
            BODY,
            || Ok(BranchBasicStyle::default_body(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            icon,
            text,
            body,
        })
    }
}

impl BranchBasicStyle {
    pub fn default_container(theme: Theme, state: BranchState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_background_visible(true);
        container.set_flow(Flow::Down);
        container.set_margin(Margin::from_f64(0.0));
        container.set_padding(Padding::from_f64(0.0));
        container.set_spacing(0.0);
        container
    }
    pub fn default_icon(theme: Theme, state: BranchState) -> SvgBasicStyle {
        let mut icon = SvgBasicStyle::from_state(theme, state.into());
        icon.container.height = Size::Fixed(16.0);
        icon.container.width = Size::Fixed(16.0);
        icon
    }
    pub fn default_text(theme: Theme, state: BranchState) -> LabelBasicStyle {
        let mut text = LabelBasicStyle::from_state(theme, state.into());
        text.set_font_size(14.0);
        text
    }
    pub fn default_body(theme: Theme, state: BranchState) -> ViewBasicStyle {
        let mut body = ViewBasicStyle::from_state(theme, state.into());
        body.set_height(Size::Fit);
        body.set_width(Size::Fill);
        body.set_background_visible(true);
        body.set_margin(Margin::from_f64(0.0));
        body.set_flow(Flow::Down);
        body
    }
}

component_state! {
    BranchState {
        Basic => BASIC,
        Active => ACTIVE,
        Disabled => DISABLED
    }, _ => BranchState::Basic
}

impl ComponentState for BranchState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<BranchState> for ViewState {
    fn from(value: BranchState) -> Self {
        match value {
            BranchState::Basic => ViewState::Basic,
            BranchState::Active => ViewState::Pressed,
            BranchState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for BranchState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => BranchState::Basic,
            ViewState::Pressed => BranchState::Active,
            ViewState::Disabled => BranchState::Disabled,
            ViewState::Hover => BranchState::Basic,
        }
    }
}

impl From<BranchState> for SvgState {
    fn from(value: BranchState) -> Self {
        match value {
            BranchState::Basic => SvgState::Basic,
            BranchState::Active => SvgState::Pressed,
            BranchState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<BranchState> for LabelState {
    fn from(value: BranchState) -> Self {
        match value {
            BranchState::Basic | BranchState::Active => LabelState::Basic,
            BranchState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    BranchPart {
        Container => container => CONTAINER,
        Icon => icon => ICON,
        Text => text => TEXT,
        Body => body => BODY
    }, BranchState
}
