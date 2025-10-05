// use std::str::FromStr;

// use makepad_widgets::*;
// use toml_edit::Item;

// use crate::{
//     component_part, component_state,
//     components::{
//         label::{LabelBasicStyle, LabelState},
//         live_props::LiveProps,
//         svg::{SvgBasicStyle, SvgPart, SvgState},
//         traits::{BasicStyle, ComponentState, Part, SlotBasicStyle, SlotStyle, Style},
//         view::{ViewBasicStyle, ViewState},
//     },
//     error::Error,
//     from_prop_to_toml, get_get_mut,
//     prop::{
//         ApplySlotMapImpl, Radius,
//         manuel::{BASIC, CLOSE, CONTAINER, DISABLED, HOVER, ICON, PRESSED, TEXT},
//         traits::NewFrom,
//     },
//     prop_interconvert,
//     themes::Theme,
//     utils::get_from_itable,
// };

// prop_interconvert! {
//     BadgeProp {
//         basic_prop = BadgeBasicStyle;
//         basic => BASIC, BadgeBasicStyle::default(), |v| (v, BadgeState::Basic).try_into(),
//         disabled => DISABLED, BadgeBasicStyle::from_state(Theme::default(), BadgeState::Disabled), |v| (v, BadgeState::Disabled).try_into()
//     }, "[component.badge.dot] should be a table"
// }

// impl SlotStyle for BadgeProp {
//     type Part = BadgePart;

//     fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
//         map.sync(
//             &mut self.basic,
//             BadgeState::Basic,
//             [(BadgeState::Disabled, &mut self.disabled)],
//             [
//                 BadgePart::Container,
//                 BadgePart::Icon,
//                 BadgePart::Text,
//                 BadgePart::Close,
//             ],
//         );
//     }
// }

// impl Style for BadgeProp {
//     type State = BadgeState;

//     type Basic = BadgeBasicStyle;

//     get_get_mut! {
//         BadgeState::Basic => basic,
//         BadgeState::Disabled => disabled
//     }

//     fn len() -> usize {
//         2 * BadgeBasicStyle::len()
//     }

//     fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
//     where
//         Self::State: Eq + std::hash::Hash + Copy,
//     {
//         ()
//     }
// }

// #[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
// #[live_ignore]
// pub struct BadgeBasicStyle {
//     #[live(BadgeBasicStyle::default_text(Theme::default(), BadgeState::default()))]
//     pub text: LabelBasicStyle,
//     #[live(BadgeBasicStyle::default_container(Theme::default(), BadgeState::default()))]
//     pub container: ViewBasicStyle,
// }

// impl Default for BadgeBasicStyle {
//     fn default() -> Self {
//         Self::from_state(Theme::default(), BadgeState::default())
//     }
// }

// impl SlotBasicStyle for BadgeBasicStyle {
//     type Part = BadgePart;

//     fn set_from_str_slot(
//         &mut self,
//         key: &str,
//         value: &crate::prop::Applys,
//         state: Self::State,
//         part: Self::Part,
//     ) -> () {
//         match part {
//             BadgePart::Container => self
//                 .container
//                 .set_from_str(key, &value.into(), state.into()),
//             BadgePart::Icon => {
//                 // if is slot, key is part, value is key + value
//                 let icon_part = SvgPart::from_str(key).unwrap();
//                 for (key, value) in value.as_kvs() {
//                     self.icon
//                         .set_from_str_slot(key, value, state.into(), icon_part);
//                 }
//             }
//             BadgePart::Text => self.text.set_from_str(key, &value.into(), state.into()),
//             BadgePart::Close => {
//                 let close_part = SvgPart::from_str(key).unwrap();
//                 for (key, value) in value.as_kvs() {
//                     self.close
//                         .set_from_str_slot(key, value, state.into(), close_part);
//                 }
//             }
//         }
//     }

//     fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
//         match part {
//             BadgePart::Container => self.container.sync(state.into()),
//             BadgePart::Text => self.text.sync(state.into()),
//         }
//     }
// }

// impl BasicStyle for BadgeBasicStyle {
//     type State = BadgeState;

//     type Colors = ();

//     fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
//         Self {
//             text: Self::default_text(theme, state),
//             container: Self::default_container(theme, state),
//         }
//     }

//     fn state_colors(_theme: crate::themes::Theme, _state: Self::State) -> Self::Colors {
//         ()
//     }

//     fn len() -> usize {
//         ViewBasicStyle::len() + SvgBasicStyle::len() + LabelBasicStyle::len() + SvgBasicStyle::len()
//     }

//     fn set_from_str(
//         &mut self,
//         _key: &str,
//         _value: &makepad_widgets::LiveValue,
//         _state: Self::State,
//     ) -> () {
//         ()
//     }

//     fn sync(&mut self, state: Self::State) -> () {
//         self.container.sync(state.into());
//         self.text.sync(state.into());
//     }

//     fn live_props() -> LiveProps {
//         vec![
//             (live_id!(text), LabelBasicStyle::live_props().into()),
//             (live_id!(container), ViewBasicStyle::live_props().into()),
//         ]
//     }

//     fn walk(&self) -> Walk {
//         self.container.walk()
//     }

//     fn layout(&self) -> Layout {
//         self.container.layout()
//     }
// }

// from_prop_to_toml! {
//     BadgeBasicStyle {
//         text => TEXT,
//         container => CONTAINER
//     }
// }

// impl TryFrom<(&Item, BadgeState)> for BadgeBasicStyle {
//     type Error = Error;

//     fn try_from((value, state): (&Item, BadgeState)) -> Result<Self, Self::Error> {
//         let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
//             "[component.badge.dot.$slot] should be an inline table".to_string(),
//         ))?;

//         let container = get_from_itable(
//             inline_table,
//             CONTAINER,
//             || Ok(BadgeBasicStyle::default_container(Theme::default(), state)),
//             |v| (v, ViewState::from(state)).try_into(),
//         )?;

//         let text = get_from_itable(
//             inline_table,
//             TEXT,
//             || Ok(Self::default_text(Theme::default(), state)),
//             |v| (v, state.into()).try_into(),
//         )?;

//         Ok(Self { text, container })
//     }
// }

// impl BadgeBasicStyle {
//     pub fn default_container(theme: Theme, state: BadgeState) -> ViewBasicStyle {
//         let mut container = ViewBasicStyle::from_state(theme, state.into());
//         container.height = Size::Fit;
//         container.width = Size::Fit;
//         container.align = Align::from_f64(0.5);
//         container.background_visible = true;
//         container.set_padding(Padding::from_all(4.0, 8.0, 4.0, 8.0));
//         container.set_border_radius(Radius::new(2.0));
//         container.set_flow(Flow::Right);
//         container.set_spacing(4.0);
//         container
//     }

//     pub fn default_text(theme: Theme, state: BadgeState) -> LabelBasicStyle {
//         let mut text = LabelBasicStyle::from_state(theme, state.into());
//         text.flow = Flow::Right;
//         text.set_font_size(10.0);
//         text
//     }
// }

// component_state! {
//     BadgeState {
//         Basic => BASIC,
//         Disabled => DISABLED
//     },
//     _ => BadgeState::Basic
// }

// impl ComponentState for BadgeState {
//     fn is_disabled(&self) -> bool {
//         matches!(self, BadgeState::Disabled)
//     }
// }

// impl From<BadgeState> for ViewState {
//     fn from(value: BadgeState) -> Self {
//         match value {
//             BadgeState::Basic => ViewState::Basic,
//             BadgeState::Disabled => ViewState::Disabled,
//         }
//     }
// }

// impl From<BadgeState> for LabelState {
//     fn from(value: BadgeState) -> Self {
//         match value {
//             BadgeState::Basic => LabelState::Basic,
//             BadgeState::Disabled => LabelState::Disabled,
//         }
//     }
// }

// component_part! {
//     BadgePart {
//         Text => text => TEXT,
//         Container => container => CONTAINER
//     }, BadgeState
// }
