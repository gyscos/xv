use cursive::Cursive;
use cursive::views::{EditView, Dialog, OnEventView, LinearLayout, TextView};
use cursive::traits::{Identifiable, Boxable};
use cursive::event::Key;
use crate::hex_view::HexView;
use crate::utilities::parse_number;
use cursive::direction::Orientation;
use cursive::direction::Orientation::Vertical;

pub fn open_set_width_dialog(s: &mut Cursive) {
    let (current_width, current_group) = get_current_width_and_group(s);
    let current_width_str = format!("{}", current_width);
    let current_group_str = format!("{}", current_group);
    
    let line_width_edit = EditView::new()
        .content(current_width_str)
        .with_id("line_width")
        .min_width(8);
    
    let group_edit = EditView::new()
        .content(current_group_str)
        .with_id("group")
        .min_width(8);
    
    let editors = LinearLayout::new(Vertical)
        .child(line_width_edit)
        .child(group_edit);
    
    let layout = LinearLayout::new(Orientation::Horizontal)
        .child(TextView::new("Line width:  \nGroup:  "))
        .child(editors);
    
    let dialog = Dialog::around(layout)
        .title("Line Width")
        .dismiss_button("Cancel")
        .button("Ok", do_set_widths);
    
    let esc_view = OnEventView::new(dialog)
        .on_event(Key::Esc, |s| {
            s.pop_layer();
        })
        .on_event(Key::Enter, do_set_widths);
    
    s.add_layer(esc_view)
}

fn get_current_width_and_group(s: &mut Cursive) -> (u64, u16) {
    s.call_on_id("hex_view", |v: &mut HexView| (v.get_line_width(), v.get_group())).unwrap()
}

fn do_set_widths(s: &mut Cursive) {
    let line_width = s.call_on_id("line_width", |v: &mut EditView| v.get_content()).unwrap();
    let group = s.call_on_id("group", |v: &mut EditView| v.get_content()).unwrap();
    
    s.pop_layer();
    
    if !line_width.is_empty() {
        match parse_number(&line_width) {
            Ok(width) => s.call_on_id("hex_view", |v: &mut HexView| {
                if width > 0 {
                    v.set_line_width(width);
                }
            }),
            _ => None,
        };
    }
    
    if !group.is_empty() {
        match parse_number(&group) {
            Ok(group) => s.call_on_id("hex_view", |v: &mut HexView| {
                if group > 0 && group < std::u16::MAX as u64 {
                    v.set_group(group as u16);
                }
            }),
            _ => None,
        };
    }
}

