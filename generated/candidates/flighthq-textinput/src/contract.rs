// @generated from upstream/packages/textinput/src/contract.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::{
    append_text_input, apply_text_input_restriction, blur_selectable_rich_text, blur_text_input,
    can_redo_text_input, can_undo_text_input, clear_text_input_history,
    connect_input_to_text_input, create_selectable_rich_text_manager, create_text_input_manager,
    delete_text_input_backward, delete_text_input_forward, delete_text_input_word_backward,
    delete_text_input_word_forward, disable_text_input, dispatch_selectable_rich_text_key_down,
    dispatch_selectable_rich_text_pointer_down, dispatch_selectable_rich_text_pointer_move,
    dispatch_selectable_rich_text_wheel, dispatch_text_input, dispatch_text_input_key_down,
    dispatch_text_input_pointer_down, dispatch_text_input_pointer_move, dispatch_text_input_wheel,
    enable_text_input, focus_selectable_rich_text, focus_text_input,
    get_selectable_rich_text_selection_text, get_text_input_caret_index,
    get_text_input_caret_rectangle, get_text_input_character_index_at_point,
    get_text_input_display_text, get_text_input_selection_begin_index,
    get_text_input_selection_end_index, get_text_input_selection_rectangles,
    get_text_input_selection_text, get_text_input_state, handle_text_input_keyboard,
    has_text_input, insert_text_input, move_text_input_caret, move_text_input_caret_by_word,
    move_text_input_caret_down, move_text_input_caret_to_line_end,
    move_text_input_caret_to_line_start, move_text_input_caret_up, redo_text_input,
    replace_selected_text_input, replace_text_input, scroll_text_input_caret_into_view,
    select_all_text_input, select_line_at_text_input_index, select_word_at_text_input_index,
    set_text_input_selection, undo_text_input,
};
