use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        ViewColors,
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys,
        manuel::{BASIC, BODY, CONTAINER},
        traits::NewFrom,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    TreeStyle {
        basic_prop = TreeBasicStyle;
        basic => BASIC, TreeBasicStyle::default(), |v| (v, TreeState::Basic).try_into()
    }, "[component.tree] should be a table"
}

impl Style for TreeStyle {
    type State = TreeState;

    type Basic = TreeBasicStyle;

    get_get_mut! {
        TreeState::Basic => basic
    }

    fn len() -> usize {
        2 * TreeBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, TreeState::Basic, []);
    }
}

impl SlotStyle for TreeStyle {
    type Part = TreePart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            TreeState::Basic,
            [],
            [
                TreePart::Container,
                TreePart::Body,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct TreeBasicStyle {
    #[live(TreeBasicStyle::default_container(Theme::default(), TreeState::Basic))]
    pub container: ViewBasicStyle,
    #[live(TreeBasicStyle::default_body(Theme::default(), TreeState::Basic))]
    pub body: ViewBasicStyle,
}

from_prop_to_toml! {
    TreeBasicStyle {
        container => CONTAINER,
        body => BODY
    }
}

impl BasicStyle for TreeBasicStyle {
    type State = TreeState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
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
        self.body.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
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

impl SlotBasicStyle for TreeBasicStyle {
    type Part = TreePart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            TreePart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            TreePart::Body => self.body.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            TreePart::Container => self.container.sync(state.into()),
            TreePart::Body => self.body.sync(state.into()),
        }
    }
}

impl Default for TreeBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), TreeState::Basic)
    }
}

impl TryFrom<(&Item, TreeState)> for TreeBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, TreeState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.tree.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(TreeBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let body = get_from_itable(
            inline_table,
            BODY,
            || Ok(TreeBasicStyle::default_body(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self { container, body })
    }
}

impl TreeBasicStyle {
    pub fn default_container(theme: Theme, state: TreeState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_width(Size::Fixed(300.0));
        container.set_height(Size::Fill);
        container.set_background_visible(false);
        container.set_clip_y(true);
        container.set_clip_x(true);
        container
    }
    pub fn default_body(theme: Theme, state: TreeState) -> ViewBasicStyle {
        let mut body = Self::default_container(theme, state);
        body.set_height(Size::Fill);
        body.set_width(Size::Fill);
        body.set_padding(Padding::from_f64(0.0));
        body.set_margin(Margin::from_f64(0.0));
        body
    }
}

component_state! {
    TreeState {
        Basic => BASIC
    }, _ => TreeState::Basic
}

impl ComponentState for TreeState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<TreeState> for ViewState {
    fn from(value: TreeState) -> Self {
        match value {
            TreeState::Basic => ViewState::Basic,
        }
    }
}

impl From<ViewState> for TreeState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => TreeState::Basic,
            _ => panic!("TreeState can only be Basic"),
        }
    }
}

component_part! {
    TreePart {
        Container => container => CONTAINER,
        Body => body => BODY
    }, TreeState
}
