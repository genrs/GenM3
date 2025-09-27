use crate::{basic_prop_interconvert, prop_interconvert};

prop_interconvert! {
    ColorPanelStyle {
        basic_prop = ColorPanelBasicStyle;
        hover => HOVER, ColorPanelBasicStyle::from_state(Theme::default(), ColorPanelState::Hover),|v| (v, ColorPanelState::Hover).try_into(),
        pressed => PRESSED, ColorPanelBasicStyle::from_state(Theme::default(), ColorPanelState::Pressed),|v| (v, ColorPanelState::Pressed).try_into(),
        disabled => DISABLED, ColorPanelBasicStyle::from_state(Theme::default(), ColorPanelState::Disabled),|v| (v, ColorPanelState::Disabled).try_into()
    }, "[component.color_panel] should be a table"
}

basic_prop_interconvert! {
    ButtonBasicStyle {
        state = ButtonState;
        {
            background_color => BACKGROUND_COLOR, |v| v.try_into(),
            shadow_color => SHADOW_COLOR, |v| v.try_into(),
            border_color => BORDER_COLOR, |v| v.try_into()
        };
        {
            background_visible: bool => BACKGROUND_VISIBLE, true, |v| v.to_bool(),
            spread_radius: f32 => SPREAD_RADIUS, 0.0, |v| v.to_f32(),
            blur_radius: f32 => BLUR_RADIUS, 0.0, |v| v.to_f32(),
            shadow_offset: Vec2 => SHADOW_OFFSET, vec2(0.0, 0.0), |v| v.to_vec2(shadow_offset),
            border_width: f32 => BORDER_WIDTH, 0.0, |v| v.to_f32(),
            border_radius: Radius => BORDER_RADIUS, Radius::new(2.0), |v| v.try_into(),
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            padding: Padding => PADDING, Padding::from_xy(9.0, 16.0), |v| v.to_padding(padding),
            flow: Flow => FLOW, Flow::Right, |v| v.to_flow(),
            align: Align => ALIGN, Align::from_f64(0.5), |v| v.to_align(align),
            height: Size => HEIGHT, Size::Fit, |v| v.to_size(),
            width: Size => WIDTH, Size::Fit, |v| v.to_size(),
            spacing: f64 => SPACING, 6.0, |v| v.to_f64(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "ButtonBasicStyle should be a inline table"
}
