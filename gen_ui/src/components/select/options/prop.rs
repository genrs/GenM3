use makepad_widgets::*;

use crate::{
    components::{
        live_props::LiveProps,
        popup::PopupState,
        traits::{BasicStyle, ComponentState, Style},
        view::ViewBasicStyle,
    },
    from_inherit_to_view_basic_style, inherits_view_basic_prop,
    prop::{
        ApplyStateMapImpl, Radius,
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CLIP_X, CLIP_Y, CURSOR, FLOW, HEIGHT, MARGIN, PADDING,
            ROTATION, SCALE, SHADOW_COLOR, SHADOW_OFFSET, SPACING, SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
    },
    prop_interconvert,
    themes::{Theme, TomlValueTo},
};

prop_interconvert! {
    SelectOptionsStyle {
        basic_prop = SelectOptionsBasicStyle;
        basic => BASIC, SelectOptionsBasicStyle::default(),|v| (v, PopupState::Basic).try_into()
    }, "[component.popup] should be a table"
}

impl Style for SelectOptionsStyle {
    type State = PopupState;

    type Basic = SelectOptionsBasicStyle;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            PopupState::Basic => &self.basic,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            PopupState::Basic => &mut self.basic,
        }
    }

    fn len() -> usize {
        SelectOptionsBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, PopupState::Basic, []);
    }
}

inherits_view_basic_prop! {
     SelectOptionsBasicStyle {
        border_width: 0.0,
        border_radius: Radius::new(2.0),
        spread_radius: 0.0,
        blur_radius: 0.0,
        shadow_offset: vec2(0.0, 0.0),
        background_visible: true,
        rotation: 0.0,
        scale: 1.0,
        padding: Padding::from_f64(4.0),
        margin: Margin::from_f64(0.0),
        clip_x: false,
        clip_y: false,
        align: Align::from_f64(0.5),
        cursor: MouseCursor::default(),
        flow: Flow::Down,
        spacing: 0.0,
        height: Size::Fit,
        width: Size::Fit,
        abs_pos: None,
    }, PopupState, "select.options",
    {
        PopupState::Basic => (500, 500, 400)
    }
}

from_inherit_to_view_basic_style!(SelectOptionsBasicStyle);
