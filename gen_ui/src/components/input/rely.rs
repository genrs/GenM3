use makepad_widgets::text::selection::Selection;
use unicode_segmentation::GraphemeCursor;

pub(crate) fn prev_grapheme_boundary(text: &str, index: usize) -> usize {
    let mut cursor = GraphemeCursor::new(index, text.len(), true);
    cursor.prev_boundary(text, 0).unwrap().unwrap_or(0)
}

pub(crate) fn next_grapheme_boundary(text: &str, index: usize) -> usize {
    let mut cursor = GraphemeCursor::new(index, text.len(), true);
    cursor.next_boundary(text, 0).unwrap().unwrap_or(text.len())
}

#[derive(Clone, Debug, Default)]
pub(crate) struct History {
    current_edit_kind: Option<EditKind>,
    undo_stack: EditStack,
    redo_stack: EditStack,
}

impl History {
    pub fn force_new_edit_group(&mut self) {
        self.current_edit_kind = None;
    }

    pub fn create_or_extend_edit_group(&mut self, edit_kind: EditKind, selection: Selection) {
        if !self.current_edit_kind.map_or(false, |current_edit_kind| {
            current_edit_kind.can_merge_with(edit_kind)
        }) {
            self.undo_stack.push_edit_group(selection);
            self.current_edit_kind = Some(edit_kind);
        }
    }

    pub fn apply_edit(&mut self, edit: Edit, text: &mut String) {
        let inverted_edit = edit.invert(&text);
        edit.apply(text);
        self.undo_stack.push_edit(inverted_edit);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, selection: Selection, text: &mut String) -> Option<Selection> {
        if let Some((new_selection, edits)) = self.undo_stack.pop_edit_group() {
            self.redo_stack.push_edit_group(selection);
            for edit in &edits {
                let inverted_edit = edit.invert(text);
                edit.apply(text);
                self.redo_stack.push_edit(inverted_edit);
            }
            self.current_edit_kind = None;
            Some(new_selection)
        } else {
            None
        }
    }

    pub fn redo(&mut self, selection: Selection, text: &mut String) -> Option<Selection> {
        if let Some((new_selection, edits)) = self.redo_stack.pop_edit_group() {
            self.undo_stack.push_edit_group(selection);
            for edit in &edits {
                let inverted_edit = edit.invert(text);
                edit.apply(text);
                self.undo_stack.push_edit(inverted_edit);
            }
            self.current_edit_kind = None;
            Some(new_selection)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.current_edit_kind = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EditKind {
    Insert,
    Backspace,
    Delete,
    Other,
}

impl EditKind {
    fn can_merge_with(self, other: EditKind) -> bool {
        if self == Self::Other {
            false
        } else {
            self == other
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct EditStack {
    edit_groups: Vec<EditGroup>,
    edits: Vec<Edit>,
}

impl EditStack {
    fn push_edit_group(&mut self, selection: Selection) {
        self.edit_groups.push(EditGroup {
            selection,
            edit_start: self.edits.len(),
        });
    }

    fn push_edit(&mut self, edit: Edit) {
        self.edits.push(edit);
    }

    fn pop_edit_group(&mut self) -> Option<(Selection, Vec<Edit>)> {
        match self.edit_groups.pop() {
            Some(edit_group) => Some((
                edit_group.selection,
                self.edits.drain(edit_group.edit_start..).rev().collect(),
            )),
            None => None,
        }
    }

    fn clear(&mut self) {
        self.edit_groups.clear();
        self.edits.clear();
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct EditGroup {
    selection: Selection,
    edit_start: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Edit {
    pub start: usize,
    pub end: usize,
    pub replace_with: String,
}

impl Edit {
    fn apply(&self, text: &mut String) {
        text.replace_range(self.start..self.end, &self.replace_with);
    }

    fn invert(&self, text: &str) -> Self {
        Self {
            start: self.start,
            end: self.start + self.replace_with.len(),
            replace_with: text[self.start..self.end].to_string(),
        }
    }
}
