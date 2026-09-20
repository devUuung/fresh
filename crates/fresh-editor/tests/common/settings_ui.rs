//! Driving the Settings dialog from a test.

use crate::common::harness::EditorTestHarness;
use crossterm::event::{KeyCode, KeyModifiers};

/// Move the Settings category selection onto `name`.
///
/// The categories are a list and `Down` is the only way along it, so the keys
/// pressed are the same however this is done. What is not the same is reading
/// the screen back and redrawing it after every one of them: the list is
/// already on screen, so the distance can be read off it once and walked in a
/// single burst, with the result checked at the end. Stepping and re-scanning
/// a row at a time measured at 587ms of a 1.23s test.
///
/// The one-at-a-time walk stays as the fallback, for when the target is not on
/// screen (the pane scrolls) or the burst does not land where the arithmetic
/// said it would.
pub fn focus_category(h: &mut EditorTestHarness, name: &str) {
    if let Some(distance) = rows_to_category(h, name) {
        for _ in 0..distance {
            h.send_key(KeyCode::Down, KeyModifiers::NONE).unwrap();
        }
        h.render().unwrap();
        if category_is_selected(h, name) {
            return;
        }
    }

    for _ in 0..40 {
        if category_is_selected(h, name) {
            return;
        }
        h.send_key(KeyCode::Down, KeyModifiers::NONE).unwrap();
        h.render().unwrap();
    }
    panic!(
        "category {:?} never became selected. Screen:\n{}",
        name,
        h.screen_to_string()
    );
}

/// Whether `name`'s row is the selected one.
pub fn category_is_selected(h: &EditorTestHarness, name: &str) -> bool {
    h.screen_to_string()
        .lines()
        .any(|line| line.contains('>') && line.contains(name))
}

/// How many `Down` presses separate the selected category from `name`, when
/// both are on screen. `None` when either is not, or when `name` sits above
/// the selection -- neither is a case this helper needs, and guessing would be
/// worse than walking.
fn rows_to_category(h: &EditorTestHarness, name: &str) -> Option<usize> {
    let screen = h.screen_to_string();
    let rows: Vec<&str> = screen.lines().collect();
    let selected = rows.iter().position(|line| line.contains('>'))?;
    let target = rows.iter().position(|line| line.contains(name))?;
    target.checked_sub(selected)
}
