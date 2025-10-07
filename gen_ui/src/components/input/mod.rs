mod event;
mod prop;
mod rely;

use std::rc::Rc;

pub use event::*;
pub use prop::*;
use rely::*;

use makepad_widgets::{
    text::{
        geom::Point,
        layouter::LaidoutText,
        selection::{Cursor, CursorPosition, Selection},
    },
    *,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::shader::{draw_input::{DrawCursor, DrawSelection}, draw_view::DrawView};

live_design! {
    link widgets;

    use link::theme::*;
    use makepad_draw::shader::std::*;

    pub GInputBase = {{GInput}} {
        width: Fill, height: Fit,
        padding: <THEME_MSPACE_1> { left: (THEME_SPACE_2), right: (THEME_SPACE_2) }
        margin: <THEME_MSPACE_V_1> {}
        flow: RightWrap,
        is_password: false,
        is_read_only: false,
        is_numeric_only: false
        empty_text: "Your text here",

        draw_text: {
            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
            instance empty: 0.0
            instance disabled: 0.0

            color: (THEME_COLOR_TEXT)
            uniform color_hover: (THEME_COLOR_TEXT_HOVER)
            uniform color_focus: (THEME_COLOR_TEXT_FOCUS)
            uniform color_down: (THEME_COLOR_TEXT_DOWN)
            uniform color_disabled: (THEME_COLOR_TEXT_DISABLED)
            uniform color_empty: (THEME_COLOR_TEXT_PLACEHOLDER)
            uniform color_empty_hover: (THEME_COLOR_TEXT_PLACEHOLDER_HOVER)
            uniform color_empty_focus: (THEME_COLOR_TEXT_FOCUS)

            text_style: <THEME_FONT_REGULAR> {
                line_spacing: (THEME_FONT_WDGT_LINE_SPACING),
                font_size: (THEME_FONT_SIZE_P)
            }

            fn get_color(self) -> vec4 {
                return
                    mix(
                        mix(
                            mix(
                                mix(
                                    self.color,
                                    mix(
                                        self.color_hover,
                                        self.color_down,
                                        self.down
                                    ),
                                    self.hover
                                ),
                                self.color_empty,
                                self.empty
                            ),
                            self.color_focus,
                            self.focus
                        ),
                        self.color_disabled,
                        self.disabled
                    )
            }
        }

        animator: {
            empty = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.}}
                    apply: {
                        draw_input: {empty: 0.0}
                        draw_text: {empty: 0.0}
                        draw_selection: {empty: 0.0}
                        draw_cursor: {empty: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_input: {empty: 1.0}
                        draw_text: {empty: 1.0}
                        draw_selection: {empty: 1.0}
                        draw_cursor: {empty: 1.0}
                    }
                }
            }
            blink = {
                default: off
                off = {
                    from: {all: Forward {duration:0.05}}
                    apply: {
                        draw_cursor: {blink:0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.05}}
                    apply: {
                        draw_cursor: {blink:1.0}
                    }
                }
            }
            disabled = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.}}
                    apply: {
                        draw_input: {disabled: 0.0}
                        draw_text: {disabled: 0.0}
                        draw_selection: {disabled: 0.0}
                        draw_cursor: {disabled: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_input: {disabled: 1.0}
                        draw_text: {disabled: 1.0}
                        draw_selection: {disabled: 1.0}
                        draw_cursor: {disabled: 1.0}
                    }
                }
            }
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_input: {down: 0.0, hover: 0.0}
                        draw_text: {down: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: 0.1}
                        down: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_input: {down: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {down: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                down = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_input: {down: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {down: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
            focus = {
                default: off
                off = {
                    from: {
                        all: Forward { duration: 0.25 }
                    }
                    apply: {
                        draw_input: { focus: 0.0 }
                        draw_text: { focus: 0.0 },
                        draw_cursor: { focus: 0.0 },
                        draw_selection: { focus: 0.0 }
                    }
                }
                on = {
                    from: { all: Snap }
                    apply: {
                        draw_input: { focus: 1.0 }
                        draw_text: { focus: 1.0 }
                        draw_cursor: { focus: 1.0 },
                        draw_selection: { focus: 1.0 }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GInput {
    #[animator]
    animator: Animator,
    #[redraw]
    #[live]
    draw_input: DrawView,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_selection: DrawSelection,
    #[live]
    draw_cursor: DrawCursor,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live]
    label_align: Align,

    #[live]
    is_password: bool,
    #[live]
    is_read_only: bool,
    #[live]
    is_numeric_only: bool,
    #[live]
    empty_text: String,
    #[live]
    text: String,
    #[live(0.5)]
    blink_speed: f64,

    #[rust]
    password_text: String,
    #[rust]
    laidout_text: Option<Rc<LaidoutText>>,
    #[rust]
    text_area: Area,
    #[rust]
    selection: Selection,
    #[rust]
    history: History,
    #[rust]
    blink_timer: Timer,
}

impl LiveHook for GInput {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.check_text_is_empty(cx);
    }
}

impl Widget for GInput {
    fn text(&self) -> String {
        self.text.clone()
    }

    fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.text = self.filter_input(text, true);
        self.set_selection(
            cx,
            Selection {
                anchor: Cursor {
                    index: self.selection.anchor.index.min(self.text.len()),
                    prefer_next_row: self.selection.anchor.prefer_next_row,
                },
                cursor: Cursor {
                    index: self.selection.cursor.index.min(self.text.len()),
                    prefer_next_row: self.selection.cursor.prefer_next_row,
                },
            },
        );
        self.history.clear();
        self.laidout_text = None;
        self.draw_input.redraw(cx);
        self.check_text_is_empty(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_input.begin(cx, walk, self.layout);
        self.draw_selection.append_to_draw_call(cx);
        self.layout_text(cx);
        let text_rect = self.draw_text(cx);
        let cursor_pos = self.draw_cursor(cx, text_rect);
        self.draw_selection(cx, text_rect);
        self.draw_input.end(cx);
        if cx.has_key_focus(self.draw_input.area()) {
            cx.show_text_ime(self.draw_input.area(), cursor_pos);
        }
        cx.add_nav_stop(self.draw_input.area(), NavRole::TextInput, Margin::default());
        DrawStep::done()
    }

    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.animator_toggle(
            cx,
            disabled,
            Animate::Yes,
            id!(disabled.on),
            id!(disabled.off),
        );
    }

    fn disabled(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, id!(disabled.on))
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_input.redraw(cx);
        }

        if self.blink_timer.is_event(event).is_some() {
            if self.animator_in_state(cx, id!(blink.off)) {
                self.animator_play(cx, id!(blink.on));
            } else {
                self.animator_play(cx, id!(blink.off));
            }
            self.blink_timer = cx.start_timeout(self.blink_speed)
        }

        let uid = self.widget_uid();
        match event.hits(cx, self.draw_input.area()) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::KeyFocus(_) => {
                self.animator_play(cx, id!(focus.on));
                self.reset_cursor_blinker(cx);
                cx.widget_action(uid, &scope.path, TextInputAction::KeyFocus);
            }
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, id!(focus.off));
                self.animator_play(cx, id!(blink.on));
                cx.stop_timer(self.blink_timer);
                cx.hide_text_ime();
                cx.widget_action(uid, &scope.path, TextInputAction::KeyFocusLost);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowLeft,
                modifiers:
                    KeyModifiers {
                        shift: keep_selection,
                        logo: false,
                        alt: false,
                        control: false,
                    },
                ..
            }) => self.move_cursor_left(cx, keep_selection),
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowRight,
                modifiers:
                    KeyModifiers {
                        shift: keep_selection,
                        logo: false,
                        alt: false,
                        control: false,
                    },
                ..
            }) => self.move_cursor_right(cx, keep_selection),
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowUp,
                modifiers:
                    KeyModifiers {
                        shift: keep_selection,
                        logo: false,
                        alt: false,
                        control: false,
                    },
                ..
            }) => {
                if self.move_cursor_up(cx, keep_selection).is_err() {
                    warning!("can't move cursor because layout was invalidated by earlier event");
                }
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowDown,
                modifiers:
                    KeyModifiers {
                        shift: keep_selection,
                        logo: false,
                        alt: false,
                        control: false,
                    },
                ..
            }) => {
                if self.move_cursor_down(cx, keep_selection).is_err() {
                    warning!("can't move cursor because layout was invalidated by earlier event");
                }
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyA,
                modifiers,
                ..
            }) if modifiers.is_primary() => self.select_all(cx),
            Hit::FingerDown(FingerDownEvent {
                abs,
                tap_count,
                device,
                ..
            }) if device.is_primary_hit() => {
                self.set_key_focus(cx);
                let rel = abs - self.text_area.rect(cx).pos;
                let Ok(cursor) =
                    self.point_in_lpxs_to_cursor(Point::new(rel.x as f32, rel.y as f32))
                else {
                    warning!("can't move cursor because layout was invalidated by earlier event");
                    return;
                };
                self.set_cursor(cx, cursor, false);
                match tap_count {
                    2 => self.select_word(cx),
                    3 => self.select_all(cx),
                    _ => {}
                }

                self.animator_play(cx, id!(hover.down));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over && fe.was_tap() {
                    if fe.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            Hit::FingerMove(FingerMoveEvent {
                abs,
                tap_count,
                device,
                ..
            }) if device.is_primary_hit() => {
                self.set_key_focus(cx);
                let rel = abs - self.text_area.rect(cx).pos;
                let Ok(cursor) =
                    self.point_in_lpxs_to_cursor(Point::new(rel.x as f32, rel.y as f32))
                else {
                    warning!("can't move cursor because layout was invalidated by earlier event");
                    return;
                };
                self.set_cursor(cx, cursor, true);
                match tap_count {
                    2 => self.select_word(cx),
                    3 => self.select_all(cx),
                    _ => {}
                }
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ReturnKey,
                modifiers: KeyModifiers { shift: false, .. },
                ..
            }) => {
                cx.hide_text_ime();
                cx.widget_action(
                    uid,
                    &scope.path,
                    TextInputAction::Returned(self.text.clone()),
                );
            }

            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Escape,
                ..
            }) => {
                cx.widget_action(uid, &scope.path, TextInputAction::Escaped);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ReturnKey,
                modifiers: KeyModifiers { shift: true, .. },
                ..
            }) if !self.is_read_only => {
                self.create_or_extend_edit_group(EditKind::Other);
                self.apply_edit(
                    cx,
                    Edit {
                        start: self.selection.start().index,
                        end: self.selection.end().index,
                        replace_with: "\n".to_string(),
                    },
                );
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    TextInputAction::Changed(self.text.clone()),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Backspace,
                ..
            }) if !self.is_read_only => {
                let mut start = self.selection.start().index;
                let end = self.selection.end().index;
                if start == end {
                    start = prev_grapheme_boundary(&self.text, start);
                }
                self.create_or_extend_edit_group(EditKind::Backspace);
                self.apply_edit(
                    cx,
                    Edit {
                        start,
                        end,
                        replace_with: String::new(),
                    },
                );
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    TextInputAction::Changed(self.text.clone()),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Delete,
                ..
            }) if !self.is_read_only => {
                let start = self.selection.start().index;
                let mut end = self.selection.end().index;
                if start == end {
                    end = next_grapheme_boundary(&self.text, end);
                }
                self.create_or_extend_edit_group(EditKind::Delete);
                self.apply_edit(
                    cx,
                    Edit {
                        start,
                        end,
                        replace_with: String::new(),
                    },
                );
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    TextInputAction::Changed(self.text.clone()),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyZ,
                modifiers: modifiers @ KeyModifiers { shift: false, .. },
                ..
            }) if modifiers.is_primary() && !self.is_read_only => {
                if !self.undo(cx) {
                    return;
                }
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    TextInputAction::Changed(self.text.clone()),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyZ,
                modifiers: modifiers @ KeyModifiers { shift: true, .. },
                ..
            }) if modifiers.is_primary() && !self.is_read_only => {
                if !self.redo(cx) {
                    return;
                }
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    TextInputAction::Changed(self.text.clone()),
                );
            }
            Hit::TextInput(TextInputEvent {
                input,
                replace_last,
                was_paste,
                ..
            }) if !self.is_read_only => {
                let input = self.filter_input(&input, false);
                if input.is_empty() {
                    return;
                }
                self.create_or_extend_edit_group(if replace_last || was_paste {
                    EditKind::Other
                } else {
                    EditKind::Insert
                });
                self.apply_edit(
                    cx,
                    Edit {
                        start: self.selection.start().index,
                        end: self.selection.end().index,
                        replace_with: input,
                    },
                );
                self.animator_play(cx, id!(empty.off));
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    TextInputAction::Changed(self.text.clone()),
                );
            }
            Hit::TextCopy(event) => {
                *event.response.borrow_mut() = Some(self.selected_text().to_string());
            }
            Hit::TextCut(event) => {
                *event.response.borrow_mut() = Some(self.selected_text().to_string());
                if !self.selected_text().is_empty() {
                    self.history
                        .create_or_extend_edit_group(EditKind::Other, self.selection);
                    self.apply_edit(
                        cx,
                        Edit {
                            start: self.selection.start().index,
                            end: self.selection.end().index,
                            replace_with: String::new(),
                        },
                    );
                    self.draw_input.redraw(cx);
                    cx.widget_action(
                        uid,
                        &scope.path,
                        TextInputAction::Changed(self.text.clone()),
                    );
                }
            }
            Hit::KeyDown(event) => {
                cx.widget_action(uid, &scope.path, TextInputAction::KeyDownUnhandled(event));
            }
            _ => {}
        }
    }
}

impl GInput {
    pub fn is_password(&self) -> bool {
        self.is_password
    }

    pub fn set_is_password(&mut self, cx: &mut Cx, is_password: bool) {
        self.is_password = is_password;
        self.laidout_text = None;
        self.draw_input.redraw(cx);
    }

    pub fn toggle_is_password(&mut self, cx: &mut Cx) {
        self.set_is_password(cx, !self.is_password);
    }

    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    pub fn set_is_read_only(&mut self, cx: &mut Cx, is_read_only: bool) {
        self.is_read_only = is_read_only;
        self.laidout_text = None;
        self.draw_input.redraw(cx);
    }

    pub fn toggle_is_read_only(&mut self, cx: &mut Cx) {
        self.set_is_read_only(cx, !self.is_read_only);
    }

    pub fn is_numeric_only(&self) -> bool {
        self.is_numeric_only
    }

    pub fn set_is_numeric_only(&mut self, cx: &mut Cx, is_numeric_only: bool) {
        self.is_numeric_only = is_numeric_only;
        self.laidout_text = None;
        self.draw_input.redraw(cx);
    }

    pub fn toggle_is_numeric_only(&mut self, cx: &mut Cx) {
        self.set_is_numeric_only(cx, !self.is_numeric_only);
    }

    pub fn empty_text(&self) -> &str {
        &self.empty_text
    }

    pub fn set_empty_text(&mut self, cx: &mut Cx, empty_text: String) {
        self.empty_text = empty_text;
        if self.text.is_empty() {
            self.draw_input.redraw(cx);
        }
    }

    pub fn selection(&self) -> Selection {
        self.selection
    }

    pub fn set_selection(&mut self, cx: &mut Cx, selection: Selection) {
        self.selection = selection;
        self.history.force_new_edit_group();
        self.reset_blink_timer(cx);
        self.draw_input.redraw(cx);
    }

    pub fn cursor(&self) -> Cursor {
        self.selection.cursor
    }

    pub fn set_cursor(&mut self, cx: &mut Cx, cursor: Cursor, keep_selection: bool) {
        self.set_selection(
            cx,
            Selection {
                anchor: if keep_selection {
                    self.selection.anchor
                } else {
                    cursor
                },
                cursor,
            },
        );
    }

    pub fn selected_text(&self) -> &str {
        &self.text[self.selection.start().index..self.selection.end().index]
    }

    pub fn reset_blink_timer(&mut self, cx: &mut Cx) {
        self.animator_cut(cx, id!(blink.off));
        if !self.is_read_only {
            cx.stop_timer(self.blink_timer);
            self.blink_timer = cx.start_timeout(self.blink_speed)
        }
    }

    fn cursor_to_position(&self, cursor: Cursor) -> Result<CursorPosition, ()> {
        let Some(laidout_text) = self.laidout_text.as_ref() else {
            return Err(());
        };
        let position = laidout_text.cursor_to_position(self.cursor_to_password_cursor(cursor));
        Ok(CursorPosition {
            row_index: position.row_index,
            x_in_lpxs: position.x_in_lpxs * self.draw_text.font_scale,
        })
    }

    fn point_in_lpxs_to_cursor(&self, point_in_lpxs: Point<f32>) -> Result<Cursor, ()> {
        let Some(laidout_text) = self.laidout_text.as_ref() else {
            return Err(());
        };
        let cursor =
            laidout_text.point_in_lpxs_to_cursor(point_in_lpxs / self.draw_text.font_scale);
        Ok(self.password_cursor_to_cursor(cursor))
    }

    fn position_to_cursor(&self, position: CursorPosition) -> Result<Cursor, ()> {
        let Some(laidout_text) = self.laidout_text.as_ref() else {
            return Err(());
        };
        let cursor = laidout_text.position_to_cursor(CursorPosition {
            row_index: position.row_index,
            x_in_lpxs: position.x_in_lpxs / self.draw_text.font_scale,
        });
        Ok(self.password_cursor_to_cursor(cursor))
    }

    fn selection_to_password_selection(&self, selection: Selection) -> Selection {
        Selection {
            cursor: self.cursor_to_password_cursor(selection.cursor),
            anchor: self.cursor_to_password_cursor(selection.anchor),
        }
    }

    fn cursor_to_password_cursor(&self, cursor: Cursor) -> Cursor {
        Cursor {
            index: self.index_to_password_index(cursor.index),
            prefer_next_row: cursor.prefer_next_row,
        }
    }

    fn password_cursor_to_cursor(&self, password_cursor: Cursor) -> Cursor {
        Cursor {
            index: self.password_index_to_index(password_cursor.index),
            prefer_next_row: password_cursor.prefer_next_row,
        }
    }

    fn index_to_password_index(&self, index: usize) -> usize {
        if !self.is_password {
            return index;
        }
        let grapheme_index = self.text[..index].graphemes(true).count();
        self.password_text
            .grapheme_indices(true)
            .nth(grapheme_index)
            .map_or(self.password_text.len(), |(index, _)| index)
    }

    fn password_index_to_index(&self, password_index: usize) -> usize {
        if !self.is_password {
            return password_index;
        }
        let grapheme_index = self.password_text[..password_index].graphemes(true).count();
        self.text
            .grapheme_indices(true)
            .nth(grapheme_index)
            .map_or(self.text.len(), |(index, _)| index)
    }

    fn inner_walk(&self) -> Walk {
        if self.walk.width.is_fit() {
            Walk::fit()
        } else {
            Walk::fill_fit()
        }
    }

    fn layout_text(&mut self, cx: &mut Cx2d) {
        if self.laidout_text.is_some() {
            return;
        }
        let text = if self.is_password {
            self.password_text.clear();
            for grapheme in self.text.graphemes(true) {
                self.password_text
                    .push(if grapheme == "\n" { '\n' } else { '•' });
            }
            &self.password_text
        } else {
            &self.text
        };
        let turtle_rect = cx.turtle().padded_rect();
        let max_width_in_lpxs = if !turtle_rect.size.x.is_nan() {
            Some(turtle_rect.size.x as f32)
        } else {
            None
        };
        let wrap_width_in_lpxs = if cx.turtle().layout().flow == Flow::RightWrap {
            max_width_in_lpxs
        } else {
            None
        };
        self.laidout_text =
            Some(
                self.draw_text
                    .layout(cx, 0.0, 0.0, wrap_width_in_lpxs, self.label_align, text),
            );
    }

    fn draw_text(&mut self, cx: &mut Cx2d) -> Rect {
        let inner_walk = self.inner_walk();
        let text_rect = if self.text.is_empty() {
            self.draw_text
                .draw_walk(cx, inner_walk, self.label_align, &self.empty_text)
        } else {
            let laidout_text = self.laidout_text.as_ref().unwrap();
            self.draw_text
                .draw_walk_laidout(cx, inner_walk, self.label_align, laidout_text)
        };
        cx.add_aligned_rect_area(&mut self.text_area, text_rect);
        text_rect
    }

    fn draw_cursor(&mut self, cx: &mut Cx2d, text_rect: Rect) -> DVec2 {
        let CursorPosition {
            row_index,
            x_in_lpxs,
        } = self
            .cursor_to_position(self.selection.cursor)
            .ok()
            .expect("layout should not be `None` because we called `layout_text` in `draw_walk`");
        let laidout_text = self
            .laidout_text
            .as_ref()
            .expect("layout should not be `None` because we called `layout_text` in `draw_walk`");
        let row = &laidout_text.rows[row_index];
        let cursor_pos = dvec2(
            (x_in_lpxs - 1.0 * self.draw_text.font_scale) as f64,
            ((row.origin_in_lpxs.y - row.ascender_in_lpxs) * self.draw_text.font_scale) as f64,
        );
        self.draw_cursor.draw_abs(
            cx,
            rect(
                text_rect.pos.x + cursor_pos.x,
                text_rect.pos.y + cursor_pos.y,
                (2.8 * self.draw_text.font_scale) as f64,
                ((row.ascender_in_lpxs - row.descender_in_lpxs) * self.draw_text.font_scale) as f64,
            ),
        );
        cursor_pos
    }

    fn draw_selection(&mut self, cx: &mut Cx2d, text_rect: Rect) {
        let laidout_text = self
            .laidout_text
            .as_ref()
            .expect("layout should not be `None` because we called `layout_text` in `draw_walk`");

        self.draw_selection.begin_many_instances(cx);
        for rect_in_lpxs in laidout_text
            .selection_rects_in_lpxs(self.selection_to_password_selection(self.selection))
        {
            self.draw_selection.draw_abs(
                cx,
                rect(
                    text_rect.pos.x + (rect_in_lpxs.origin.x * self.draw_text.font_scale) as f64,
                    text_rect.pos.y + (rect_in_lpxs.origin.y * self.draw_text.font_scale) as f64,
                    (rect_in_lpxs.size.width * self.draw_text.font_scale) as f64,
                    (rect_in_lpxs.size.height * self.draw_text.font_scale) as f64,
                ),
            );
        }
        self.draw_selection.end_many_instances(cx);
    }

    pub fn move_cursor_left(&mut self, cx: &mut Cx, keep_selection: bool) {
        self.set_cursor(
            cx,
            Cursor {
                index: prev_grapheme_boundary(&self.text, self.selection.cursor.index),
                prefer_next_row: true,
            },
            keep_selection,
        );
    }

    pub fn move_cursor_right(&mut self, cx: &mut Cx, keep_selection: bool) {
        self.set_cursor(
            cx,
            Cursor {
                index: next_grapheme_boundary(&self.text, self.selection.cursor.index),
                prefer_next_row: false,
            },
            keep_selection,
        );
    }

    pub fn move_cursor_up(&mut self, cx: &mut Cx, keep_selection: bool) -> Result<(), ()> {
        let position = self.cursor_to_position(self.selection.cursor)?;
        self.set_cursor(
            cx,
            self.position_to_cursor(CursorPosition {
                row_index: if position.row_index == 0 {
                    0
                } else {
                    position.row_index - 1
                },
                x_in_lpxs: position.x_in_lpxs,
            })?,
            keep_selection,
        );
        Ok(())
    }

    pub fn move_cursor_down(&mut self, cx: &mut Cx, keep_selection: bool) -> Result<(), ()> {
        let laidout_text = self.laidout_text.as_ref().unwrap();
        let position = self.cursor_to_position(self.selection.cursor)?;
        self.set_cursor(
            cx,
            self.position_to_cursor(CursorPosition {
                row_index: if position.row_index == laidout_text.rows.len() - 1 {
                    laidout_text.rows.len() - 1
                } else {
                    position.row_index + 1
                },
                x_in_lpxs: position.x_in_lpxs,
            })?,
            keep_selection,
        );
        Ok(())
    }

    pub fn select_all(&mut self, cx: &mut Cx) {
        self.set_selection(
            cx,
            Selection {
                anchor: Cursor {
                    index: 0,
                    prefer_next_row: false,
                },
                cursor: Cursor {
                    index: self.text.len(),
                    prefer_next_row: false,
                },
            },
        );
    }

    pub fn select_word(&mut self, cx: &mut Cx) {
        if self.selection.cursor.index < self.selection.anchor.index {
            self.set_cursor(
                cx,
                Cursor {
                    index: self.ceil_word_boundary(self.selection.cursor.index),
                    prefer_next_row: true,
                },
                true,
            );
        } else if self.selection.cursor.index > self.selection.anchor.index {
            self.set_cursor(
                cx,
                Cursor {
                    index: self.floor_word_boundary(self.selection.cursor.index),
                    prefer_next_row: false,
                },
                true,
            );
        } else {
            self.set_selection(
                cx,
                Selection {
                    anchor: Cursor {
                        index: self.ceil_word_boundary(self.selection.cursor.index),
                        prefer_next_row: true,
                    },
                    cursor: Cursor {
                        index: self.floor_word_boundary(self.selection.cursor.index),
                        prefer_next_row: false,
                    },
                },
            );
        }
    }

    pub fn force_new_edit_group(&mut self) {
        self.history.force_new_edit_group();
    }

    fn ceil_word_boundary(&self, index: usize) -> usize {
        let mut prev_word_boundary_index = 0;
        for (word_boundary_index, _) in self.text.split_word_bound_indices() {
            if word_boundary_index > index {
                return prev_word_boundary_index;
            }
            prev_word_boundary_index = word_boundary_index;
        }
        prev_word_boundary_index
    }

    fn floor_word_boundary(&self, index: usize) -> usize {
        let mut prev_word_boundary_index = self.text.len();
        for (word_boundary_index, _) in self.text.split_word_bound_indices().rev() {
            if word_boundary_index < index {
                return prev_word_boundary_index;
            }
            prev_word_boundary_index = word_boundary_index;
        }
        prev_word_boundary_index
    }

    fn filter_input(&self, input: &str, is_set_text: bool) -> String {
        if self.is_numeric_only {
            let mut contains_dot = if is_set_text {
                false
            } else {
                let before_selection = self.text[..self.selection.start().index].to_string();
                let after_selection = self.text[self.selection.end().index..].to_string();
                before_selection.contains('.') || after_selection.contains('.')
            };
            input
                .chars()
                .filter(|char| match char {
                    '.' | ',' if !contains_dot => {
                        contains_dot = true;
                        true
                    }
                    char => char.is_ascii_digit(),
                })
                .collect()
        } else {
            input.to_string()
        }
    }

    fn create_or_extend_edit_group(&mut self, edit_kind: EditKind) {
        self.history
            .create_or_extend_edit_group(edit_kind, self.selection);
    }

    fn apply_edit(&mut self, cx: &mut Cx, edit: Edit) {
        self.selection.cursor.index = edit.start + edit.replace_with.len();
        self.selection.anchor.index = self.selection.cursor.index;
        self.history.apply_edit(edit, &mut self.text);
        self.laidout_text = None;
        self.check_text_is_empty(cx);
    }

    fn undo(&mut self, cx: &mut Cx) -> bool {
        if let Some(new_selection) = self.history.undo(self.selection, &mut self.text) {
            self.laidout_text = None;
            self.selection = new_selection;
            self.check_text_is_empty(cx);
            true
        } else {
            false
        }
    }

    fn redo(&mut self, cx: &mut Cx) -> bool {
        if let Some(new_selection) = self.history.redo(self.selection, &mut self.text) {
            self.laidout_text = None;
            self.selection = new_selection;
            self.check_text_is_empty(cx);
            true
        } else {
            false
        }
    }

    fn check_text_is_empty(&mut self, cx: &mut Cx) {
        if self.text.is_empty() {
            self.animator_play(cx, id!(empty.on));
        } else {
            self.animator_play(cx, id!(empty.off));
        }
    }

    fn reset_cursor_blinker(&mut self, cx: &mut Cx) {
        if self.is_read_only {
            self.animator_cut(cx, id!(blink.off));
        } else {
            self.animator_cut(cx, id!(blink.off));
            cx.stop_timer(self.blink_timer);
            self.blink_timer = cx.start_timeout(self.blink_speed)
        }
    }
}

// impl TextInputRef {
//     pub fn is_password(&self) -> bool {
//         if let Some(inner) = self.borrow(){
//             inner.is_password()
//         }
//         else{
//             false
//         }
//     }

//     pub fn set_is_password(&self, cx: &mut Cx, is_password: bool) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.set_is_password(cx, is_password);
//         }
//     }

//     pub fn toggle_is_password(&self, cx: &mut Cx) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.toggle_is_password(cx);
//         }
//     }

//     pub fn is_read_only(&self) -> bool {
//         if let Some(inner) = self.borrow(){
//             inner.is_read_only()
//         }
//         else{
//             false
//         }
//     }

//     pub fn set_is_read_only(&self, cx: &mut Cx, is_read_only: bool) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.set_is_read_only(cx, is_read_only);
//         }
//     }

//     pub fn toggle_is_read_only(&self, cx: &mut Cx) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.toggle_is_read_only(cx);
//         }
//     }

//     pub fn is_numeric_only(&self) -> bool {
//         if let Some(inner) = self.borrow(){
//             inner.is_numeric_only()
//         }
//         else{
//             false
//         }
//     }

//     pub fn set_is_numeric_only(&self, cx: &mut Cx, is_numeric_only: bool) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.set_is_numeric_only(cx, is_numeric_only);
//         }
//     }

//     pub fn toggle_is_numeric_only(&self, cx: &mut Cx) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.toggle_is_numeric_only(cx);
//         }
//     }

//     pub fn empty_text(&self) -> String {
//         if let Some(inner) = self.borrow(){
//             inner.empty_text().to_string()
//         }
//         else{
//             String::new()
//         }
//     }

//     pub fn set_empty_text(&self, cx: &mut Cx, empty_text: String) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.set_empty_text(cx, empty_text);
//         }
//     }

//     pub fn selection(&self) -> Selection {
//         if let Some(inner) = self.borrow(){
//             inner.selection()
//         }
//         else{
//             Default::default()
//         }
//     }

//     pub fn set_selection(&self, cx: &mut Cx, selection: Selection) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.set_selection(cx, selection);
//         }
//     }

//     pub fn cursor(&self) -> Cursor {
//         if let Some(inner) = self.borrow(){
//             inner.cursor()
//         }
//         else{
//             Default::default()
//         }
//     }

//     pub fn set_cursor(&self, cx: &mut Cx, cursor: Cursor, keep_selection: bool) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.set_cursor(cx, cursor, keep_selection);
//         }
//     }

//     pub fn selected_text(&self) -> String {
//         if let Some(inner) = self.borrow(){
//             inner.selected_text().to_string()
//         }
//         else{
//             String::new()
//         }
//     }

//     pub fn returned(&self, actions: &Actions) -> Option<String> {
//         for action in actions.filter_widget_actions_cast::<TextInputAction>(self.widget_uid()){
//             if let TextInputAction::Returned(text) = action{
//                 return Some(text);
//             }
//         }
//         None
//     }

//     pub fn escaped(&self, actions: &Actions) -> bool {
//         for action in actions.filter_widget_actions_cast::<TextInputAction>(self.widget_uid()){
//             if let TextInputAction::Escaped = action {
//                 return true;
//             }
//         }
//         false
//     }

//     pub fn changed(&self, actions: &Actions) -> Option<String> {
//         for action in actions.filter_widget_actions_cast::<TextInputAction>(self.widget_uid()){
//             if let TextInputAction::Changed(text) = action{
//                 return Some(text);
//             }
//         }
//         None
//     }

//     pub fn key_down_unhandled(&self, actions: &Actions) -> Option<KeyEvent> {
//         for action in actions.filter_widget_actions_cast::<TextInputAction>(self.widget_uid()){
//             if let TextInputAction::KeyDownUnhandled(event) = action{
//                 return Some(event);
//             }
//         }
//         None
//     }
// }
