use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    basic_prop_interconvert, component_color, component_part,
    components::{
        InputState, LabelBasicStyle, ViewColors,
        live_props::LiveProps,
        traits::{BasicStyle, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys, Radius,
        manuel::{
            BASIC, COLOR, CONTAINER, CURSOR, DISABLED, EMPTY, FOCUS, HOVER, PLACEHOLDER, SELECTION,
            TEXT, THEME,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
    },
    prop_interconvert, state_color,
    themes::{ColorFontConf, Theme},
    utils::get_from_itable,
};

prop_interconvert! {
    InputAreaStyle {
        basic_prop = InputAreaBasicStyle;
        basic => BASIC, InputAreaBasicStyle::default(), |v| (v, InputState::Basic).try_into(),
        hover => HOVER, InputAreaBasicStyle::from_state(Theme::default(), InputState::Hover), |v| (v, InputState::Hover).try_into(),
        focus => FOCUS, InputAreaBasicStyle::from_state(Theme::default(), InputState::Focus), |v| (v, InputState::Focus).try_into(),
        empty => EMPTY, InputAreaBasicStyle::from_state(Theme::default(), InputState::Empty), |v| (v, InputState::Empty).try_into(),
        disabled => DISABLED, InputAreaBasicStyle::from_state(Theme::default(), InputState::Disabled), |v| (v, InputState::Disabled).try_into()
    }, "[component.input.input] should be a table"
}

impl Style for InputAreaStyle {
    type State = InputState;

    type Basic = InputAreaBasicStyle;

    get_get_mut! {
        InputState::Basic => basic,
        InputState::Empty => empty,
        InputState::Hover => hover,
        InputState::Focus => focus,
        InputState::Disabled => disabled
    }

    fn len() -> usize {
        5 * InputAreaBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            InputState::Basic,
            [
                (InputState::Empty, &mut self.empty),
                (InputState::Hover, &mut self.hover),
                (InputState::Focus, &mut self.focus),
                (InputState::Disabled, &mut self.disabled),
            ],
        );
    }
}

impl SlotStyle for InputAreaStyle {
    type Part = InputAreaPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            InputState::Basic,
            [
                (InputState::Empty, &mut self.empty),
                (InputState::Hover, &mut self.hover),
                (InputState::Focus, &mut self.focus),
                (InputState::Disabled, &mut self.disabled),
            ],
            [
                InputAreaPart::Container,
                InputAreaPart::Text,
                InputAreaPart::Cursor,
                InputAreaPart::Selection,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct InputAreaBasicStyle {
    #[live(InputAreaBasicStyle::default_container(Theme::default(), InputState::Basic))]
    pub container: ViewBasicStyle,
    #[live(InputAreaBasicStyle::default_text(Theme::default(), InputState::Basic))]
    pub text: LabelBasicStyle,
    #[live(InputAreaBasicStyle::default_cursor(Theme::default(), InputState::Basic))]
    pub cursor: CursorBasicStyle,
    #[live(InputAreaBasicStyle::default_selection(Theme::default(), InputState::Basic))]
    pub selection: SelectionBasicStyle,
}

impl BasicStyle for InputAreaBasicStyle {
    type State = InputState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            text: Self::default_text(theme, state),
            cursor: Self::default_cursor(theme, state),
            selection: Self::default_selection(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicStyle::state_colors(theme, state.into())
    }

    fn len() -> usize {
        3 * ViewBasicStyle::len()
            + LabelBasicStyle::len()
            + CursorBasicStyle::len()
            + SelectionBasicStyle::len()
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.container.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(text), LabelBasicStyle::live_props().into()),
            (live_id!(cursor), CursorBasicStyle::live_props().into()),
            (
                live_id!(selection),
                SelectionBasicStyle::live_props().into(),
            ),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for InputAreaBasicStyle {
    type Part = InputAreaPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            InputAreaPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            InputAreaPart::Text => self.text.set_from_str(key, &value.into(), state.into()),
            InputAreaPart::Cursor => self.cursor.set_from_str(key, &value.into(), state),
            InputAreaPart::Selection => self.selection.set_from_str(key, &value.into(), state),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            InputAreaPart::Container => self.container.sync(state.into()),
            InputAreaPart::Text => self.text.sync(state.into()),
            InputAreaPart::Cursor => self.cursor.sync(state),
            InputAreaPart::Selection => self.selection.sync(state),
        }
    }
}

impl Default for InputAreaBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), InputState::Basic)
    }
}

from_prop_to_toml! {
    InputAreaBasicStyle {
        container => CONTAINER,
        text => TEXT,
        cursor => CURSOR,
        selection => SELECTION
    }
}

impl TryFrom<(&InlineTable, InputState)> for InputAreaBasicStyle {
    type Error = Error;
    
    fn try_from((inline_table, state): (&InlineTable, InputState)) -> Result<Self, Self::Error> {
        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(InputAreaBasicStyle::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(InputAreaBasicStyle::default_text(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        let cursor = get_from_itable(
            inline_table,
            CURSOR,
            || Ok(InputAreaBasicStyle::default_cursor(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;

        let selection = get_from_itable(
            inline_table,
            SELECTION,
            || {
                Ok(InputAreaBasicStyle::default_selection(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, state).try_into(),
        )?;

        Ok(Self {
            container,
            text,
            cursor,
            selection,
        })
    }
    
}

impl TryFrom<(&Value, InputState)> for InputAreaBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Value, InputState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.input.input.$slot] should be an inline table".to_string(),
        ))?;

        Self::try_from((inline_table, state))
    }
}

impl TryFrom<(&Item, InputState)> for InputAreaBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, InputState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.input.input.$slot] should be an inline table".to_string(),
        ))?;

        Self::try_from((inline_table, state))
    }
}

impl InputAreaBasicStyle {
    pub fn default_container(theme: Theme, state: InputState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(MouseCursor::Text);
        container.set_background_visible(true);
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_border_radius(Radius::new(2.0));
        container.set_flow(Flow::Right);
        container.set_margin(Margin::from_f64(0.0));
        container
    }

    pub fn default_text(theme: Theme, state: InputState) -> LabelBasicStyle {
        let mut text = LabelBasicStyle::from_state(theme, state.into());
        if state == InputState::Empty {
            text.set_color(ColorFontConf::from_key(PLACEHOLDER).into());
        }
        text
    }

    pub fn default_cursor(theme: Theme, state: InputState) -> CursorBasicStyle {
        CursorBasicStyle::from_state(theme, state)
    }

    pub fn default_selection(theme: Theme, state: InputState) -> SelectionBasicStyle {
        SelectionBasicStyle::from_state(theme, state)
    }
}

basic_prop_interconvert! {
    CursorBasicStyle {
        state = InputState;
        {
            color => COLOR, |v| v.try_into()
        };
        {}
    }, "CursorBasicStyle should be a inline table"
}

component_color! {
    CursorColors {
        colors = (Color);
        color
    }
}

impl BasicStyle for CursorBasicStyle {
    type State = InputState;

    type Colors = CursorColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        Self {
            theme,
            color: Self::state_colors(theme, state).color.into(),
        }
    }

    state_color! {
        (color),
        InputState::Basic => (300),
        InputState::Hover => (300),
        InputState::Focus => (300),
        InputState::Empty => (300),
        InputState::Disabled => (200)
    }

    fn len() -> usize {
        2
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.color.into());
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let CursorColors { color } = Self::state_colors(self.theme, state);
        self.color = color.into();
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(color), None.into()),
        ]
    }

    fn walk(&self) -> Walk {
        Walk::default()
    }

    fn layout(&self) -> Layout {
        Layout::default()
    }
}

basic_prop_interconvert! {
    SelectionBasicStyle {
        state = InputState;
        {
            color => COLOR, |v| v.try_into()
        };
        {}
    }, "SelectionBasicStyle should be a inline table"
}

component_color! {
    SelectionColors {
        colors = (Color);
        color
    }
}

impl BasicStyle for SelectionBasicStyle {
    type State = InputState;

    type Colors = SelectionColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        Self {
            theme,
            color: Self::state_colors(theme, state).color.into(),
        }
    }

    state_color! {
        (color),
        InputState::Basic => (200),
        InputState::Hover => (200),
        InputState::Focus => (200),
        InputState::Empty => (200),
        InputState::Disabled => (100)
    }

    fn len() -> usize {
        2
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.color.into());
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let SelectionColors { color } = Self::state_colors(self.theme, state);
        self.color = color.into();
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(color), None.into()),
        ]
    }

    fn walk(&self) -> Walk {
        Walk::default()
    }

    fn layout(&self) -> Layout {
        Layout::default()
    }
}

component_part! {
    InputAreaPart {
        Container => container => CONTAINER,
        Text => text => TEXT,
        Cursor => cursor => CURSOR,
        Selection => selection => SELECTION
    }, InputState
}
