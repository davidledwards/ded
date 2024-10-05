//! Editing operations.
//!
//! A collection of functions intended to be associated with names of editing
//! operations. These functions serve as the glue between [`Key`]s and respective
//! actions in the context of the editing experience.
//!
//! See [`Bindings`](crate::bind::Bindings) for further details on binding keys
//! at runtime.
use crate::editor::Align;
use crate::error::Result;
use crate::session::Session;
use crate::workspace::Placement;

use std::collections::HashMap;

/// A function type that implements an editing operation.
pub type OpFn = fn(&mut Session) -> Result<Action>;

/// Map of editing operations to editing functions.
pub type OpMap = HashMap<&'static str, OpFn>;

/// An action returned by an editing function that is to be carried out by the
/// [`Controller`].
pub enum Action {
    Nothing,
    Continue,
    Alert(String),
    UndefinedKey,
    Quit,
}

/// Operation: `insert-line`
pub fn insert_line(session: &mut Session) -> Result<Action> {
    session.active_editor().insert_char('\n');
    Ok(Action::Nothing)
}

/// Operation: `delete-char-left`
pub fn delete_char_left(session: &mut Session) -> Result<Action> {
    // todo: should we return deleted char in result?
    let _ = session.active_editor().delete_left();
    Ok(Action::Nothing)
}

/// Operation: `delete-char-right`
pub fn delete_char_right(session: &mut Session) -> Result<Action> {
    // todo: should we return deleted char in result?
    let _ = session.active_editor().delete_right();
    Ok(Action::Nothing)
}

/// Operation: `move-up`
pub fn move_up(session: &mut Session) -> Result<Action> {
    session.active_editor().move_up();
    Ok(Action::Nothing)
}

/// Operation: `move-down`
pub fn move_down(session: &mut Session) -> Result<Action> {
    session.active_editor().move_down();
    Ok(Action::Nothing)
}

/// Operation: `move-left`
pub fn move_left(session: &mut Session) -> Result<Action> {
    session.active_editor().move_left();
    Ok(Action::Nothing)
}

/// Operation: `move-right`
pub fn move_right(session: &mut Session) -> Result<Action> {
    session.active_editor().move_right();
    Ok(Action::Nothing)
}

/// Operation: `move-page-up`
pub fn move_page_up(session: &mut Session) -> Result<Action> {
    session.active_editor().move_page_up();
    Ok(Action::Nothing)
}

/// Operation: `move-page-down`
pub fn move_page_down(session: &mut Session) -> Result<Action> {
    session.active_editor().move_page_down();
    Ok(Action::Nothing)
}

/// Operation: `move-top`
pub fn move_top(session: &mut Session) -> Result<Action> {
    session.active_editor().move_top();
    Ok(Action::Nothing)
}

/// Operation: `move-bottom`
pub fn move_bottom(session: &mut Session) -> Result<Action> {
    session.active_editor().move_bottom();
    Ok(Action::Nothing)
}

/// Operation: `scroll-up`
pub fn scroll_up(session: &mut Session) -> Result<Action> {
    session.active_editor().scroll_up();
    Ok(Action::Nothing)
}

/// Operation: `scroll-down`
pub fn scroll_down(session: &mut Session) -> Result<Action> {
    session.active_editor().scroll_down();
    Ok(Action::Nothing)
}

/// Operation: `move-begin-line`
pub fn move_begin_line(session: &mut Session) -> Result<Action> {
    session.active_editor().move_beg();
    Ok(Action::Nothing)
}

/// Operation: `move-end-line`
pub fn move_end_line(session: &mut Session) -> Result<Action> {
    session.active_editor().move_end();
    Ok(Action::Nothing)
}

/// Operation: `redraw`
pub fn redraw(session: &mut Session) -> Result<Action> {
    session.active_editor().draw();
    Ok(Action::Nothing)
}

/// Operation: `redraw-and-center`
pub fn redraw_and_center(session: &mut Session) -> Result<Action> {
    let mut editor = session.active_editor();
    editor.align_cursor(Align::Center);
    editor.draw();
    Ok(Action::Nothing)
}

/// Operation: `quit`
pub fn quit(_: &mut Session) -> Result<Action> {
    // FIXME: ask to save dirty buffers
    Ok(Action::Quit)
}

pub fn open_window_top(session: &mut Session) -> Result<Action> {
    let action = session
        .add_view(Placement::Top)
        .map(|_| Action::Nothing)
        .unwrap_or(Action::Nothing);
    Ok(action)
}

pub fn open_window_bottom(session: &mut Session) -> Result<Action> {
    let action = session
        .add_view(Placement::Bottom)
        .map(|_| Action::Nothing)
        .unwrap_or(Action::Nothing);
    Ok(action)
}

pub fn open_window_above(session: &mut Session) -> Result<Action> {
    let action = session
        .add_view(Placement::Above(session.active_id()))
        .map(|_| Action::Nothing)
        .unwrap_or(Action::Nothing);
    Ok(action)
}

pub fn open_window_below(session: &mut Session) -> Result<Action> {
    let action = session
        .add_view(Placement::Below(session.active_id()))
        .map(|_| Action::Nothing)
        .unwrap_or(Action::Nothing);
    Ok(action)
}

pub fn close_window(session: &mut Session) -> Result<Action> {
    let action = session
        .remove_view(session.active_id())
        .map(|_| Action::Nothing)
        .unwrap_or(Action::Nothing);
    Ok(action)
}

pub fn prev_window(session: &mut Session) -> Result<Action> {
    session.prev_view();
    Ok(Action::Nothing)
}

pub fn next_window(session: &mut Session) -> Result<Action> {
    session.next_view();
    Ok(Action::Nothing)
}

/// Predefined mapping of editing operations to editing functions.
const OP_MAPPINGS: [(&'static str, OpFn); 25] = [
    ("insert-line", insert_line),
    ("delete-char-left", delete_char_left),
    ("delete-char-right", delete_char_right),
    ("move-up", move_up),
    ("move-down", move_down),
    ("move-left", move_left),
    ("move-right", move_right),
    ("move-page-up", move_page_up),
    ("move-page-down", move_page_down),
    ("move-top", move_top),
    ("move-bottom", move_bottom),
    ("scroll-up", scroll_up),
    ("scroll-down", scroll_down),
    ("move-begin-line", move_begin_line),
    ("move-end-line", move_end_line),
    ("redraw", redraw),
    ("redraw-and-center", redraw_and_center),
    ("quit", quit),
    ("open-window-top", open_window_top),
    ("open-window-bottom", open_window_bottom),
    ("open-window-above", open_window_above),
    ("open-window-below", open_window_below),
    ("close-window", close_window),
    ("prev-window", prev_window),
    ("next-window", next_window),
];

pub fn init_op_map() -> OpMap {
    let mut op_map = OpMap::new();
    for (op, op_fn) in OP_MAPPINGS {
        op_map.insert(op, op_fn);
    }
    op_map
}
