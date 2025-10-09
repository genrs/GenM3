use makepad_widgets::*;

live_design!{
    link genui_animation_prop;

    pub AN_DURATION = 0.25, // default animation duration
    pub AN_DURATION_FAST = 0.1, // fast animation duration
    pub AN_DURATION_FASTEST = 0.05, // fastest animation duration
    pub AN_DURATION_NO = 0.0, // no animation duration

    // animation for default draw view
    pub AN_DRAW_VIEW = {
        background_color: #777777,
        border_color: #777777,
        border_width: 0.0,
        // border_radius: vec4(4.0, 4.0, 4.0, 4.0),
        shadow_color: #777777,
        spread_radius: 0.0,
        blur_radius: 0.0,
        background_visible: 1.0,
        rotation: 0.0,
        scale: 1.0,
        shadow_offset: vec2(0.0, 0.0),
    }

    pub AN_DRAW_TEXT = {
        color: #ffffff,
        text_style: {
            font_size: 12.0,
            line_spacing: 1.0,
        }
    }

    pub AN_DRAW_DOT = {
        background_color: #777777,
        border_color: #777777,
        border_width: 0.0,
        // border_radius: vec4(4.0, 4.0, 4.0, 4.0),
        shadow_color: #777777,
        spread_radius: 0.0,
        blur_radius: 0.0,
        background_visible: 1.0,
        rotation: 0.0,
        scale: 1.0,
        shadow_offset: vec2(0.0, 0.0),
        dot: 0.0,
    }

    pub AN_DRAW_RADIO = {
        background_color: #777777,
        background_visible: 1.0,
        border_color: #777777,
        border_width: 1.0,
        size: 16.0,
        stroke_color: #ffffff,
    }

    pub AN_DRAW_CHECKBOX = {
        background_color: #777777,
        background_visible: 1.0,
        border_color: #777777,
        border_width: 1.0,
        size: 16.0,
        stroke_color: #ffffff,
    }

    pub AN_DRAW_SWITCH = {
        background_color: #777777,
        background_visible: 1.0,
        border_color: #777777,
        border_width: 1.0,
        stroke_color: #ffffff,
        active: 0.0,
    }

    pub AN_DRAW_SVG = {
        color: #777777,
    }

    pub AN_DRAW_LINK_TEXT = {
        color: #777777,
        text_style: {
            font_size: 12.0,
            line_spacing: 1.0,
        }
    }

    pub AN_DRAW_LINK = {
        background_color: #777777,
        border_color: #777777,
        border_width: 0.0,
        // border_radius: vec4(4.0, 4.0, 4.0, 4.0),
        shadow_color: #777777,
        spread_radius: 0.0,
        blur_radius: 0.0,
        background_visible: 0.0,
        rotation: 0.0,
        scale: 1.0,
        shadow_offset: vec2(0.0, 0.0),
        underline_color: #777777,
        underline_visible: 1.0,
        underline_width: 1.0,
    }

    pub AN_DRAW_PROGRESS = {
        background_color: #777777,
        border_color: #777777,
        border_width: 0.0,
        // border_radius: vec4(4.0, 4.0, 4.0, 4.0),
        shadow_color: #777777,
        spread_radius: 0.0,
        blur_radius: 0.0,
        background_visible: 1.0,
        rotation: 0.0,
        scale: 1.0,
        shadow_offset: vec2(0.0, 0.0),
        color: #ffffff,
        value: 0.0,
    }

    pub AN_DRAW_SLIDER = {
        background_color: #777777,
        border_color: #777777,
        border_width: 0.0,
        // border_radius: vec4(4.0, 4.0, 4.0, 4.0),
        shadow_color: #777777,
        spread_radius: 0.0,
        blur_radius: 0.0,
        background_visible: 1.0,
        rotation: 0.0,
        scale: 1.0,
        shadow_offset: vec2(0.0, 0.0),
        color: #ffffff,
        value: 0.0,
        proportion: 0.8,
        dragging: 0.0,
    }

    pub AN_DRAW_RATE = {
        color: #777777,
        spacing: 4.0,
        count: 5.0,
        value: 0.0,
    }

    pub AN_DRAW_CURSOR = {
        color: #ffffff,
        border_radius: 0.5,
    }

    pub AN_DRAW_SELECTION = {
        color: #ffffff,
        border_radius: 0.5,
    }
}