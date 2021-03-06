use cursive::Cursive;
use cursive::views::{TextView, Dialog, ScrollView, OnEventView};
use cursive::event::Key;

const HELP_TEXT: &str = include_str!("help_text.md");

pub fn show_help(s: &mut Cursive) {
    let text_view = TextView::new(HELP_TEXT);
    let dialog = Dialog::around(ScrollView::new(text_view))
        .dismiss_button("Ok");
    let esc_view = OnEventView::new(dialog)
        .on_event(Key::Esc, |s| {
            s.pop_layer();
        });
    s.add_layer(esc_view);
}
