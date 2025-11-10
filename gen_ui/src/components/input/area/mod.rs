mod prop;

use super::event::*;
use super::rely::*;
pub use prop::*;
use std::rc::Rc;

use makepad_widgets::{
    text::{
        geom::Point,
        layouter::LaidoutText,
        selection::{Cursor, CursorPosition, Selection},
    },
    *,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::components::InputState;
use crate::event_option;
use crate::event_option_ref;
use crate::switch_state;
use crate::{
    ComponentAnInit, active_event, animation_open_then_redraw,
    components::{
        BasicStyle, Component, LabelBasicStyle, LifeCycle, SlotComponent, SlotStyle, Style,
        ViewBasicStyle,
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        ApplySlotMap,
        manuel::{BASIC, DISABLED, EMPTY, FOCUS, HOVER},
        traits::ToFloat,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::{
        draw_input::{DrawCursor, DrawSelection},
        draw_view::DrawView,
    },
    sync,
    themes::conf::Conf,
    visible,
};

live_design! {
    link genui_basic;
    use link::theme::*;
    use link::genui_animation_prop::*;

    pub GInputAreaBase = {{GInputArea}} {
        draw_text: {
            text_style: <THEME_FONT_REGULAR>{}
        }

        animator: {
            blink = {
                default: off
                off = {
                    from: {all: Forward {duration: (AN_DURATION_FASTEST)}}
                    apply: {
                        draw_cursor: {blink:0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (AN_DURATION_FASTEST)}}
                    apply: {
                        draw_cursor: {blink:1.0}
                    }
                }
            }
            input = {
                default: basic,

                basic = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_input: <AN_DRAW_VIEW> {},
                        draw_text: <AN_DRAW_TEXT> {},
                        draw_selection: <AN_DRAW_SELECTION> {},
                        draw_cursor: <AN_DRAW_CURSOR> {}
                    }
                }

                hover = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_input: <AN_DRAW_VIEW> {},
                       draw_text: <AN_DRAW_TEXT> {},
                       draw_selection: <AN_DRAW_SELECTION> {},
                       draw_cursor: <AN_DRAW_CURSOR> {}
                    }
                }

                empty = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_input: <AN_DRAW_VIEW> {},
                       draw_text: <AN_DRAW_TEXT> {},
                       draw_selection: <AN_DRAW_SELECTION> {},
                       draw_cursor: <AN_DRAW_CURSOR> {}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_input: <AN_DRAW_VIEW> {},
                        draw_text: <AN_DRAW_TEXT> {},
                        draw_selection: <AN_DRAW_SELECTION> {},
                        draw_cursor: <AN_DRAW_CURSOR> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_input: <AN_DRAW_VIEW> {},
                        draw_text: <AN_DRAW_TEXT> {},
                        draw_selection: <AN_DRAW_SELECTION> {},
                        draw_cursor: <AN_DRAW_CURSOR> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GInputArea {
    #[live]
    pub style: InputAreaStyle,
    #[live]
    pub draw_input: DrawView,
    #[live]
    pub draw_text: DrawText,
    #[live]
    pub draw_selection: DrawSelection,
    #[live]
    pub draw_cursor: DrawCursor,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_spread: bool,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub state: InputState,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live]
    pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_slot_map: ApplySlotMap<InputState, InputAreaPart>,
    // -------------------------------
    #[live]
    pub is_password: bool,
    #[live]
    pub is_read_only: bool,
    #[live]
    pub is_numeric_only: bool,
    #[live]
    pub placeholder: String,
    #[live]
    pub value: String,
    /// max length of input area
    #[live(None)]
    pub length: Option<usize>,
    #[live(0.6)]
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

impl WidgetNode for GInputArea {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_input.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_input.redraw(cx);
        self.draw_cursor.redraw(cx);
        self.draw_selection.redraw(cx);
        self.draw_text.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl LiveHook for GInputArea {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            apply.from,
            nodes,
            index,
            [
                live_id!(empty),
                live_id!(basic),
                live_id!(hover),
                live_id!(focus),
                live_id!(disabled),
            ],
            [
                (InputAreaPart::Container, &ViewBasicStyle::live_props()),
                (InputAreaPart::Text, &LabelBasicStyle::live_props()),
                (InputAreaPart::Selection, &SelectionBasicStyle::live_props()),
                (InputAreaPart::Cursor, &CursorBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(InputState::Basic, applys);
                }
                HOVER => {
                    component.apply_slot_map.insert(InputState::Hover, applys);
                }
                FOCUS => {
                    component.apply_slot_map.insert(InputState::Focus, applys);
                }
                EMPTY => {
                    component.apply_slot_map.insert(InputState::Empty, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(InputState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Widget for GInputArea {
    fn text(&self) -> String {
        self.value.to_string()
    }

    fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.value = self.filter_input(text, true);
        self.set_selection(
            cx,
            Selection {
                anchor: Cursor {
                    index: self.selection.anchor.index.min(self.value.len()),
                    prefer_next_row: self.selection.anchor.prefer_next_row,
                },
                cursor: Cursor {
                    index: self.selection.cursor.index.min(self.value.len()),
                    prefer_next_row: self.selection.cursor.prefer_next_row,
                },
            },
        );
        self.history.clear();
        self.laidout_text = None;
        self.draw_input.redraw(cx);
        self.check_text_is_empty(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state).container;
        self.draw_input.begin(cx, walk, style.layout());
        self.draw_selection.append_to_draw_call(cx);
        self.layout_text(cx);
        let text_rect = self.draw_text(cx);
        let cursor_pos = self.draw_cursor(cx, text_rect);
        self.draw_selection(cx, text_rect);
        self.draw_input.end(cx);
        if cx.has_key_focus(self.draw_input.area()) {
            cx.show_text_ime(self.draw_input.area(), cursor_pos);
        }

        cx.add_nav_stop(
            self.draw_input.area(),
            NavRole::TextInput,
            Margin::default(),
        );
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.animator_toggle(
            cx,
            disabled,
            Animate::Yes,
            id!(input.basic),
            id!(input.disabled),
        );
    }

    fn disabled(&self, _cx: &Cx) -> bool {
        // self.animator_in_state(cx, id!(disabled.on))
        self.disabled
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().input_area = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl SlotComponent<InputState> for GInputArea {
    type Part = InputAreaPart;

    fn merge_prop_to_slot(&mut self) -> () {
        ()
    }
}

impl Component for GInputArea {
    type Error = Error;

    type State = InputState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.input_area;
        self.style = style.clone();
        self.merge_prop_to_slot();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(InputState::Disabled);
        } else if self.value.is_empty() {
            self.switch_state(InputState::Empty);
        } else {
            self.switch_state(InputState::Basic);
        }

        let style = self.style.get(self.state);
        self.draw_input.merge(&style.container.into());
        self.draw_text.color = style.text.color;
        self.draw_text.text_style.font_size = style.text.font_size;
        self.draw_text.text_style.line_spacing = style.text.line_spacing;
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, InputState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, _area: Area) {
        animation_open_then_redraw!(self, cx, event);

        if self.value.is_empty() && !self.animator_in_state(cx, id!(input.empty)) {
            self.switch_state_with_animation(cx, InputState::Empty);
            self.animator_play(cx, id!(input.empty));
            self.redraw(cx);
        }

        if self.blink_timer.is_event(event).is_some() {
            if self.animator_in_state(cx, id!(blink.off)) {
                self.animator_play(cx, id!(blink.on));
            } else {
                self.animator_play(cx, id!(blink.off));
            }
            self.blink_timer = cx.start_timeout(self.blink_speed)
        }

        match hit {
            Hit::FingerHoverIn(e) => {
                self.switch_state_with_animation(cx, InputState::Hover);
                self.animator_play(cx, id!(input.hover));
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, InputState::Basic);
                self.animator_play(cx, id!(input.basic));
                self.active_hover_out(cx, e);
            }
            Hit::KeyFocus(e) => {
                self.animator_play(cx, id!(input.focus));
                self.reset_cursor_blinker(cx);
                self.active_focus(cx, e.into());
            }
            Hit::KeyFocusLost(e) => {
                // self.animator_play(cx, id!(focus.off));
                // self.animator_play(cx, id!(blink.on));
                self.switch_state_with_animation(cx, InputState::Basic);
                cx.stop_timer(self.blink_timer);
                cx.hide_text_ime();
                self.active_focus_lost(cx, Some(e));
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

                self.animator_play(cx, id!(input.focus));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over && fe.was_tap() {
                    if fe.has_hovers() {
                        self.switch_state_with_animation(cx, InputState::Hover);
                        self.animator_play(cx, id!(input.hover));
                    } else {
                        self.switch_state_with_animation(cx, InputState::Basic);
                        self.animator_play(cx, id!(input.basic));
                    }
                } else {
                    self.switch_state_with_animation(cx, InputState::Basic);
                    self.animator_play(cx, id!(input.basic));
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
            Hit::KeyDown(key_down) => {
                let KeyEvent {
                    key_code,
                    modifiers,
                    ..
                } = &key_down;

                match key_code {
                    KeyCode::Escape => {
                        self.active_esc(cx, Some(key_down));
                    }
                    KeyCode::ReturnKey => {
                        if !modifiers.shift {
                            cx.hide_text_ime();
                            self.active_returned(cx, Some(key_down));
                        } else {
                            if !self.is_read_only {
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
                                self.active_changed(cx, InputChangedMetaEvent::Returned(key_down));
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if !self.is_read_only {
                            let mut start = self.selection.start().index;
                            let end = self.selection.end().index;
                            if start == end {
                                start = prev_grapheme_boundary(&self.value, start);
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
                            self.active_changed(
                                cx,
                                InputChangedMetaEvent::TextInput(TextInputEvent {
                                    input: String::new(),
                                    replace_last: false,
                                    was_paste: false,
                                }),
                            );
                        }
                    }
                    KeyCode::Delete => {
                        if !self.is_read_only {
                            let start = self.selection.start().index;
                            let mut end = self.selection.end().index;
                            if start == end {
                                end = next_grapheme_boundary(&self.value, end);
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
                            self.active_changed(cx, InputChangedMetaEvent::Delete(key_down));
                        }
                    }
                    KeyCode::KeyZ => {
                        if modifiers.is_primary() && !self.is_read_only {
                            if modifiers.shift {
                                if !self.redo(cx) {
                                    return;
                                }
                                self.draw_input.redraw(cx);
                                self.active_changed(cx, InputChangedMetaEvent::Redo(key_down));
                            } else {
                                if !self.undo(cx) {
                                    return;
                                }
                                self.draw_input.redraw(cx);
                                self.active_changed(cx, InputChangedMetaEvent::Undo(key_down));
                            }
                        }
                    }
                    _ => {
                        self.active_key_down_unhandled(cx, Some(key_down));
                    }
                }
            }
            Hit::TextInput(e) => {
                let TextInputEvent {
                    input,
                    replace_last,
                    was_paste,
                    ..
                } = &e;
                if !self.is_read_only {
                    let new_value = input.to_string();
                    let input = self.filter_input(&input, false);
                    if input.is_empty() {
                        return;
                    }
                    self.create_or_extend_edit_group(if *replace_last || *was_paste {
                        EditKind::Other
                    } else {
                        EditKind::Insert
                    });
                    if let Some(length) = self.length {
                        let current_len = self.value.graphemes(true).count();
                        let input_len = input.graphemes(true).count();
                        if current_len + input_len - if *replace_last { 1 } else { 0 } > length {
                            // 达到最大长度限制触发max_length事件
                            self.active_max_length_reached(cx, new_value);
                            return;
                        }
                    }
                    self.apply_edit(
                        cx,
                        Edit {
                            start: self.selection.start().index,
                            end: self.selection.end().index,
                            replace_with: input,
                        },
                    );
                    self.switch_state_with_animation(cx, InputState::Focus);
                    if !self.animator_in_state(cx, id!(input.focus)) {
                        self.animator_play(cx, id!(input.focus));
                    }
                    self.draw_input.redraw(cx);
                    self.active_changed(cx, InputChangedMetaEvent::TextInput(e));
                }
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
                    self.active_changed(
                        cx,
                        InputChangedMetaEvent::Cut(
                            event.response.borrow().as_ref().unwrap().to_string(),
                        ),
                    );
                }
            }

            _ => {}
        }
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().input_area;
        let live_ptr = match self.animator.live_ptr {
            Some(ptr) => ptr.file_id.0,
            None => return,
        };

        let mut registry = cx.live_registry.borrow_mut();
        let live_file = match registry.live_files.get_mut(live_ptr as usize) {
            Some(lf) => lf,
            None => return,
        };
        let nodes = &mut live_file.expanded.nodes;

        if self.lifecycle.is_created() || !init_global || self.scope_path.is_none() {
            self.lifecycle.next();
            let basic_prop = self.style.get(InputState::Basic);
            let hover_prop = self.style.get(InputState::Hover);
            let focus_prop = self.style.get(InputState::Focus);
            let empty_prop = self.style.get(InputState::Empty);
            let disabled_prop = self.style.get(InputState::Disabled);
            let (
                mut basic_index,
                mut hover_index,
                mut empty_index,
                mut focus_index,
                mut disabled_index,
            ) = (None, None, None, None, None);

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(basic).as_instance(),
                ],
            ) {
                basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(hover).as_instance(),
                ],
            ) {
                hover_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(empty).as_instance(),
                ],
            ) {
                empty_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(focus).as_instance(),
                ],
            ) {
                focus_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(input).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_input = {
                    basic_index => {
                        background_color => basic_prop.container.background_color,
                        border_color =>basic_prop.container.border_color,
                        border_radius => basic_prop.container.border_radius,
                        border_width =>(basic_prop.container.border_width as f64),
                        shadow_color => basic_prop.container.shadow_color,
                        spread_radius => (basic_prop.container.spread_radius as f64),
                        blur_radius => (basic_prop.container.blur_radius as f64),
                        shadow_offset => basic_prop.container.shadow_offset,
                        background_visible => basic_prop.container.background_visible.to_f64()
                    },
                    hover_index => {
                        background_color => hover_prop.container.background_color,
                        border_color => hover_prop.container.border_color,
                        border_radius => hover_prop.container.border_radius,
                        border_width => (hover_prop.container.border_width as f64),
                        shadow_color => hover_prop.container.shadow_color,
                        spread_radius => (hover_prop.container.spread_radius as f64),
                        blur_radius => (hover_prop.container.blur_radius as f64),
                        shadow_offset => hover_prop.container.shadow_offset,
                        background_visible => hover_prop.container.background_visible.to_f64()
                    },
                    empty_index => {
                        background_color => empty_prop.container.background_color,
                        border_color => empty_prop.container.border_color,
                        border_radius => empty_prop.container.border_radius,
                        border_width => (empty_prop.container.border_width as f64),
                        shadow_color => empty_prop.container.shadow_color,
                        spread_radius => (empty_prop.container.spread_radius as f64),
                        blur_radius => (empty_prop.container.blur_radius as f64),
                        shadow_offset => empty_prop.container.shadow_offset,
                        background_visible => empty_prop.container.background_visible.to_f64()
                    },
                    focus_index => {
                        background_color => focus_prop.container.background_color,
                        border_color => focus_prop.container.border_color,
                        border_radius => focus_prop.container.border_radius,
                        border_width => (focus_prop.container.border_width as f64),
                        shadow_color => focus_prop.container.shadow_color,
                        spread_radius => (focus_prop.container.spread_radius as f64),
                        blur_radius => (focus_prop.container.blur_radius as f64),
                        shadow_offset => focus_prop.container.shadow_offset,
                        background_visible => focus_prop.container.background_visible.to_f64()
                    },
                    disabled_index => {
                        background_color => disabled_prop.container.background_color,
                        border_color => disabled_prop.container.border_color,
                        border_radius => disabled_prop.container.border_radius,
                        border_width => (disabled_prop.container.border_width as f64),
                        shadow_color => disabled_prop.container.shadow_color,
                        spread_radius => (disabled_prop.container.spread_radius as f64),
                        blur_radius => (disabled_prop.container.blur_radius as f64),
                        shadow_offset => disabled_prop.container.shadow_offset,
                        background_visible => disabled_prop.container.background_visible.to_f64()
                    }
                }
            }

            set_animation! {
                nodes: draw_selection = {
                    basic_index => {
                        color => basic_prop.selection.color
                    },
                    hover_index => {
                        color => hover_prop.selection.color
                    },
                    empty_index => {
                        color => empty_prop.selection.color
                    },
                    focus_index => {
                        color => focus_prop.selection.color
                    },
                    disabled_index => {
                        color => disabled_prop.selection.color
                    }
                }
            }

            set_animation! {
                nodes: draw_cursor = {
                    basic_index => {
                        color => basic_prop.cursor.color
                    },
                    hover_index => {
                        color => hover_prop.cursor.color
                    },
                    empty_index => {
                        color => empty_prop.cursor.color
                    },
                    focus_index => {
                        color => focus_prop.cursor.color
                    },
                    disabled_index => {
                        color => disabled_prop.cursor.color
                    }
                }
            }

            set_animation! {
                nodes: draw_text = {
                    basic_index => {
                        color => basic_prop.text.color,
                        text_style.font_size => (basic_prop.text.font_size as f64),
                        text_style.line_spacing => (basic_prop.text.line_spacing as f64)
                    },
                    hover_index => {
                        color => hover_prop.text.color,
                        text_style.font_size => (hover_prop.text.font_size as f64),
                        text_style.line_spacing => (hover_prop.text.line_spacing as f64)
                    },
                    empty_index => {
                        color => empty_prop.text.color,
                        text_style.font_size => (empty_prop.text.font_size as f64),
                        text_style.line_spacing => (empty_prop.text.line_spacing as f64)
                    },
                    focus_index => {
                        color => focus_prop.text.color,
                        text_style.font_size => (focus_prop.text.font_size as f64),
                        text_style.line_spacing => (focus_prop.text.line_spacing as f64)
                    },
                    disabled_index => {
                        color => disabled_prop.text.color,
                        text_style.font_size => (disabled_prop.text.font_size as f64),
                        text_style.line_spacing => (disabled_prop.text.line_spacing as f64)
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                InputState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(basic).as_instance(),
                    ],
                ),
                InputState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(hover).as_instance(),
                    ],
                ),
                InputState::Empty => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(empty).as_instance(),
                    ],
                ),
                InputState::Focus => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(focus).as_instance(),
                    ],
                ),
                InputState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(input).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_input = {
                    index => {
                        background_color => style.container.background_color,
                        border_color =>style.container.border_color,
                        border_radius => style.container.border_radius,
                        border_width =>(style.container.border_width as f64),
                        shadow_color => style.container.shadow_color,
                        spread_radius => (style.container.spread_radius as f64),
                        blur_radius => (style.container.blur_radius as f64),
                        shadow_offset => style.container.shadow_offset,
                        background_visible => style.container.background_visible.to_f64()
                    }
                }
            }

            set_animation! {
                nodes: draw_selection = {
                    index => {
                        color => style.selection.color
                    }
                }
            }

            set_animation! {
                nodes: draw_cursor = {
                    index => {
                        color => style.cursor.color
                    }
                }
            }

            set_animation! {
                nodes: draw_text = {
                    index => {
                        color => style.text.color,
                        text_style.font_size => (style.text.font_size as f64),
                        text_style.line_spacing => (style.text.line_spacing as f64)
                    }
                }
            }
        }
    }

    sync!();
    switch_state!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl GInputArea {
    active_event! {
        active_hover_in: InputEvent::HoverIn |meta: FingerHoverEvent| => InputHoverIn { meta },
        active_hover_out: InputEvent::HoverOut |meta: FingerHoverEvent| => InputHoverOut { meta },
        active_focus_lost: InputEvent::FocusLost |meta: Option<KeyFocusEvent>| => InputFocusLost { meta },
        active_clicked: InputEvent::Clicked |meta: FingerUpEvent| => InputClicked { meta }
    }

    pub fn active_focus(&mut self, cx: &mut Cx, meta: InputFocusMetaEvent) {
        if self.event_open {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    InputEvent::Focus(InputFocus {
                        meta,
                        value: self.value.to_string(),
                    }),
                );
            }
        }
    }

    pub fn active_changed(&mut self, cx: &mut Cx, meta: InputChangedMetaEvent) {
        if self.event_open {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    InputEvent::Changed(InputChanged {
                        meta,
                        value: self.value.to_string(),
                    }),
                );
            }
        }
    }

    pub fn active_max_length_reached(&mut self, cx: &mut Cx, new_input: String) {
        if self.event_open {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    InputEvent::MaxLengthReached(InputMaxLengthReached {
                        new_input,
                        value: self.value.to_string(),
                    }),
                );
            }
        }
    }

    event_option! {
        hover_in: InputEvent::HoverIn => InputHoverIn,
        hover_out: InputEvent::HoverOut => InputHoverOut,
        focus_lost: InputEvent::FocusLost => InputFocusLost,
        clicked: InputEvent::Clicked => InputClicked,
        changed: InputEvent::Changed => InputChanged,
        focus: InputEvent::Focus => InputFocus,
        backspace: InputEvent::Backspace => InputKeyDown,
        esc: InputEvent::Esc => InputKeyDown,
        returned: InputEvent::Returned => InputKeyDown,
        key_down_unhandled: InputEvent::KeyDownUnhandled => InputKeyDown,
        max_length_reached: InputEvent::MaxLengthReached => InputMaxLengthReached
    }

    pub fn active_backspace(&mut self, cx: &mut Cx, meta: Option<KeyEvent>) {
        self.active_key_down(cx, meta, KeyType::Backspace);
    }

    pub fn active_returned(&mut self, cx: &mut Cx, meta: Option<KeyEvent>) {
        self.active_key_down(cx, meta, KeyType::Return);
    }

    pub fn active_esc(&mut self, cx: &mut Cx, meta: Option<KeyEvent>) {
        self.active_key_down(cx, meta, KeyType::Esc);
    }

    pub fn active_key_down_unhandled(&mut self, cx: &mut Cx, meta: Option<KeyEvent>) {
        self.active_key_down(cx, meta, KeyType::Other);
    }

    pub fn active_key_down(&mut self, cx: &mut Cx, meta: Option<KeyEvent>, key_type: KeyType) {
        if self.event_open {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    key_type.event(meta, self.value.to_string()),
                );
            }
        }
    }

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

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn set_empty_text(&mut self, cx: &mut Cx, placeholder: String) {
        self.placeholder = placeholder;
        if self.value.is_empty() {
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
        &self.value[self.selection.start().index..self.selection.end().index]
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
        let grapheme_index = self.value[..index].graphemes(true).count();
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
        self.value
            .grapheme_indices(true)
            .nth(grapheme_index)
            .map_or(self.value.len(), |(index, _)| index)
    }

    fn inner_walk(&self) -> Walk {
        let walk = self.style.get(self.state).container.walk();
        if walk.width.is_fit() {
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
            for grapheme in self.value.graphemes(true) {
                self.password_text
                    .push(if grapheme == "\n" { '\n' } else { '•' });
            }
            &self.password_text
        } else {
            &self.value
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
        let align = self.style.get(self.state).text.align;
        self.laidout_text =
            Some(
                self.draw_text
                    .layout(cx, 0.0, 0.0, wrap_width_in_lpxs, align, text),
            );
    }

    fn draw_text(&mut self, cx: &mut Cx2d) -> Rect {
        let inner_walk = self.inner_walk();
        let style = self.style.get(self.state);
        let text_rect = if self.value.is_empty() {
            self.draw_text
                .draw_walk(cx, inner_walk, style.text.align, &self.placeholder)
        } else {
            let laidout_text = self.laidout_text.as_ref().unwrap();
            self.draw_text
                .draw_walk_laidout(cx, inner_walk, style.text.align, laidout_text)
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
                index: prev_grapheme_boundary(&self.value, self.selection.cursor.index),
                prefer_next_row: true,
            },
            keep_selection,
        );
    }

    pub fn move_cursor_right(&mut self, cx: &mut Cx, keep_selection: bool) {
        self.set_cursor(
            cx,
            Cursor {
                index: next_grapheme_boundary(&self.value, self.selection.cursor.index),
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
                    index: self.value.len(),
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
        for (word_boundary_index, _) in self.value.split_word_bound_indices() {
            if word_boundary_index > index {
                return prev_word_boundary_index;
            }
            prev_word_boundary_index = word_boundary_index;
        }
        prev_word_boundary_index
    }

    fn floor_word_boundary(&self, index: usize) -> usize {
        let mut prev_word_boundary_index = self.value.len();
        for (word_boundary_index, _) in self.value.split_word_bound_indices().rev() {
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
                let before_selection = self.value[..self.selection.start().index].to_string();
                let after_selection = self.value[self.selection.end().index..].to_string();
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
        self.history.apply_edit(edit, &mut self.value);
        self.laidout_text = None;
        self.check_text_is_empty(cx);
    }

    fn undo(&mut self, cx: &mut Cx) -> bool {
        if let Some(new_selection) = self.history.undo(self.selection, &mut self.value) {
            self.laidout_text = None;
            self.selection = new_selection;
            self.check_text_is_empty(cx);
            true
        } else {
            false
        }
    }

    fn redo(&mut self, cx: &mut Cx) -> bool {
        if let Some(new_selection) = self.history.redo(self.selection, &mut self.value) {
            self.laidout_text = None;
            self.selection = new_selection;
            self.check_text_is_empty(cx);
            true
        } else {
            false
        }
    }

    fn check_text_is_empty(&mut self, cx: &mut Cx) {
        if self.value.is_empty() {
            self.animator_play(cx, id!(input.empty));
        } else {
            self.animator_play(cx, id!(input.basic));
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

impl GInputAreaRef {
    event_option_ref! {
        hover_in => InputHoverIn,
        hover_out => InputHoverOut,
        focus_lost => InputFocusLost,
        clicked => InputClicked,
        focus => InputFocus,
        changed => InputChanged,
        backspace => InputKeyDown,
        esc => InputKeyDown,
        returned => InputKeyDown,
        key_down_unhandled => InputKeyDown,
        max_length_reached => InputMaxLengthReached
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

//     pub fn placeholder(&self) -> String {
//         if let Some(inner) = self.borrow(){
//             inner.placeholder().to_string()
//         }
//         else{
//             String::new()
//         }
//     }

//     pub fn set_empty_text(&self, cx: &mut Cx, placeholder: String) {
//         if let Some(mut inner) = self.borrow_mut(){
//             inner.set_empty_text(cx, placeholder);
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
