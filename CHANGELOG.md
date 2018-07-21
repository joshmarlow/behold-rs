# Change Log

## 1.0.3

Several functions that accepted `String`s were updated to instead accept `&str` as that seems more ergonomic.
Specifically:

- `set_context` now accepts a &str instead of a String for it's context key
- `tag now` accepts a &str instead of a String for it's context key
- `when_context` now accepts a &str instead of a String for it's context key

Removed now redundant functions that accepted `&str`s.
Specifically:

- `tag_str` is removed
- `show_str` is removed
