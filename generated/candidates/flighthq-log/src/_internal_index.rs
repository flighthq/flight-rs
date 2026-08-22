// @generated from upstream/packages/log/src/index.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::{
    add_log_sink, begin_log_group, clear_log_channel_level, clear_log_channel_levels,
    clear_log_groups, clear_log_once_keys, clear_log_redaction_paths, clear_log_serializers,
    clear_log_sinks, clear_memory_log_sink, create_buffered_log_sink, create_child_log_context,
    create_console_capture_sink, create_console_log_sink, create_fanout_log_sink,
    create_file_log_sink, create_filter_log_sink, create_json_log_formatter, create_log_context,
    create_log_span, create_memory_log_sink, create_rate_limited_log_sink, create_sampled_log_sink,
    create_text_log_formatter, dispose_file_log_sink, dispose_log_sink, enable_log_signals,
    end_log_group, end_log_timer, enter_log_span, exit_log_span, flush_log_sink,
    get_log_channel_level, get_log_console_level, get_log_level, get_log_level_name,
    get_memory_log_sink_entries, log, log_assert, log_debug, log_debug_with, log_error,
    log_error_with, log_info, log_info_with, log_once, log_verbose, log_verbose_with, log_warn,
    log_warn_with, log_with, parse_log_level, register_log_serializer, remove_log_sink,
    serialize_log_error, set_log_channel_level, set_log_console_level, set_log_level,
    set_log_redaction_paths, set_log_sink, start_log_timer,
};
