//! Editing operations.
//!
//! A collection of functions intended to be associated with canonical names of
//! editing operations. These functions serve as the glue between a [`Key`] and
//! its respective action in the context of the editing experience.
//!
//! See [`BindingMap`](crate::bind::BindingMap) for further details on binding keys
//! at runtime.
use crate::editor::Align;
use crate::error::Result;
use crate::key::Key;
use crate::session::Session;
use crate::workspace::Placement;

/// A function type that implements an editing operation.
pub type OpFn = fn(&mut Session, &Key) -> Result<Action>;

/// A function type that implements a continuation of an editing operation.
pub type ContinueFn = dyn FnMut(&mut Session, &Key) -> Result<Action>;

/// An action returned by an editing operation that is to be carried out by the
/// [`Controller`].
pub enum Action {
    Nothing,
    Continue(Box<ContinueFn>),
    Alert(String),
    UndefinedKey(Key),
    Quit,
}

/// Canonical name: `meta-key`
pub fn meta_key(_: &mut Session, _: &Key) -> Result<Action> {
    let cont_fn = |_: &mut Session, key: &Key| match key {
        // FIXME: translate to meta key
        _ => Ok(Action::Nothing),
    };
    Ok(Action::Continue(Box::new(cont_fn)))
}

/// Canonical name: `insert-char`
pub fn insert_char(session: &mut Session, key: &Key) -> Result<Action> {
    match key {
        Key::Char(c) => {
            session.active_editor().insert_char(*c);
            Ok(Action::Nothing)
        }
        _ => panic!("{key:?}: expecting Key::Char"),
    }
}

/// Canonical name: `insert-line`
pub fn insert_line(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().insert_char('\n');
    Ok(Action::Nothing)
}

/// Canonical name: `delete-char-left`
pub fn delete_char_left(session: &mut Session, _: &Key) -> Result<Action> {
    // todo: should we return deleted char in result?
    let _ = session.active_editor().delete_left();
    Ok(Action::Nothing)
}

/// Canonical name: `delete-char-right`
pub fn delete_char_right(session: &mut Session, _: &Key) -> Result<Action> {
    // todo: should we return deleted char in result?
    let _ = session.active_editor().delete_right();
    Ok(Action::Nothing)
}

/// Canonical name: `move-up`
pub fn move_up(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_up();
    Ok(Action::Nothing)
}

/// Canonical name: `move-down`
pub fn move_down(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_down();
    Ok(Action::Nothing)
}

/// Canonical name: `move-left`
pub fn move_left(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_left();
    Ok(Action::Nothing)
}

/// Canonical name: `move-right`
pub fn move_right(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_right();
    Ok(Action::Nothing)
}

/// Canonical name: `move-page-up`
pub fn move_page_up(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_page_up();
    Ok(Action::Nothing)
}

/// Canonical name: `move-page-down`
pub fn move_page_down(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_page_down();
    Ok(Action::Nothing)
}

/// Canonical name: `move-top`
pub fn move_top(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_top();
    Ok(Action::Nothing)
}

/// Canonical name: `move-bottom`
pub fn move_bottom(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_bottom();
    Ok(Action::Nothing)
}

/// Canonical name: `scroll-up`
pub fn scroll_up(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().scroll_up();
    Ok(Action::Nothing)
}

/// Canonical name: `scroll-down`
pub fn scroll_down(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().scroll_down();
    Ok(Action::Nothing)
}

/// Canonical name: `move-begin-line`
pub fn move_begin_line(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_beg();
    Ok(Action::Nothing)
}

/// Canonical name: `move-end-line`
pub fn move_end_line(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().move_end();
    Ok(Action::Nothing)
}

/// Canonical name: `redraw`
pub fn redraw(session: &mut Session, _: &Key) -> Result<Action> {
    session.active_editor().draw();
    Ok(Action::Nothing)
}

/// Canonical name: `redraw-and-center`
pub fn redraw_and_center(session: &mut Session, _: &Key) -> Result<Action> {
    let mut editor = session.active_editor();
    editor.align_cursor(Align::Center);
    editor.draw();
    Ok(Action::Nothing)
}

/// Canonical name: `quit`
pub fn quit(_: &mut Session, _: &Key) -> Result<Action> {
    // FIXME: ask to save dirty buffers
    Ok(Action::Quit)
}

/// Canonical name: `window-key`
pub fn window_key(_: &mut Session, _: &Key) -> Result<Action> {
    let cont_fn = |session: &mut Session, key: &Key| {
        let action = match key {
            Key::Char('/') => session
                .add_view(Placement::Top)
                .map(|_| Action::Nothing)
                .unwrap_or(Action::Nothing),
            Key::Char('\\') => session
                .add_view(Placement::Bottom)
                .map(|_| Action::Nothing)
                .unwrap_or(Action::Nothing),
            Key::Char('[') => session
                .add_view(Placement::Above(session.active_id()))
                .map(|_| Action::Nothing)
                .unwrap_or(Action::Nothing),
            Key::Char(']') => session
                .add_view(Placement::Below(session.active_id()))
                .map(|_| Action::Nothing)
                .unwrap_or(Action::Nothing),
            Key::Char('k') => session
                .remove_view(session.active_id())
                .map(|_| Action::Nothing)
                .unwrap_or(Action::Nothing),
            Key::Char('p') => {
                session.prev_view();
                Action::Nothing
            }
            Key::Char('n') => {
                session.next_view();
                Action::Nothing
            }
            _ => Action::UndefinedKey(key.clone()),
        };
        Ok(action)
    };
    Ok(Action::Continue(Box::new(cont_fn)))
}
