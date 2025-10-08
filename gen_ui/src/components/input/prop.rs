use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    basic_prop_interconvert, component_color, component_part, component_state,
    components::{
        LabelBasicStyle, LabelState, SelectBasicStyle, ViewColors,
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, SlotBasicStyle, SlotStyle, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        ApplySlotMapImpl, ApplyStateMapImpl, Applys, Radius,
        manuel::{
            BASIC, BORDER_RADIUS, COLOR, CONTAINER, CURSOR, DISABLED, EMPTY, FOCUS, HOVER, PREFIX,
            SELECTION, SUFFIX, TEXT,
        },
        traits::NewFrom,
    },
    prop_interconvert, state_color,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    InputStyle {
        basic_prop = InputBasicStyle;
        basic => BASIC, InputBasicStyle::default(), |v| (v, InputState::Basic).try_into(),
        hover => HOVER, InputBasicStyle::from_state(Theme::default(), InputState::Hover), |v| (v, InputState::Hover).try_into(),
        focus => FOCUS, InputBasicStyle::from_state(Theme::default(), InputState::Focus), |v| (v, InputState::Focus).try_into(),
        empty => EMPTY, InputBasicStyle::from_state(Theme::default(), InputState::Empty), |v| (v, InputState::Empty).try_into(),
        disabled => DISABLED, InputBasicStyle::from_state(Theme::default(), InputState::Disabled), |v| (v, InputState::Disabled).try_into()
    }, "[component.input] should be a table"
}

impl Style for InputStyle {
    type State = InputState;

    type Basic = InputBasicStyle;

    get_get_mut! {
        InputState::Basic => basic,
        InputState::Empty => empty,
        InputState::Hover => hover,
        InputState::Focus => focus,
        InputState::Disabled => disabled
    }

    fn len() -> usize {
        5 * InputBasicStyle::len()
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

impl SlotStyle for InputStyle {
    type Part = InputPart;

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
            [InputPart::Container, InputPart::Prefix, InputPart::Suffix],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct InputBasicStyle {
    #[live(InputBasicStyle::default_container(Theme::default(), InputState::Basic))]
    pub container: ViewBasicStyle,
    #[live(InputBasicStyle::default_prefix(Theme::default(), InputState::Basic))]
    pub prefix: ViewBasicStyle,
    #[live(InputBasicStyle::default_suffix(Theme::default(), InputState::Basic))]
    pub suffix: ViewBasicStyle,
    #[live(InputBasicStyle::default_text(Theme::default(), InputState::Basic))]
    pub text: LabelBasicStyle,
    #[live(InputBasicStyle::default_cursor(Theme::default(), InputState::Basic))]
    pub cursor: CursorBasicStyle,
    #[live(InputBasicStyle::default_selection(Theme::default(), InputState::Basic))]
    pub selection: SelectionBasicStyle,
}

impl BasicStyle for InputBasicStyle {
    type State = InputState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            prefix: Self::default_prefix(theme, state),
            suffix: Self::default_suffix(theme, state),
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
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.container.sync(state.into());
        self.prefix.sync(state.into());
        self.suffix.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
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

impl SlotBasicStyle for InputBasicStyle {
    type Part = InputPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            InputPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            InputPart::Prefix => self.prefix.set_from_str(key, &value.into(), state.into()),
            InputPart::Suffix => self.suffix.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            InputPart::Container => self.container.sync(state.into()),
            InputPart::Prefix => self.prefix.sync(state.into()),
            InputPart::Suffix => self.suffix.sync(state.into()),
        }
    }
}

impl Default for InputBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), InputState::Basic)
    }
}

from_prop_to_toml! {
    InputBasicStyle {
        container => CONTAINER,
        prefix => PREFIX,
        suffix => SUFFIX
    }
}

impl TryFrom<(&Item, InputState)> for InputBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, InputState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.badge.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(InputBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let prefix = get_from_itable(
            inline_table,
            PREFIX,
            || Ok(InputBasicStyle::default_prefix(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let suffix = get_from_itable(
            inline_table,
            SUFFIX,
            || Ok(InputBasicStyle::default_suffix(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(InputBasicStyle::default_text(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        let cursor = get_from_itable(
            inline_table,
            CURSOR,
            || Ok(InputBasicStyle::default_cursor(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;

        let selection = get_from_itable(
            inline_table,
            SELECTION,
            || Ok(InputBasicStyle::default_selection(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;

        Ok(Self {
            container,
            prefix,
            suffix,
            text,
            cursor,
            selection,
        })
    }
}

impl InputBasicStyle {
    pub fn default_container(theme: Theme, state: InputState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_background_visible(true);
        container.set_height(Size::Fixed(36.0));
        container.set_width(Size::Fill);
        container
    }

    pub fn default_prefix(theme: Theme, state: InputState) -> ViewBasicStyle {
        let mut prefix = ViewBasicStyle::from_state(theme, state.into());
        prefix.set_background_visible(false);
        prefix.set_padding(Padding::from_f64(0.0));
        prefix.set_height(Size::Fill);
        prefix.set_width(Size::Fit);
        prefix
    }

    pub fn default_suffix(theme: Theme, state: InputState) -> ViewBasicStyle {
        Self::default_prefix(theme, state)
    }

    pub fn default_text(theme: Theme, state: InputState) -> LabelBasicStyle {
        let mut text = LabelBasicStyle::from_state(theme, state.into());
        text.set_font_size(14.0);
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
    CursorBasicColors {
        colors = (Color);
        color
    }
}

impl BasicStyle for CursorBasicStyle {
    type State = InputState;

    type Colors = CursorBasicColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        Self {
            theme,
            color: Self::state_colors(theme, state).color.into(),
        }
    }

    state_color! {
        (color),
        InputState::Basic => (500),
        InputState::Hover => (500),
        InputState::Focus => (500),
        InputState::Empty => (500),
        InputState::Disabled => (300)
    }

    fn len() -> usize {
        2
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        todo!()
    }

    fn sync(&mut self, state: Self::State) -> () {
        todo!()
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
        {
            border_radius: Radius => BORDER_RADIUS, Radius::new(2.0), |v| v.try_into()
        }
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
        todo!()
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        todo!()
    }

    fn len() -> usize {
        todo!()
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        todo!()
    }

    fn sync(&mut self, state: Self::State) -> () {
        todo!()
    }

    fn live_props() -> LiveProps {
        todo!()
    }

    fn walk(&self) -> Walk {
        todo!()
    }

    fn layout(&self) -> Layout {
        todo!()
    }
}

component_state! {
    InputState {
        Basic => BASIC,
        Empty => EMPTY,
        Hover => HOVER,
        Focus => FOCUS,
        Disabled => DISABLED
    }, _ => InputState::Basic
}

impl ComponentState for InputState {
    fn is_disabled(&self) -> bool {
        matches!(self, InputState::Disabled)
    }
}

impl From<InputState> for ViewState {
    fn from(value: InputState) -> Self {
        match value {
            InputState::Basic | InputState::Empty => ViewState::Basic,
            InputState::Disabled => ViewState::Disabled,
            InputState::Hover => ViewState::Hover,
            InputState::Focus => ViewState::Pressed,
        }
    }
}

impl From<InputState> for LabelState {
    fn from(value: InputState) -> Self {
        match value {
            InputState::Disabled => LabelState::Disabled,
            _ => LabelState::Basic,
        }
    }
}

component_part! {
    InputPart {
        Container => container => CONTAINER,
        Prefix => prefix => PREFIX,
        Suffix => suffix => SUFFIX
    }, InputState
}
